// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(missing_docs)]

//! This crate provides a builder which parses given Rust source code to search
//! for CXX-Qt or CXX macros and generate any resulting C++ code. It also builds
//! the C++ code into a binary with any cxx-qt-lib code and Qt linked.

use convert_case::{Case, Casing};
use quote::ToTokens;
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use cxx_qt_gen::{
    parse_qt_file, write_cpp, write_rust, CppFragment, CxxQtItem, GeneratedCppBlocks,
    GeneratedRustBlocks, Parser, QmlElementMetadata,
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
    qobject_header: Option<QObjectHeader>,
}

struct QObjectHeader {
    path: PathBuf,
    qml_metadata: Vec<QmlElementMetadata>,
}

struct GeneratedCpp {
    cxx_qt: Option<CppFragment>,
    cxx: cxx_gen::GeneratedCode,
    file_ident: String,
    qml_metadata: Vec<QmlElementMetadata>,
}

impl GeneratedCpp {
    /// Generate QObject and cxx header/source C++ file contents
    pub fn new(rust_file_path: impl AsRef<Path>) -> cxx_qt_gen::Result<Self> {
        let rust_file_path = rust_file_path.as_ref();
        let file = parse_qt_file(rust_file_path).unwrap();

        let mut cxx_qt = None;
        let mut qml_metadata = Vec::new();
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

                    let parser = Parser::from(m.clone())?;
                    let generated_cpp = GeneratedCppBlocks::from(&parser)?;
                    // TODO: we'll have to extend the C++ data here rather than overwriting
                    // assuming we share the same file
                    cxx_qt = Some(write_cpp(&generated_cpp));

                    let generated_rust = GeneratedRustBlocks::from(&parser)?;
                    let rust_tokens = write_rust(&generated_rust);
                    file_ident = parser.cxx_file_stem.clone();
                    for (_, qobject) in parser.cxx_qt_data.qobjects {
                        if let Some(q) = qobject.qml_metadata {
                            qml_metadata.push(q);
                        }
                    }

                    // We need to do this and can't rely on the macro, as we need to generate the
                    // CXX bridge Rust code that is then fed into the cxx_gen generation.
                    tokens.extend(rust_tokens);
                }
                CxxQtItem::Item(item) => {
                    tokens.extend(item.into_token_stream());
                }
            }
        }

        let opt = cxx_gen::Opt::default();
        let cxx = cxx_gen::generate_header_and_cc(tokens, &opt)
            .expect("Could not generate C++ from Rust file");

        Ok(GeneratedCpp {
            cxx_qt,
            cxx,
            file_ident,
            qml_metadata,
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
        for directory in [cpp_directory, header_directory] {
            std::fs::create_dir_all(directory)
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
            let header_generated = match cxx_qt_generated {
                CppFragment::Pair { header, source: _ } => header,
                CppFragment::Header(header) => header,
                CppFragment::Source(_) => panic!("Unexpected call for source fragment."),
            };
            header
                .write_all(header_generated.as_bytes())
                .expect("Could not write cxx-qt header file");
            cpp_file_paths.qobject_header = Some(QObjectHeader {
                path: header_path,
                qml_metadata: self.qml_metadata,
            });

            let cpp_path = PathBuf::from(format!(
                "{}/{}.cxxqt.cpp",
                cpp_directory.display(),
                self.file_ident
            ));
            let mut cpp = File::create(&cpp_path).expect("Could not create cxx-qt source file");
            let source_generated = match cxx_qt_generated {
                CppFragment::Pair { header: _, source } => source,
                CppFragment::Header(_) => panic!("Unexpected call for header fragment."),
                CppFragment::Source(source) => source,
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
        let mut header = File::create(header_path).expect("Could not create cxx header file");
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
) -> cxx_qt_gen::Result<Vec<GeneratedCppFilePaths>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut generated_file_paths: Vec<GeneratedCppFilePaths> = Vec::with_capacity(rs_source.len());
    for rs_path in rs_source {
        let cpp_directory = format!("{}/cxx-qt-gen/src", env::var("OUT_DIR").unwrap());
        let path = format!("{manifest_dir}/{}", rs_path.display());
        println!("cargo:rerun-if-changed={path}");

        let generated_code = GeneratedCpp::new(&path)?;
        generated_file_paths.push(generated_code.write_to_directories(cpp_directory, &header_dir));
    }

    Ok(generated_file_paths)
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
    qobject_headers: Vec<QObjectHeader>,
    qrc_files: Vec<PathBuf>,
    qt_modules: HashSet<String>,
    cc_builder: cc::Build,
}

impl CxxQtBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        let mut qt_modules = HashSet::new();
        qt_modules.insert("Core".to_owned());
        #[cfg(feature = "qt_gui")]
        qt_modules.insert("Gui".to_owned());
        #[cfg(feature = "qt_qml")]
        qt_modules.insert("Qml".to_owned());
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
    /// The Core and any feature enabled modules are linked automatically; there is no need to specify them.
    pub fn qt_module(mut self, module: &str) -> Self {
        self.qt_modules.insert(module.to_owned());
        self
    }

    /// Specify a C++ header containing a Q_OBJECT macro to run [moc](https://doc.qt.io/qt-6/moc.html) on.
    /// This allows building QObject C++ subclasses besides the ones autogenerated by cxx-qt.
    pub fn qobject_header(mut self, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        self.qobject_headers.push(QObjectHeader {
            path: path.to_owned(),
            qml_metadata: Vec::new(),
        });
        println!("cargo:rerun-if-changed={}", path.display());
        self
    }

    /// Convenience wrapper around [qt_build_utils::setup_linker].
    pub fn setup_linker(self) -> Self {
        qt_build_utils::setup_linker();
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
        let out_dir = env::var("OUT_DIR").unwrap();
        // The include directory needs to be namespaced by crate name when exporting for a C++ build system,
        // but for using cargo build without a C++ build system, OUT_DIR is already namespaced by crate name.
        let header_root = match env::var("CXXQT_EXPORT_DIR") {
            Ok(export_dir) => format!("{export_dir}/{}", env::var("CARGO_PKG_NAME").unwrap()),
            Err(_) => out_dir,
        };
        let generated_header_dir = format!("{header_root}/cxx-qt-gen");

        let mut qtbuild = qt_build_utils::QtBuild::new(self.qt_modules.into_iter().collect())
            .expect("Could not find Qt installation");
        qtbuild.cargo_link_libraries();

        // Write cxx-qt-lib and cxx headers
        cxx_qt_lib_headers::write_headers(format!("{header_root}/cxx-qt-lib"));
        std::fs::create_dir_all(format!("{header_root}/rust"))
            .expect("Could not create cxx header directory");
        let h_path = format!("{header_root}/rust/cxx.h");
        // Wrap the File in a block scope so the file is closed before the compiler is run.
        // Otherwise MSVC fails to open cxx.h because the process for this build script already has it open.
        {
            let mut header = File::create(h_path).expect("Could not create cxx.h");
            write!(header, "{}", cxx_gen::HEADER).expect("Could not write cxx.h");
        }

        // Setup compiler
        // Static QML plugin and Qt resource initialization need to be linked with +whole-archive
        // because they use static variables which need to be initialized before main
        // (regardless of whether main is in Rust or C++). Normally linkers only copy symbols referenced
        // from within main when static linking, which would result in discarding those static variables.
        // Use a separate cc::Build for the little amount of code that needs to be linked with +whole-archive
        // to avoid bloating the binary.
        let mut cc_builder_whole_archive = cc::Build::new();
        cc_builder_whole_archive.link_lib_modifier("+whole-archive");
        for builder in [&mut self.cc_builder, &mut cc_builder_whole_archive] {
            builder.cpp(true);
            // MSVC
            builder.flag_if_supported("/std:c++17");
            builder.flag_if_supported("/Zc:__cplusplus");
            builder.flag_if_supported("/permissive-");
            // GCC + Clang
            builder.flag_if_supported("-std=c++17");
            // Enable Qt Gui in C++ if the feature is enabled
            #[cfg(feature = "qt_gui")]
            builder.define("CXX_QT_GUI_FEATURE", None);
            // Enable Qt Gui in C++ if the feature is enabled
            #[cfg(feature = "qt_qml")]
            builder.define("CXX_QT_QML_FEATURE", None);
            for include_dir in qtbuild.include_paths() {
                builder.include(&include_dir);
            }
            builder.include(&header_root);
            builder.include(&generated_header_dir);
        }

        // Generate files
        match generate_cxxqt_cpp_files(&self.rust_sources, &generated_header_dir) {
            Ok(generated_files) => {
                for files in generated_files {
                    self.cc_builder.file(files.plain_cpp);
                    if let (Some(qobject), Some(qobject_header)) =
                        (files.qobject, files.qobject_header)
                    {
                        self.cc_builder.file(&qobject);
                        self.qobject_headers.push(qobject_header);
                    }
                }
            }
            Err(e) => {
                // Don't panic here because rust-analyzer would fail with a verbose backtrace
                // when running the build script. The same error will be encountered when the proc_macro
                // expands after the build script runs, which allows rust-analyzer to make sense of the
                // error and point the user to the code causing the problem.
                println!("cargo:warning=cxx-qt-build failed to parse cxx_qt::bridge: {e:?}");
                return;
            }
        }

        // To support multiple QML elements with the same import URI, qmltyperegistrar must be run
        // only once for each QML module (URI). So, collect the metadata for all QML elements within
        // each module, regardless of which Rust QObject they are from.
        let mut qml_modules = HashMap::<(String, usize, usize), Vec<PathBuf>>::new();
        let mut cc_builder_whole_archive_files_added = false;
        // Run moc on C++ headers with Q_OBJECT macro
        for qobject_header in self.qobject_headers {
            let moc_products = qtbuild.moc(&qobject_header.path);
            self.cc_builder.file(moc_products.cpp);
            for qml_metadata in qobject_header.qml_metadata {
                self.cc_builder.define("QT_STATICPLUGIN", None);
                qml_modules
                    .entry((
                        qml_metadata.uri.clone(),
                        qml_metadata.version_major,
                        qml_metadata.version_minor,
                    ))
                    .or_default()
                    .push(moc_products.metatypes_json.clone());
            }
        }
        for ((uri, version_major, version_minor), paths) in qml_modules {
            let qml_type_registration_files =
                qtbuild.register_qml_types(&paths, version_major, version_minor, &uri);
            self.cc_builder
                .file(qml_type_registration_files.qmltyperegistrar);
            self.cc_builder.file(qml_type_registration_files.plugin);
            cc_builder_whole_archive.file(qml_type_registration_files.plugin_init);
            cc_builder_whole_archive_files_added = true;
        }
        for qrc_file in self.qrc_files {
            cc_builder_whole_archive.file(qtbuild.qrc(&qrc_file));
            cc_builder_whole_archive_files_added = true;
        }
        if cc_builder_whole_archive_files_added {
            cc_builder_whole_archive.compile("qt-static-initializers");
        }
        self.cc_builder.compile("cxx-qt-gen");
    }
}
