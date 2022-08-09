// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clang_format::ClangFormatStyle;
use cxx_qt_gen::{
    extract_qobject, generate_format, generate_qobject_cpp, generate_qobject_rs, parse_qt_file,
    CppObject, CxxQtItem,
};

/// Representation of a generated CXX header, source, and name
#[derive(Serialize, Deserialize)]
struct GeneratedType {
    header: String,
    name: String,
    source: String,
}

// TODO: we need to eventually support having multiple modules defined in a single file. This
// is currently an issue because we are using the Rust file name to derive the cpp file name
// and are blindly re-writing files.
//
// As we use struct names for the QObject files, we should actually be able to support multiple
// QObject macros and at most one "raw CXX" macro per file already. For now this remains a TODO
// as to keep things simpler. We also want to able to warn users about duplicate names eventually.

fn manifest_dir() -> String {
    let mut manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");
    // CARGO_MANIFEST_DIR uses \ path separators on Windows, but the format! macros that
    // use the return value of this function use / separators. CMake's `file(STRINGS)` command
    // which is used to load list of generated file paths fails if \ and / path separators
    // are mixed. CMake also fails when using all \ separators because it treats them as escaping
    // the character after the \
    if cfg!(windows) {
        manifest_dir = manifest_dir.replace('\\', "/");
    }
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");
    manifest_dir
}

pub struct GeneratedCpp {
    cxx_qt: Option<CppObject>,
    cxx: cxx_gen::GeneratedCode,
    file_ident: String,
}

impl GeneratedCpp {
    /// Generate QObject and cxx header/source C++ file contents
    pub fn new(rust_file_path: &impl AsRef<std::path::Path>) -> Self {
        let file = parse_qt_file(rust_file_path).unwrap();

        let mut cxx_qt = None;
        // Use the Rust filename as the resultant CXX/CXX-Qt names
        //
        // TODO: we should consider sub directories so you can have a/b/c and a/d/c and not collide
        let file_ident: String = rust_file_path
            .as_ref()
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let mut tokens = proc_macro2::TokenStream::new();

        // Add any attributes in the file into the tokenstream
        for attr in &file.attrs {
            tokens.extend(attr.into_token_stream());
        }

        // Loop through the items looking for any CXX or CXX-Qt blocks
        for item in &file.items {
            match item {
                CxxQtItem::Cxx(m) => tokens.extend(m.into_token_stream()),
                CxxQtItem::CxxQt(m) => {
                    // TODO: later we will allow for multiple CXX-Qt blocks in one file
                    if cxx_qt.is_some() {
                        panic!(
                            "Unfortunately only files with either a single cxx_qt module are currently supported.
                            The file {} has more than one of these.",
                            rust_file_path.as_ref().display());
                    }

                    // TODO: later we will likely have cxx_qt_gen::generate_header_and_cpp
                    // which will take a CxxQtItemMod and respond with a C++ header and source
                    let qobject = extract_qobject(m, rust_file_path.as_ref().to_path_buf()).unwrap();
                    // TODO: we'll have to extend the C++ data here rather than overwriting
                    // assuming we share the same file
                    cxx_qt = Some(generate_qobject_cpp(&qobject).unwrap());

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

    /// Write generated code to files in a directory. Returns the absolute paths of all files written.
    pub fn write_to_directory(&self, directory: &impl AsRef<std::path::Path>) -> Vec<PathBuf> {
        let directory = directory.as_ref();
        if !directory.is_dir() {
            panic!(
                "Output directory {} is not a directory",
                directory.display()
            );
        }

        let include_directory_path = PathBuf::from(format!("{}/include", &directory.display()));
        std::fs::create_dir_all(&include_directory_path)
            .expect("Could not create cxx-qt include dir");

        let source_directory_path = PathBuf::from(format!("{}/src", &directory.display()));
        std::fs::create_dir_all(&source_directory_path)
            .expect("Could not create cxx-qt source dir");

        let mut written_files = Vec::with_capacity(4);

        if let Some(cxx_qt_generated) = &self.cxx_qt {
            let header_path = PathBuf::from(format!(
                "{}/{}.cxxqt.h",
                include_directory_path.display(),
                self.file_ident
            ));
            let mut header =
                File::create(&header_path).expect("Could not create cxx-qt header file");
            header
                .write_all(cxx_qt_generated.header.as_bytes())
                .expect("Could not write cxx-qt header file");
            written_files.push(header_path);

            let cpp_path = PathBuf::from(format!(
                "{}/{}.cxxqt.cpp",
                source_directory_path.display(),
                self.file_ident
            ));
            let mut cpp = File::create(&cpp_path).expect("Could not create cxx-qt source file");
            cpp.write_all(cxx_qt_generated.source.as_bytes())
                .expect("Could not write cxx-qt source file");
            written_files.push(cpp_path);
        }

        let header_path = PathBuf::from(format!(
            "{}/{}.cxx.h",
            include_directory_path.display(),
            self.file_ident
        ));
        let mut header = File::create(&header_path).expect("Could not create cxx header file");
        header
            .write_all(&self.cxx.header)
            .expect("Could not write cxx header file");
        written_files.push(header_path);

        let cpp_path = PathBuf::from(format!(
            "{}/{}.cxx.cpp",
            source_directory_path.display(),
            self.file_ident
        ));
        let mut cpp = File::create(&cpp_path).expect("Could not create cxx source file");
        cpp.write_all(&self.cxx.implementation)
            .expect("Could not write cxx source file");
        written_files.push(cpp_path);

        written_files
    }
}

/// Generate C++ files from a given list of Rust files, returning the generated paths
fn write_cxx_generated_files_for_cargo(rs_source: &[&'static str]) -> Vec<PathBuf> {
    let manifest_dir = manifest_dir();
    let directory = format!("{}/target/cxx-qt-gen", manifest_dir);
    std::fs::create_dir_all(&directory).expect("Could not create cxx-qt code generation directory");

    let mut cpp_files = Vec::new();

    for rs_path in rs_source {
        let path = format!("{}/{}", manifest_dir, rs_path);
        println!("cargo:rerun-if-changed={}", path);

        let generated_code = GeneratedCpp::new(&path);
        cpp_files.append(&mut generated_code.write_to_directory(&directory));
    }

    cpp_files
}

/// Write the list of C++ paths to the file
fn write_cpp_sources_list(paths: &[PathBuf]) {
    let manifest_dir = manifest_dir();

    let path = format!("{}/target/cxx-qt-gen", manifest_dir);
    std::fs::create_dir_all(path).expect("Could not create target dir");

    let path = format!("{}/target/cxx-qt-gen/cpp_sources.txt", manifest_dir);
    let mut file = File::create(&path).expect("Could not create cpp_sources file");

    for path in paths {
        writeln!(file, "{}", path.display()).unwrap();
    }
}

/// Write out the static header file for both the cxx
fn write_cxx_static_header() {
    let manifest_dir = manifest_dir();

    let path = format!("{}/target/cxx-qt-gen/statics/rust", manifest_dir);
    std::fs::create_dir_all(&path).expect("Could not create static header dir");

    let h_path = format!("{}/cxx.h", path);
    let mut header = File::create(&h_path).expect("Could not create cxx.h");
    write!(header, "{}", cxx_gen::HEADER).expect("Could not write cxx.h");
}

/// Describes a cxx Qt builder which helps parse and generate sources for cxx-qt
#[derive(Default)]
pub struct CxxQtBuilder {
    cpp_format: Option<ClangFormatStyle>,
    rust_sources: Vec<&'static str>,
    qt_enabled: bool,
}

impl CxxQtBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            cpp_format: None,
            rust_sources: vec![],
            qt_enabled: true,
        }
    }

    /// Choose the ClangFormatStyle to use for generated C++ files
    pub fn cpp_format(mut self, format: ClangFormatStyle) -> Self {
        self.cpp_format = Some(format);
        self
    }

    /// Choose to disable Qt support
    ///
    /// This will disable including cxx-qt-lib headers.
    pub fn disable_qt(mut self) -> Self {
        self.qt_enabled = false;
        self
    }

    /// Specify rust file paths to parse through the cxx-qt marco
    ///
    /// Currently the path should be relative to CARGO_MANIFEST_DIR
    pub fn file(mut self, rust_source: &'static str) -> Self {
        self.rust_sources.push(rust_source);
        self
    }

    // TODO: support globs with files("src/**/*.rs")

    /// Perform the build task, for example parsing and generating sources
    pub fn build(self) {
        // Set clang-format format
        if generate_format(self.cpp_format).is_err() {
            panic!("Failed to set clang-format.");
        }

        // TODO: somewhere check that we don't have duplicate class names
        // TODO: later use the module::object to turn into module/object.h

        // Generate files
        let cpp_paths = write_cxx_generated_files_for_cargo(&self.rust_sources);

        // TODO: in large projects where where CXX-Qt is used in multiple individual
        // components that end up being linked together, having these same static
        // files in each one could cause issues.
        write_cxx_static_header();

        // TODO: find a way to only do this when cargo is called during the config stage of CMake
        write_cpp_sources_list(&cpp_paths);
    }
}
