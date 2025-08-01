// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(missing_docs)]
#![allow(clippy::missing_panics_doc)]

//! This crate provides a builder which parses given Rust source code to search
//! for CXX-Qt or CXX macros and generate any resulting C++ code. It also builds
//! the C++ code into a binary with any cxx-qt-lib code and Qt linked.

mod cfg_evaluator;
mod utils;

mod diagnostics;
use diagnostics::{Diagnostic, GeneratedError};

pub mod dir;
use dir::INCLUDE_VERB;

mod dependencies;
use dependencies::{Dependency, Manifest};

mod interface;
pub use interface::Interface;

mod opts;
pub use opts::CxxQtBuildersOpts;
pub use opts::QObjectHeaderOpts;

mod qml_modules;
use qml_modules::OwningQmlModule;
pub use qml_modules::QmlModule;

pub use qt_build_utils::MocArguments;
use quote::ToTokens;
use semver::Version;
use std::{
    collections::HashSet,
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use cxx_qt_gen::{
    parse_qt_file, self_inlining::qualify_self_types, write_cpp, write_rust, CppFragment,
    CxxQtItem, GeneratedCppBlocks, GeneratedOpt, GeneratedRustBlocks, Parser,
};

// TODO: we need to eventually support having multiple modules defined in a single file. This
// is currently an issue because we are using the Rust file name to derive the cpp file name
// and are blindly re-writing files.
//
// As we use struct names for the QObject files, we should actually be able to support multiple
// QObject macros and at most one "raw CXX" macro per file already. For now this remains a TODO
// as to keep things simpler. We also want to able to warn users about duplicate names eventually.

struct GeneratedCppFilePaths {
    plain_cpp: PathBuf,
    qobject: Option<PathBuf>,
    qobject_header: Option<PathBuf>,
}

struct GeneratedCpp {
    cxx_qt: Option<CppFragment>,
    cxx: cxx_gen::GeneratedCode,
    file_ident: String,
}

impl GeneratedCpp {
    /// Generate QObject and cxx header/source C++ file contents
    pub fn new(
        rust_file_path: impl AsRef<Path>,
        relative_path: impl AsRef<Path>,
        include_prefix: &str,
    ) -> Result<Self, Diagnostic> {
        let to_diagnostic = |err| Diagnostic::new(rust_file_path.as_ref().to_owned(), err);

        let rust_file_path = rust_file_path.as_ref();

        let file = parse_qt_file(rust_file_path)
            .map_err(GeneratedError::from)
            .map_err(to_diagnostic)?;

        let mut cxx_qt = None;
        let mut tokens = proc_macro2::TokenStream::new();

        // Add any attributes in the file into the tokenstream
        for attr in &file.attrs {
            tokens.extend(attr.into_token_stream());
        }

        // Match upstream where they use the file name and folders as the ident
        //
        // We need the relative path here as we want the folders
        let file_ident = relative_path
            .as_ref()
            // Remove the .rs extension
            .with_extension("")
            .to_string_lossy()
            .into_owned();

        // The include path we inject needs any prefix (eg the crate name) too
        let include_ident = format!("{include_prefix}/{file_ident}");

        let mut cxx_qt_opt = GeneratedOpt::default();
        cxx_qt_opt.cfg_evaluator = Box::new(cfg_evaluator::CargoEnvCfgEvaluator);

        // Loop through the items looking for any CXX or CXX-Qt blocks
        let mut found_bridge = false;
        for item in &file.items {
            match item {
                CxxQtItem::Cxx(m) => {
                    // TODO: later we will allow for multiple CXX or CXX-Qt blocks in one file
                    assert!(!found_bridge,
                            "Unfortunately only files with either a single cxx or a single cxx_qt module are currently supported.
                            The file {} has more than one of these.",
                            rust_file_path.display());
                    found_bridge = true;

                    tokens.extend(m.into_token_stream());
                }
                CxxQtItem::CxxQt(m) => {
                    // TODO: later we will allow for multiple CXX or CXX-Qt blocks in one file
                    assert!(!found_bridge,
                            "Unfortunately only files with either a single cxx or a single cxx_qt module are currently supported.
                            The file {} has more than one of these.",
                            rust_file_path.display());
                    found_bridge = true;

                    let mut parser = Parser::from(*m.clone())
                        .map_err(GeneratedError::from)
                        .map_err(to_diagnostic)?;
                    qualify_self_types(&mut parser)
                        .map_err(GeneratedError::from)
                        .map_err(to_diagnostic)?;
                    let generated_cpp = GeneratedCppBlocks::from(&parser, &cxx_qt_opt)
                        .map_err(GeneratedError::from)
                        .map_err(to_diagnostic)?;
                    let generated_rust = GeneratedRustBlocks::from(&parser)
                        .map_err(GeneratedError::from)
                        .map_err(to_diagnostic)?;

                    // TODO: we'll have to extend the C++ data here rather than overwriting
                    // assuming we share the same file
                    cxx_qt = Some(write_cpp(&generated_cpp, &include_ident));
                    let rust_tokens = write_rust(&generated_rust, Some(&include_ident));

                    // We need to do this and can't rely on the macro, as we need to generate the
                    // CXX bridge Rust code that is then fed into the cxx_gen generation.
                    tokens.extend(rust_tokens);
                }
                CxxQtItem::Item(item) => {
                    tokens.extend(item.into_token_stream());
                }
            }
        }

        let mut opt = cxx_gen::Opt::default();
        opt.cfg_evaluator = Box::new(cfg_evaluator::CargoEnvCfgEvaluator);
        let cxx = cxx_gen::generate_header_and_cc(tokens, &opt)
            .map_err(GeneratedError::from)
            .map_err(to_diagnostic)?;

        Ok(GeneratedCpp {
            cxx_qt,
            cxx,
            file_ident,
        })
    }

    /// Write generated .cpp and .h files to specified directories. Returns the paths of all files written.
    pub fn write_to_directories(
        self,
        cpp_directory: impl AsRef<Path>,
        header_directory: impl AsRef<Path>,
    ) -> GeneratedCppFilePaths {
        let cpp_directory = cpp_directory.as_ref();
        let header_directory = header_directory.as_ref();

        let mut cpp_file_paths = GeneratedCppFilePaths {
            plain_cpp: PathBuf::new(),
            qobject: None,
            qobject_header: None,
        };
        if let Some(cxx_qt_generated) = &self.cxx_qt {
            let header_path = PathBuf::from(format!(
                "{}/{}.cxxqt.h",
                header_directory.display(),
                self.file_ident
            ));
            if let Some(directory) = header_path.parent() {
                std::fs::create_dir_all(directory)
                    .expect("Could not create directory to write cxx-qt generated files");
            }
            let mut header =
                File::create(&header_path).expect("Could not create cxx-qt header file");
            let header_generated = match cxx_qt_generated {
                CppFragment::Header(header) | CppFragment::Pair { header, source: _ } => header,
                CppFragment::Source(_) => panic!("Unexpected call for source fragment."),
            };
            header
                .write_all(header_generated.as_bytes())
                .expect("Could not write cxx-qt header file");
            cpp_file_paths.qobject_header = Some(header_path);

            let cpp_path = PathBuf::from(format!(
                "{}/{}.cxxqt.cpp",
                cpp_directory.display(),
                self.file_ident
            ));
            if let Some(directory) = cpp_path.parent() {
                std::fs::create_dir_all(directory)
                    .expect("Could not create directory to write cxx-qt generated files");
            }
            let mut cpp = File::create(&cpp_path).expect("Could not create cxx-qt source file");
            let source_generated = match cxx_qt_generated {
                CppFragment::Source(source) | CppFragment::Pair { header: _, source } => source,
                CppFragment::Header(_) => panic!("Unexpected call for header fragment."),
            };
            cpp.write_all(source_generated.as_bytes())
                .expect("Could not write cxx-qt source file");
            cpp_file_paths.qobject = Some(cpp_path);
        }

        let header_path = PathBuf::from(format!(
            "{}/{}.cxx.h",
            header_directory.display(),
            self.file_ident
        ));
        if let Some(directory) = header_path.parent() {
            std::fs::create_dir_all(directory)
                .expect("Could not create directory to write cxx-qt generated header files");
        }
        let mut header = File::create(header_path).expect("Could not create cxx header file");
        header
            .write_all(&self.cxx.header)
            .expect("Could not write cxx header file");

        let cpp_path = PathBuf::from(format!(
            "{}/{}.cxx.cpp",
            cpp_directory.display(),
            self.file_ident
        ));
        if let Some(directory) = cpp_path.parent() {
            std::fs::create_dir_all(directory)
                .expect("Could not create directory to write cxx-qt generated source files");
        }
        let mut cpp = File::create(&cpp_path).expect("Could not create cxx source file");
        cpp.write_all(&self.cxx.implementation)
            .expect("Could not write cxx source file");
        cpp_file_paths.plain_cpp = cpp_path;

        cpp_file_paths
    }
}

/// Generate C++ files from a given list of Rust files, returning the generated paths
fn generate_cxxqt_cpp_files(
    rs_source: &[impl AsRef<Path>],
    header_dir: impl AsRef<Path>,
    include_prefix: &str,
) -> Vec<GeneratedCppFilePaths> {
    let cxx_qt_dir = dir::gen();
    std::fs::create_dir_all(&cxx_qt_dir).expect("Failed to create cxx-qt-gen directory!");
    std::fs::write(cxx_qt_dir.join("include-prefix.txt"), include_prefix).expect("");

    let header_dir = header_dir.as_ref().join(include_prefix);
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut generated_file_paths: Vec<GeneratedCppFilePaths> = Vec::with_capacity(rs_source.len());
    for rs_path in rs_source {
        let path = manifest_dir.join(rs_path);
        println!("cargo::rerun-if-changed={}", path.display());

        let generated_code = match GeneratedCpp::new(&path, rs_path, include_prefix) {
            Ok(v) => v,
            Err(diagnostic) => {
                diagnostic.report();
                std::process::exit(1);
            }
        };
        generated_file_paths.push(generated_code.write_to_directories(&cxx_qt_dir, &header_dir));
    }

    generated_file_paths
}

pub(crate) fn module_name_from_uri(module_uri: &str) -> String {
    // Note: We need to make sure this matches the conversion done in CMake!
    module_uri.replace('.', "_")
}

pub(crate) fn crate_name() -> String {
    env::var("CARGO_PKG_NAME").unwrap()
}

pub(crate) fn link_name() -> Option<String> {
    env::var("CARGO_MANIFEST_LINKS").ok()
}

fn qt_modules_import() -> Option<String> {
    env::var("CXX_QT_QT_MODULES").ok()
}

fn static_lib_name() -> String {
    format!("{}-cxxqt-generated", crate_name())
}

fn crate_init_key() -> String {
    format!("crate_{}", crate_name().replace('-', "_"))
}

fn qml_module_init_key(module_uri: &str) -> String {
    format!("qml_module_{}", module_name_from_uri(module_uri))
}

fn panic_duplicate_file_and_qml_module(
    path: impl AsRef<Path>,
    uri: &str,
    version_major: usize,
    version_minor: usize,
) {
    panic!("CXX-Qt bridge Rust file {} specified in QML module {uri} (version {version_major}.{version_minor}), but also specified via CxxQtBuilder::file. Bridge files must be specified via CxxQtBuilder::file or CxxQtBuilder::qml_module, but not both.", path.as_ref().display());
}

/// Run cxx-qt's C++ code generator on Rust modules marked with the `cxx_qt::bridge` macro, compile
/// the code, and link to Qt. This is the complement of the `cxx_qt::bridge` macro, which the Rust
/// compiler uses to generate the corresponding Rust code. No dependencies besides Qt, a C++17 compiler,
/// and Rust toolchain are required.
///
/// For example, if your `cxx_qt::bridge` module is in a file called `src/lib.rs` within your crate,
/// put this in your [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html):
///
/// ```no_run
/// use cxx_qt_build::CxxQtBuilder;
///
/// CxxQtBuilder::new()
///     .file("src/lib.rs")
///     .build();
/// ```
///
/// If you have multiple major versions of Qt installed (for example, 5 and 6), you can tell
/// [CxxQtBuilder] which one to use by setting the `QT_VERSION_MAJOR` environment variable to when
/// running `cargo build`. Otherwise [CxxQtBuilder] prefers the newer version by default.
///
/// To use [CxxQtBuilder] for a library to link with a C++ application, specify a directory to output
/// cxx-qt's autogenerated headers by having the C++ build system set the `CXX_QT_EXPORT_DIR`
/// environment variable before calling `cargo build`. Then, add the same directory path to the C++
/// include paths. Also, set the `QMAKE` environment variable to the path of the `qmake` executable
/// for the Qt installation found by the C++ build system. This ensures that the C++ build system and
/// [CxxQtBuilder] link to the same installation of Qt.
///
/// Under the hood, [CxxQtBuilder] uses [cc::Build], which allows compiling aditional C++ files as well.
/// Refer to [CxxQtBuilder::cc_builder] for details.
///
/// In addition to autogenerating and building QObject C++ subclasses, manually written QObject
/// subclasses can be parsed by moc and built using [CxxQtBuilder::qobject_header].
#[derive(Default)]
pub struct CxxQtBuilder {
    rust_sources: Vec<PathBuf>,
    qobject_headers: Vec<QObjectHeaderOpts>,
    qrc_files: Vec<PathBuf>,
    init_files: Vec<qt_build_utils::Initializer>,
    qt_modules: HashSet<String>,
    qml_modules: Vec<OwningQmlModule>,
    cc_builder: cc::Build,
    include_prefix: String,
    crate_include_root: Option<String>,
    additional_include_dirs: Vec<PathBuf>,
}

impl CxxQtBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        let mut qt_modules = HashSet::new();

        // Add any Qt modules from CMake
        if let Some(modules) = qt_modules_import() {
            qt_modules.extend(
                modules
                    // Each module is split by a comma
                    .split(',')
                    // Each module could be Qt::Core or Qt6::Core etc
                    // we only want the last part
                    .map(|module| {
                        if let Some((_, end)) = module.rsplit_once("::") {
                            end
                        } else {
                            module
                        }
                    })
                    .map(str::to_owned),
            );
        } else {
            // When building with Cargo we implicitly add Qt Core
            // for CMake this must be set in CMake
            qt_modules.insert("Core".to_owned());
        }

        Self {
            rust_sources: vec![],
            qobject_headers: vec![],
            qrc_files: vec![],
            init_files: vec![],
            qt_modules,
            qml_modules: vec![],
            cc_builder: cc::Build::new(),
            include_prefix: crate_name(),
            crate_include_root: Some(String::new()),
            additional_include_dirs: vec![],
        }
    }

    /// Specify rust file paths to parse through the cxx-qt marco
    /// Relative paths are treated as relative to the path of your crate's Cargo.toml file
    pub fn file(mut self, rust_source: impl AsRef<Path>) -> Self {
        let rust_source = rust_source.as_ref().to_path_buf();
        for qml_module in &self.qml_modules {
            if qml_module.rust_files.contains(&rust_source) {
                panic_duplicate_file_and_qml_module(
                    &rust_source,
                    &qml_module.uri,
                    qml_module.version_major,
                    qml_module.version_minor,
                );
            }
        }
        println!("cargo::rerun-if-changed={}", rust_source.display());
        self.rust_sources.push(rust_source);
        self
    }

    #[doc(hidden)]
    pub fn initializer(mut self, initializer: qt_build_utils::Initializer) -> Self {
        if let Some(ref init_file) = initializer.file {
            println!("cargo::rerun-if-changed={}", init_file.display());
        }
        self.init_files.push(initializer);
        self
    }

    /// Specify the sub-directory within the crate that should act as the root include directory of
    /// the crate.
    /// All header files under this subdirectory will be includable in C++ under this crates name.
    /// This is useful for crates that export C++ headers to be included by other crates.
    ///
    /// For example, if your crate is called `my_crate` and you specify `crate_include_dir(Some("include"))`,
    /// The file: `include/my_header.h` would become available as:
    ///
    /// ```cpp
    /// #include <my_crate/my_header.h>
    /// ```
    ///
    /// Specify `None` to disable automatic inclusion of your crate as a header directory.
    ///
    /// The default is `Some("")` which means that the entire crate directory is used as the include directory.
    pub fn crate_include_root(mut self, include_dir: Option<String>) -> Self {
        self.crate_include_root = include_dir;
        self
    }

    /// Specify a directory to include additional C++ headers from.
    ///
    /// This directory will be namespaced by the crate name!
    /// So if you call `include_dir("include/")` a header `include/my_header.h` will be available as:
    /// ```cpp
    /// #include <crate_name/my_header.h>
    /// ```
    ///
    /// Note that if you are trying to specify an include directory that is inside your own crate,
    /// prefer using [Self::crate_include_root], which expects a path relative to the crate
    /// directory.
    ///
    /// Also note that unlike the [Self::crate_include_root] method, this does not emit rerun-if-changed
    /// directives for the directory!
    /// If you need to rerun the build script when files in this directory change, you must emit
    /// appropriate rerun-if-changed directives yourself.
    pub fn include_dir(mut self, dir: impl AsRef<Path>) -> Self {
        let dir = dir.as_ref().to_owned();
        self.additional_include_dirs.push(dir);
        self
    }

    /// Include files listed in a .qrc file into the binary
    /// with [Qt's resource system](https://doc.qt.io/qt-6/resources.html).
    /// ```no_run
    /// # use cxx_qt_build::CxxQtBuilder;
    /// CxxQtBuilder::new()
    ///     .file("src/cxxqt_module.rs")
    ///     .qrc("src/my_resources.qrc")
    ///     .build();
    /// ```
    ///
    /// ⚠️  In CMake projects, the .qrc file is typically added to the `SOURCES` of the target.
    /// Prefer this to adding the qrc file through cxx-qt-build.
    /// When using CMake, the qrc file will **not** be built by cxx-qt-build!
    pub fn qrc(mut self, qrc_file: impl AsRef<Path>) -> Self {
        let qrc_file = qrc_file.as_ref();
        self.qrc_files.push(qrc_file.to_path_buf());
        println!("cargo::rerun-if-changed={}", qrc_file.display());
        self
    }

    /// Link additional [Qt modules](https://doc.qt.io/qt-6/qtmodules.html).
    /// Specify their names without the `Qt` prefix, for example `"Widgets"`.
    /// The `Core` module and any modules from dependencies are linked automatically; there is no need to specify them.
    ///
    /// Note that any qt_module you specify here will be enabled for all downstream
    /// dependencies as well if this crate is exported.
    /// It is therefore best practice to specify features on your crate that allow downstream users
    /// to disable any qt modules that are optional.
    pub fn qt_module(mut self, module: &str) -> Self {
        // Ensure that CMake and Cargo build.rs are not out of sync
        assert!(qt_modules_import().is_none() || self.qt_modules.contains(module), "Qt module mismatch between cxx-qt-build and CMake!\n\
                    Qt module '{module}' was not specified in CMake!\n\
                    When building with CMake, all Qt modules must be specified with the QT_MODULES argument in cxx_qt_import_crate");

        self.qt_modules.insert(module.to_owned());
        self
    }

    /// Instead of generating files under the crate name, generate files under the given prefix.
    pub fn include_prefix(mut self, prefix: &str) -> Self {
        prefix.clone_into(&mut self.include_prefix);
        self
    }

    /// Register a QML module at build time. The `rust_files` of the [QmlModule] struct
    /// should contain `#[cxx_qt::bridge]` modules with QObject types annotated with `#[qml_element]`.
    ///
    /// The QmlModule struct's `qml_files` are registered with the [Qt Resource System](https://doc.qt.io/qt-6/resources.html) in
    /// the [default QML import path](https://doc.qt.io/qt-6/qtqml-syntax-imports.html#qml-import-path) `qrc:/qt/qml/uri/of/module/`.
    /// Additional resources such as images can be added to the Qt resources for the QML module by specifying
    /// the `qrc_files` field.
    ///
    /// When using Qt 6, this will [run qmlcachegen](https://doc.qt.io/qt-6/qtqml-qtquick-compiler-tech.html)
    /// to compile the specified `.qml` files ahead-of-time.
    ///
    /// ```no_run
    /// use cxx_qt_build::{CxxQtBuilder, QmlModule};
    ///
    /// CxxQtBuilder::new()
    ///     .qml_module(QmlModule {
    ///         uri: "com.kdab.cxx_qt.demo",
    ///         rust_files: &["src/cxxqt_object.rs"],
    ///         qml_files: &["qml/main.qml"],
    ///         ..Default::default()
    ///     })
    ///     .build();
    /// ```
    pub fn qml_module<A: AsRef<Path>, B: AsRef<Path>>(
        mut self,
        qml_module: QmlModule<A, B>,
    ) -> CxxQtBuilder {
        let qml_module = OwningQmlModule::from(qml_module);
        for path in &qml_module.rust_files {
            if self.rust_sources.contains(path) {
                panic_duplicate_file_and_qml_module(
                    path,
                    &qml_module.uri,
                    qml_module.version_major,
                    qml_module.version_minor,
                );
            }
        }
        self.qml_modules.push(qml_module);
        self
    }

    /// Specify a C++ header containing a Q_OBJECT macro to run [moc](https://doc.qt.io/qt-6/moc.html) on.
    /// This allows building QObject C++ subclasses besides the ones autogenerated by cxx-qt.
    pub fn qobject_header(mut self, opts: impl Into<QObjectHeaderOpts>) -> Self {
        let opts = opts.into();
        println!("cargo::rerun-if-changed={}", opts.path.display());
        self.qobject_headers.push(opts);
        self
    }

    /// Use a closure to run additional customization on [CxxQtBuilder]'s internal [cc::Build]
    /// before calling [CxxQtBuilder::build]. This allows to add extra include paths, compiler flags,
    /// or anything else available via [cc::Build]'s API. For example, to add an include path for
    /// manually written C++ headers located in a directory called `include` within your crate:
    ///
    /// ```no_run
    /// # use cxx_qt_build::CxxQtBuilder;
    ///
    /// CxxQtBuilder::new()
    ///     .file("src/lib.rs")
    ///     .cc_builder(|cc| {
    ///         cc.include("include");
    ///     })
    ///     .build();
    /// ```
    pub fn cc_builder(mut self, mut callback: impl FnMut(&mut cc::Build)) -> Self {
        callback(&mut self.cc_builder);
        self
    }

    fn define_cfg_variable(key: &str, value: Option<&str>) {
        if let Some(value) = value {
            println!("cargo::rustc-cfg={key}=\"{value}\"");
        } else {
            println!("cargo::rustc-cfg={key}");
        }
        let variable_cargo = format!("CARGO_CFG_{key}");
        env::set_var(variable_cargo, value.unwrap_or("true"));
    }

    fn define_cfg_check_variable(key: &str, values: Option<Vec<&str>>) {
        if let Some(values) = values {
            let values = values
                .iter()
                // Escape and add quotes
                .map(|value| format!("\"{}\"", value.escape_default()))
                .collect::<Vec<_>>()
                .join(", ");
            println!("cargo::rustc-check-cfg=cfg({key}, values({values}))");
        } else {
            println!("cargo::rustc-check-cfg=cfg({key})");
        }
    }

    fn define_qt_version_cfg_variables(version: &Version) {
        // Allow for Qt 5 or Qt 6 as valid values
        CxxQtBuilder::define_cfg_check_variable("cxxqt_qt_version_major", Some(vec!["5", "6"]));
        // Find the Qt version and tell the Rust compiler
        // this allows us to have conditional Rust code
        CxxQtBuilder::define_cfg_variable(
            "cxxqt_qt_version_major",
            Some(version.major.to_string().as_str()),
        );

        // Seed all values from Qt 5.0 through to Qt 7.99
        for major in 5..=7 {
            CxxQtBuilder::define_cfg_check_variable(
                &format!("cxxqt_qt_version_at_least_{major}"),
                None,
            );

            for minor in 0..=99 {
                CxxQtBuilder::define_cfg_check_variable(
                    &format!("cxxqt_qt_version_at_least_{major}_{minor}"),
                    None,
                );
            }
        }

        for minor in 0..=version.minor {
            let qt_version_at_least =
                format!("cxxqt_qt_version_at_least_{}_{}", version.major, minor);
            CxxQtBuilder::define_cfg_variable(&qt_version_at_least, None);
        }

        // We don't support Qt < 5
        for major in 5..=version.major {
            let at_least_qt_major_version = format!("cxxqt_qt_version_at_least_{major}");
            CxxQtBuilder::define_cfg_variable(&at_least_qt_major_version, None);
        }
    }

    fn write_common_headers() {
        let header_root = dir::header_root();
        // Write cxx headers
        std::fs::create_dir_all(header_root.join("rust"))
            .expect("Could not create cxx header directory");
        let h_path = header_root.join("rust").join("cxx.h");
        // Wrap the File in a block scope so the file is closed before the compiler is run.
        // Otherwise MSVC fails to open cxx.h because the process for this build script already has it open.
        {
            std::fs::write(h_path, cxx_gen::HEADER).expect("Failed to write cxx.h");
        }
    }

    // A dependency can specify which of its own include paths it wants to export.
    // Set up each of these exported include paths as symlinks in our own include directory,
    // or deep copy the files if the platform does not support symlinks.
    fn include_dependency(dependency: &Dependency) {
        let header_root = dir::header_root();
        let dependency_root = dependency.path.join("include");
        for include_prefix in &dependency.manifest.exported_include_prefixes {
            // setup include directory
            let source = dependency_root.join(include_prefix);
            let dest = header_root.join(include_prefix);

            match dir::symlink_or_copy_directory(source, dest) {
                Ok(true) => (),
                Ok(false) => {
                    panic!(
                        "Conflicting include_prefixes for {include_prefix}!\nDependency {dep_name} conflicts with existing include path",
                        dep_name = dependency.manifest.name,
                    );
                }
                Err(e) => {
                    panic!("Could not {INCLUDE_VERB} for include_prefix {include_prefix}: {e:?}");
                }
            }
        }
    }

    fn setup_cc_builder(builder: &mut cc::Build, include_paths: &[impl AsRef<Path>]) {
        // Note, ensure our settings stay in sync across cxx-qt and cxx-qt-lib
        builder.cpp(true);
        builder.std("c++17");
        // MSVC
        builder.flag_if_supported("/Zc:__cplusplus");
        builder.flag_if_supported("/permissive-");
        builder.flag_if_supported("/bigobj");
        // MinGW requires big-obj otherwise debug builds fail
        builder.flag_if_supported("-Wa,-mbig-obj");

        for include_path in include_paths {
            builder.include(include_path);
        }
    }

    fn moc_qobject_headers(&mut self, qtbuild: &mut qt_build_utils::QtBuild) {
        for QObjectHeaderOpts {
            path,
            moc_arguments,
        } in &self.qobject_headers
        {
            let moc_products = qtbuild.moc().compile(path, moc_arguments.clone());
            // Include the moc folder
            if let Some(dir) = moc_products.cpp.parent() {
                self.cc_builder.include(dir);
            }
            self.cc_builder.file(moc_products.cpp);
        }
    }

    fn generate_cpp_files_from_cxxqt_bridges(
        &mut self,
        header_dir: impl AsRef<Path>,
        include_prefix: &str,
    ) {
        for files in generate_cxxqt_cpp_files(&self.rust_sources, &header_dir, include_prefix) {
            self.cc_builder.file(files.plain_cpp);
            if let (Some(qobject), Some(qobject_header)) = (files.qobject, files.qobject_header) {
                self.cc_builder.file(&qobject);
                self.qobject_headers.push(qobject_header.into());
            }
        }
    }

    fn export_object_file(
        mut obj_builder: cc::Build,
        file_path: impl AsRef<Path>,
        export_path: &Path,
    ) {
        obj_builder.file(file_path);

        // We only expect a single file, so destructure the vec.
        // If there's 0 or > 1 file, we panic in the `else` branch, because then the builder is
        // probably not correctly configured.
        let obj_files = obj_builder.compile_intermediates();
        if let [obj_file] = &obj_files[..] {
            if let Some(directory) = export_path.parent() {
                std::fs::create_dir_all(directory).unwrap_or_else(|_| {
                    panic!(
                        "Could not create directory for exporting object file: {}",
                        export_path.display()
                    )
                });
            }
            std::fs::copy(obj_file, &export_path).unwrap_or_else(|_| {
                panic!("Failed to export object file to {}!", export_path.display())
            });
        } else {
            panic!(
            "CXX-Qt internal error: Expected only one object file for export out of cc::Build! Got {}",
            obj_files.len()
        );
        }
    }

    fn build_qml_modules(
        &mut self,
        qtbuild: &mut qt_build_utils::QtBuild,
        generated_header_dir: impl AsRef<Path>,
        header_prefix: &str,
    ) -> Vec<qt_build_utils::Initializer> {
        let mut initializer_functions = Vec::new();
        // Extract qml_modules out of self so we don't have to hold onto `self` for the duration of
        // the loop.
        let qml_modules: Vec<_> = self.qml_modules.drain(..).collect();
        for qml_module in qml_modules {
            dir::clean(dir::module_target(&qml_module.uri))
                .expect("Failed to clean qml module export directory!");

            let mut qml_metatypes_json = Vec::new();

            // Check that all rust files are within the same directory
            //
            // Note we need to do this as moc generates an inputFile which only
            // includes the file name, qmltyperegistrar then uses this for the
            // include path (and doesn't consider any prefix).
            //
            // This can also be observed when using qt_add_qml_module, if a class
            // has a QML_ELEMENT the file must be in the same directory as the
            // CMakeLists and cannot be a relative path to a sub directory.
            let dirs = qml_module
                .rust_files
                .iter()
                .map(|file| {
                    if let Some(parent) = file.parent() {
                        parent.to_string_lossy().into_owned()
                    } else {
                        // Fallback to an empty string if there is no parent path
                        String::new()
                    }
                })
                .collect::<HashSet<String>>();
            assert!(
                dirs.len() <= 1,
                "Only one directory is supported per QmlModule for rust_files.\n\
                    This is due to Qt bug https://bugreports.qt.io/browse/QTBUG-93443\n\
                    Found directories: {dirs:?}"
            );

            // TODO: for now we use the global CxxQtBuilder cc_builder
            // this means that any includes/files etc on these are in this builder
            // but we cannot have separate builds until we can configure includes,
            // qt modules, files, cc_builder options etc in the QmlModule itself
            let cc_builder = &mut self.cc_builder;
            qtbuild.cargo_link_libraries(cc_builder);

            let mut moc_include_paths = HashSet::new();
            for files in generate_cxxqt_cpp_files(
                &qml_module.rust_files,
                &generated_header_dir,
                header_prefix,
            ) {
                cc_builder.file(files.plain_cpp);
                if let (Some(qobject), Some(qobject_header)) = (files.qobject, files.qobject_header)
                {
                    // Ensure that the generated QObject header is in the include path
                    // so that qmltyperegistar can include them later
                    if let Some(dir) = qobject_header.parent() {
                        moc_include_paths.insert(dir.to_path_buf());
                    }

                    cc_builder.file(&qobject);
                    let moc_products = qtbuild.moc().compile(
                        qobject_header,
                        MocArguments::default().uri(qml_module.uri.clone()),
                    );
                    // Include the moc folder
                    if let Some(dir) = moc_products.cpp.parent() {
                        moc_include_paths.insert(dir.to_path_buf());
                    }
                    cc_builder.file(moc_products.cpp);
                    qml_metatypes_json.push(moc_products.metatypes_json);
                }
            }

            let qml_module_registration_files = qtbuild.register_qml_module(
                &qml_metatypes_json,
                &qml_module.uri,
                qml_module.version_major,
                qml_module.version_minor,
                // TODO: This will be passed to the `optional plugin ...` part of the qmldir
                // We don't load any shared libraries, so the name shouldn't matter
                // But make sure it still works
                &module_name_from_uri(&qml_module.uri),
                &qml_module.qml_files,
                &qml_module.qrc_files,
            );
            if let Some(qmltyperegistrar) = qml_module_registration_files.qmltyperegistrar {
                cc_builder.file(qmltyperegistrar);
            }
            cc_builder
                .file(qml_module_registration_files.plugin)
                // In comparison to the other RCC files, we don't need to link this with whole-archive or
                // anything like that.
                // The plugin_init file already takes care of loading the resources associated with this
                // RCC file.
                .file(qml_module_registration_files.rcc);

            // Add any include paths the qml module registration needs
            // this is most likely the moc folder for the plugin
            if let Some(include_path) = qml_module_registration_files.include_path {
                moc_include_paths.insert(include_path);
            }

            // Ensure that all include paths from moc folders that are required
            for include_path in &moc_include_paths {
                cc_builder.include(include_path);
            }

            for qmlcachegen_file in qml_module_registration_files.qmlcachegen {
                cc_builder.file(qmlcachegen_file);
            }
            // This is required, as described here: plugin_builder
            cc_builder.define("QT_STATICPLUGIN", None);

            // If any of the files inside the qml module change, then trigger a rerun
            for path in qml_module.qml_files.iter().chain(
                qml_module
                    .rust_files
                    .iter()
                    .chain(qml_module.qrc_files.iter()),
            ) {
                println!("cargo::rerun-if-changed={}", path.display());
            }

            let module_init_key = qml_module_init_key(&qml_module.uri);
            let private_initializers = [qml_module_registration_files.plugin_init];
            let public_initializer =
                Self::generate_public_initializer(&private_initializers, &module_init_key);
            self.build_initializers(
                &private_initializers,
                &public_initializer,
                dir::module_export(&qml_module.uri).map(|dir| dir.join("plugin_init.o")),
                &module_init_key,
            );

            initializer_functions.push(public_initializer.strip_file());
        }
        initializer_functions
    }

    /// Generate the public initializer.
    /// It will call all the private initializers.
    ///
    /// Downstream crates can therefore just call the initializer once to initialize this crate.
    fn generate_public_initializer(
        private_initializers: &[qt_build_utils::Initializer],
        key: &str,
    ) -> qt_build_utils::Initializer {
        let (declarations, calls): (Vec<_>, Vec<_>) = private_initializers
            .iter()
            .map(|initializer| {
                (
                    // declaration
                    initializer.init_declaration.clone().unwrap_or_default(),
                    // call
                    initializer.init_call.clone().unwrap_or_default(),
                )
            })
            .unzip();

        let init_fun = format!("cxx_qt_init_{key}");
        // For the init_function, we need to use an internal function that is not
        // `extern "C"` as Q_INIT_RESOURCES needs name mangling, which doesn't happen if it's
        // called within an `extern "C"` function.
        // So add a static do_init function that we then call from the actual initializer function.
        let init_function = format!(
            r#"
#include <mutex>

{declarations}

static bool do_init() {{
    static std::once_flag flag;
    std::call_once(flag, []() {{
        {calls}
    }});
    return true;
}}

extern "C" bool {init_fun}() {{
    return do_init();
}}
            "#,
            declarations = declarations.join("\n"),
            calls = calls.join("\n"),
        );
        let init_function_path = dir::initializers(key).join("public-initializer.cpp");
        std::fs::write(&init_function_path, init_function)
            .expect("Failed to write public initializer file!");

        qt_build_utils::Initializer {
            file: Some(init_function_path),
            ..qt_build_utils::Initializer::default_signature(&init_fun)
        }
    }

    fn build_initializers<'a>(
        &mut self,
        private_initializers: impl IntoIterator<Item = &'a qt_build_utils::Initializer>,
        public_initializer: &qt_build_utils::Initializer,
        export_path: Option<PathBuf>,
        key: &str,
    ) {
        // Build the initializers themselves into the main library.
        self.cc_builder
            .file(
                public_initializer
                    .file
                    .as_ref()
                    .expect("Public initializer must have a file!"),
            )
            .files(
                private_initializers
                    .into_iter()
                    .filter_map(|initializer| initializer.file.as_ref()),
            );

        // Build the initializer call into a separate library to be linked with whole-archive.
        // We can just use a plain cc::Build for this, as this doesn't use any non-standard
        // features.
        let mut init_call_builder = cc::Build::new();
        let includes: &[&str] = &[]; // <-- Needed for type annotations
        Self::setup_cc_builder(&mut init_call_builder, includes);

        let init_call = format!(
            "{declaration}\nstatic const bool do_init_{key} = {init_call}",
            declaration = public_initializer
                .init_declaration
                .clone()
                .unwrap_or_default(),
            init_call = public_initializer
                .init_call
                .clone()
                .expect("Public initializer must be callable!"),
        );

        let init_file = dir::initializers(key).join("call-initializers.cpp");
        std::fs::write(&init_file, init_call).expect("Could not write initializers call file!");

        if let Some(export_path) = export_path {
            Self::export_object_file(init_call_builder, init_file, &export_path);
        } else {
            // Link the call-init-lib with +whole-archive to ensure that the static initializers are not discarded.
            // We previously used object files that we linked directly into the final binary, but this caused
            // issues, as the static initializers could sometimes not link to the initializer functions.
            // This is simpler and ends up linking correctly.
            //
            // The trick is that we only link the initializer call with +whole-archive, and not the entire
            // Rust static library, as the initializer is rather simple and shouldn't lead to issues with
            // duplicate symbols.
            // Note that for CMake builds we still need to export an object file to link to.
            init_call_builder
                .file(init_file)
                .link_lib_modifier("+whole-archive")
                .compile(&format!("cxx-qt-call-init-{key}"));
        }
    }

    fn generate_cpp_from_qrc_files(
        &mut self,
        qtbuild: &mut qt_build_utils::QtBuild,
    ) -> Vec<qt_build_utils::Initializer> {
        self.qrc_files
            .iter()
            .map(|qrc_file| {
                // Also ensure that each of the files in the qrc can cause a change
                for qrc_inner_file in qtbuild.rcc().list(qrc_file) {
                    println!("cargo::rerun-if-changed={}", qrc_inner_file.display());
                }
                // We need to link this using an object file or +whole-achive, the static initializer of
                // the qrc file isn't lost.
                qtbuild.rcc().compile(qrc_file)
            })
            .collect()
    }

    fn qt_modules(&self, dependencies: &[Dependency]) -> HashSet<String> {
        let mut qt_modules = self.qt_modules.clone();
        for dependency in dependencies {
            qt_modules.extend(dependency.manifest.qt_modules.iter().cloned());
        }
        qt_modules
    }

    /// Generate and compile cxx-qt C++ code, as well as compile any additional files from
    /// [CxxQtBuilder::qobject_header] and [CxxQtBuilder::cc_builder].
    pub fn build(mut self) -> Interface {
        const MAX_INCLUDE_DEPTH: usize = 6;

        dir::clean(dir::crate_target()).expect("Failed to clean crate export directory!");

        // We will do these two steps first, as setting up the dependencies can modify flags we
        // need further down the line
        // Also write the common headers first, to make sure they don't conflict with any
        // dependencies
        Self::write_common_headers();
        let dependencies = Dependency::find_all();
        for dependency in &dependencies {
            Self::include_dependency(dependency);
        }
        let qt_modules = self.qt_modules(&dependencies);

        // Ensure that the linker is setup correctly for Cargo builds
        qt_build_utils::setup_linker();

        let header_root = dir::header_root();

        let mut qtbuild = qt_build_utils::QtBuild::new(qt_modules.iter().cloned().collect())
            .expect("Could not find Qt installation");
        qtbuild.cargo_link_libraries(&mut self.cc_builder);
        Self::define_qt_version_cfg_variables(&qtbuild.version());

        // Ensure that Qt modules and apple framework are linked and searched correctly
        let mut include_paths = qtbuild.include_paths();
        include_paths.push(header_root.clone());
        // TODO: Some of the code generated by qmltyperegistrar doesn't add the include_prefix to
        // the #include directives.
        // We therefore need to push the full header directory including the prefix as an include path.
        // This is not ideal and should be removed in future as it allows user code direct access
        // to the generated files without any namespacing.
        include_paths.push(header_root.join(&self.include_prefix));

        let crate_header_dir = self.crate_include_root.as_ref().map(|subdir| {
            dir::manifest()
                .expect("Could not find crate directory!")
                .join(subdir)
        });
        if let Some(crate_header_dir) = crate_header_dir {
            utils::best_effort_copy_headers(
                crate_header_dir.as_path(),
                header_root.join(crate_name()).as_path(),
                MAX_INCLUDE_DEPTH,
                // Emit rerun-if-changed for this directory as it is be part of the crate root it
                // should not contain any generated files which may cause unwanted reruns.
                true,
            );
        }
        for include_dir in &self.additional_include_dirs {
            utils::best_effort_copy_headers(
                include_dir,
                header_root.join(crate_name()).as_path(),
                MAX_INCLUDE_DEPTH,
                // Do not emit rerun-if-changed for this directory as it may not be part of the crate root
                // and we do not know if these headers are generated or not.
                // If they are generated by the build script, they should not be marked with
                // rerun-if-changed, because they would cause unwanted reruns.
                false,
            );
        }

        Self::setup_cc_builder(&mut self.cc_builder, &include_paths);

        // Generate files
        self.generate_cpp_files_from_cxxqt_bridges(&header_root, &self.include_prefix.clone());

        self.moc_qobject_headers(&mut qtbuild);

        // Bridges for QML modules are handled separately because
        // the metatypes_json generated by moc needs to be passed to qmltyperegistrar
        let module_initializers =
            self.build_qml_modules(&mut qtbuild, &header_root, &self.include_prefix.clone());

        let qrc_files = self.generate_cpp_from_qrc_files(&mut qtbuild);

        let dependency_initializers = dependencies::initializers(&dependencies);
        let private_initializers = dependency_initializers
            .into_iter()
            .chain(qrc_files)
            .chain(module_initializers)
            .chain(self.init_files.iter().cloned())
            .collect::<Vec<_>>();

        let public_initializer =
            Self::generate_public_initializer(&private_initializers, &crate_init_key());
        let export_path = if dir::is_exporting_crate() {
            Some(dir::crate_target().join("initializers.o"))
        } else {
            None
        };
        self.build_initializers(
            &private_initializers,
            &public_initializer,
            export_path,
            &crate_init_key(),
        );

        // Only compile if we have added files to the builder
        // otherwise we end up with no static library but ask cargo to link to it which causes an error
        if self.cc_builder.get_files().count() > 0 {
            self.cc_builder.compile(&static_lib_name());
        }

        Interface {
            manifest: Manifest {
                name: crate_name(),
                link_name: link_name().unwrap_or_default(),
                initializers: vec![public_initializer.strip_file()],
                qt_modules: qt_modules.into_iter().collect(),
                exported_include_prefixes: vec![],
            },
            dependencies,
            ..Default::default()
        }
    }
}
