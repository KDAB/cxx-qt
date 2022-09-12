// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use convert_case::{Case, Casing};
use quote::ToTokens;
use std::{
    collections::HashSet,
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use cxx_qt_gen::{
    extract_qobject, generate_qobject_rs, parse_qt_file, write_cpp, CppFragmentPair, CxxQtItem,
    GeneratedCppBlocks, Parser,
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
    cxx_qt: Option<CppFragmentPair>,
    cxx: cxx_gen::GeneratedCode,
    file_ident: String,
}

impl GeneratedCpp {
    /// Generate QObject and cxx header/source C++ file contents
    pub fn new(rust_file_path: impl AsRef<Path>) -> Self {
        let rust_file_path = rust_file_path.as_ref();
        let file = parse_qt_file(rust_file_path).unwrap();

        let mut cxx_qt = None;
        // TODO: later change how the resultant filename is chosen, can we match the input file like
        // CXX does?
        //
        // For CXX-Qt generating one header per QObject likely makes sense, but what happens with CXX data?
        // for now this uses the module ident
        let mut file_ident: String = "".to_owned();
        let mut tokens = proc_macro2::TokenStream::new();

        // Add any attributes in the file into the tokenstream
        for attr in &file.attrs {
            tokens.extend(attr.into_token_stream());
        }

        // Loop through the items looking for any CXX or CXX-Qt blocks
        for item in &file.items {
            match item {
                CxxQtItem::Cxx(m) => {
                    // TODO: later we will allow for multiple CXX or CXX-Qt blocks in one file
                    if !file_ident.is_empty() {
                        panic!(
                            "Unfortunately only files with either a single cxx or a single cxx_qt module are currently supported.
                            The file {} has more than one of these.",
                            rust_file_path.display());
                    }

                    file_ident = m.ident.to_string().to_case(Case::Snake);
                    tokens.extend(m.into_token_stream());
                }
                CxxQtItem::CxxQt(m) => {
                    // TODO: later we will allow for multiple CXX or CXX-Qt blocks in one file
                    if !file_ident.is_empty() {
                        panic!(
                            "Unfortunately only files with either a single cxx or a single cxx_qt module are currently supported.
                            The file {} has more than one of these.",
                            rust_file_path.display());
                    }

                    let parser = Parser::from(m.clone()).unwrap();
                    let generated = GeneratedCppBlocks::from(&parser).unwrap();
                    // TODO: we'll have to extend the C++ data here rather than overwriting
                    // assuming we share the same file
                    cxx_qt = Some(write_cpp(&generated));

                    // TODO: later we will likely have cxx_qt_gen::generate_header_and_cpp
                    // which will take a CxxQtItemMod and respond with a C++ header and source
                    let qobject = extract_qobject(m).unwrap();
                    // Use the qobject ident as the output file name?
                    file_ident = qobject.ident.to_string().to_case(Case::Snake);

                    // TODO: later we will likely have cxx_qt_gen::generate_rust
                    // which will take a CxxQtItemMod and respond with the Rust code
                    //
                    // We need to do this and can't rely on the macro, as we need to generate the
                    // CXX bridge Rust code that is then fed into the cxx_gen generation.
                    tokens.extend(generate_qobject_rs(&qobject).unwrap());
                }
                CxxQtItem::Item(item) => {
                    tokens.extend(item.into_token_stream());
                }
            }
        }

        let opt = cxx_gen::Opt::default();
        let cxx = cxx_gen::generate_header_and_cc(tokens, &opt)
            .expect("Could not generate C++ from Rust file");

        GeneratedCpp {
            cxx_qt,
            cxx,
            file_ident,
        }
    }

    /// Write generated .cpp and .h files to specified directories. Returns the paths of all files written.
    pub fn write_to_directories(
        &self,
        cpp_directory: impl AsRef<Path>,
        header_directory: impl AsRef<Path>,
    ) -> GeneratedCppFilePaths {
        let cpp_directory = cpp_directory.as_ref();
        let header_directory = header_directory.as_ref();
        for directory in [cpp_directory, header_directory] {
            std::fs::create_dir_all(&directory)
                .expect("Could not create directory to write cxx-qt generated files");
        }

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
            let mut header =
                File::create(&header_path).expect("Could not create cxx-qt header file");
            header
                .write_all(cxx_qt_generated.header.as_bytes())
                .expect("Could not write cxx-qt header file");
            cpp_file_paths.qobject_header = Some(header_path);

            let cpp_path = PathBuf::from(format!(
                "{}/{}.cxxqt.cpp",
                cpp_directory.display(),
                self.file_ident
            ));
            let mut cpp = File::create(&cpp_path).expect("Could not create cxx-qt source file");
            cpp.write_all(cxx_qt_generated.source.as_bytes())
                .expect("Could not write cxx-qt source file");
            cpp_file_paths.qobject = Some(cpp_path);
        }

        let header_path = PathBuf::from(format!(
            "{}/{}.cxx.h",
            header_directory.display(),
            self.file_ident
        ));
        let mut header = File::create(&header_path).expect("Could not create cxx header file");
        header
            .write_all(&self.cxx.header)
            .expect("Could not write cxx header file");

        let cpp_path = PathBuf::from(format!(
            "{}/{}.cxx.cpp",
            cpp_directory.display(),
            self.file_ident
        ));
        let mut cpp = File::create(&cpp_path).expect("Could not create cxx source file");
        cpp.write_all(&self.cxx.implementation)
            .expect("Could not write cxx source file");
        cpp_file_paths.plain_cpp = cpp_path;

        cpp_file_paths
    }
}

/// Generate C++ files from a given list of Rust files, returning the generated paths
fn generate_cxxqt_cpp_files(
    rs_source: &[PathBuf],
    header_dir: impl AsRef<Path>,
) -> Vec<GeneratedCppFilePaths> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cpp_directory = format!("{}/cxx-qt-gen/src", env::var("OUT_DIR").unwrap());

    let mut generated_file_paths: Vec<GeneratedCppFilePaths> = Vec::with_capacity(rs_source.len());
    for rs_path in rs_source {
        let path = format!("{}/{}", manifest_dir, rs_path.display());
        println!("cargo:rerun-if-changed={}", path);

        let generated_code = GeneratedCpp::new(&path);
        generated_file_paths.push(generated_code.write_to_directories(&cpp_directory, &header_dir));
    }

    generated_file_paths
}

/// Run cxx-qt's C++ code generator on Rust modules marked with the [cxx_qt::bridge] macro, compile
/// the code, and link to Qt. This is the complement of the [cxx_qt::bridge] macro, which the Rust
/// compiler uses to generate the corresponding Rust code. No dependencies besides Qt, a C++17 compiler,
/// and Rust toolchain are required.
///
/// For example, if your [cxx_qt::bridge] module is in a file called `src/lib.rs` within your crate,
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
/// cxx-qt's autogenerated headers by having the C++ build system set the `CXXQT_EXPORT_DIR`
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
    qobject_headers: Vec<PathBuf>,
    qrc_files: Vec<PathBuf>,
    qt_modules: HashSet<String>,
    cc_builder: cc::Build,
}

impl CxxQtBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        let mut qt_modules = HashSet::new();
        qt_modules.insert("Core".to_owned());
        qt_modules.insert("Gui".to_owned());
        Self {
            rust_sources: vec![],
            qobject_headers: vec![],
            qrc_files: vec![],
            qt_modules,
            cc_builder: cc::Build::new(),
        }
    }

    /// Specify rust file paths to parse through the cxx-qt marco
    /// Relative paths are treated as relative to the path of your crate's Cargo.toml file
    pub fn file(mut self, rust_source: impl AsRef<Path>) -> Self {
        let rust_source = rust_source.as_ref();
        self.rust_sources.push(rust_source.to_path_buf());
        println!("cargo:rerun-if-changed={}", rust_source.display());
        self
    }

    /// Generate C++ files from [Qt resource .qrc files](https://doc.qt.io/qt-6/resources.html).
    /// The generated file needs to be `#include`d in another .cpp file. For example:
    /// ```no_run
    /// # use cxx_qt_build::CxxQtBuilder;
    /// CxxQtBuilder::new()
    ///     .file("src/cxxqt_module.rs")
    ///     .qrc("src/my_resources.qrc")
    ///     .cc_builder(|cc| {
    ///         cc.file("file_with_include.cpp");
    ///     })
    ///     .build();
    /// ```
    ///
    /// In `file_with_include.cpp`:
    /// ```C++
    /// #include "my_resources.qrc.cpp"
    /// ```
    ///
    /// You also need to [explicitly load](https://doc.qt.io/qt-6/resources.html#explicit-loading-and-unloading-of-embedded-resources)
    /// the resources in your .cpp file by calling `qInitResources()` once before starting your application.
    pub fn qrc(mut self, qrc_file: impl AsRef<Path>) -> Self {
        let qrc_file = qrc_file.as_ref();
        self.qrc_files.push(qrc_file.to_path_buf());
        println!("cargo:rerun-if-changed={}", qrc_file.display());
        self
    }

    /// Link additional [Qt modules](https://doc.qt.io/qt-6/qtmodules.html).
    /// Specify their names without the `Qt` prefix, for example `"Widgets"`.
    /// The Core and Gui modules are linked automatically; there is no need to specify them.
    pub fn qt_modules(mut self, modules: &[&str]) -> Self {
        self.qt_modules
            .extend(modules.iter().cloned().map(String::from));
        self
    }

    /// Specify a C++ header containing a Q_OBJECT macro to run [moc](https://doc.qt.io/qt-6/moc.html) on.
    /// This allows building QObject C++ subclasses besides the ones autogenerated by cxx-qt.
    pub fn qobject_header(mut self, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        self.qobject_headers.push(path.to_path_buf());
        println!("cargo:rerun-if-changed={}", path.display());
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

    /// Generate and compile cxx-qt C++ code, as well as compile any additional files from
    /// [CxxQtBuilder::qobject_header] and [CxxQtBuilder::cc_builder].
    pub fn build(mut self) {
        self.cc_builder.cpp(true);
        // MSVC
        self.cc_builder.flag_if_supported("/std:c++17");
        self.cc_builder.flag_if_supported("/Zc:__cplusplus");
        self.cc_builder.flag_if_supported("/permissive-");
        // GCC + Clang
        self.cc_builder.flag_if_supported("-std=c++17");

        let mut qtbuild = qt_build::QtBuild::new(self.qt_modules.into_iter().collect())
            .expect("Could not find Qt installation");
        qtbuild.cargo_link_libraries();
        for include_dir in qtbuild.include_paths() {
            self.cc_builder.include(include_dir);
        }

        // The include directory needs to be namespaced by crate name when exporting for a C++ build system,
        // but for using cargo build without a C++ build system, OUT_DIR is already namespaced by crate name.
        let header_root = match env::var("CXXQT_EXPORT_DIR") {
            Ok(export_dir) => format!("{}/{}", export_dir, env::var("CARGO_PKG_NAME").unwrap()),
            Err(_) => env::var("OUT_DIR").unwrap(),
        };
        self.cc_builder.include(&header_root);
        let generated_header_dir = format!("{}/cxx-qt-gen/include", header_root);

        cxx_qt_lib_headers::write_headers(&format!("{}/cxx-qt-lib/include", header_root));

        // Write cxx header
        std::fs::create_dir_all(&format!("{}/rust", header_root))
            .expect("Could not create cxx header directory");
        let h_path = format!("{}/rust/cxx.h", header_root);
        // Wrap the File in a block scope so the file is closed before the compiler is run.
        // Otherwise MSVC fails to open cxx.h because the process for this build script already has it open.
        {
            let mut header = File::create(&h_path).expect("Could not create cxx.h");
            write!(header, "{}", cxx_gen::HEADER).expect("Could not write cxx.h");
        }

        // Generate files
        for files in generate_cxxqt_cpp_files(&self.rust_sources, &generated_header_dir) {
            self.cc_builder.file(files.plain_cpp);
            if let (Some(qobject), Some(qobject_header)) = (files.qobject, files.qobject_header) {
                self.cc_builder.file(&qobject);
                self.qobject_headers.push(qobject_header);
            }
        }

        // Run moc on C++ headers with Q_OBJECT macro
        for qobject_header in self.qobject_headers {
            self.cc_builder.file(qtbuild.moc(&qobject_header));
        }

        // Generate code from .qrc files, but do not compile it. Instead, the user needs to #include them
        // in a .cpp file. Otherwise, MSVC won't link if the generated C++ is built separately.
        for qrc_file in self.qrc_files {
            qtbuild.qrc(&qrc_file);
        }

        self.cc_builder.compile("cxx-qt-gen");
    }
}
