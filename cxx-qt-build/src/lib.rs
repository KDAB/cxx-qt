use proc_macro2::TokenStream;
use std::env;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

use clang_format::ClangFormatStyle;
use cxx_qt_gen::generate_format;

/// Retrieve the list of rust sources from the file
fn read_rs_sources() -> Vec<String> {
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");
    let path = format!("{}/target/cxx-qt/rust_sources.txt", dir_manifest);

    let contents = std::fs::read_to_string(path).expect("Could not read list of Rust source files");
    contents.split(';').map(|s| s.to_string()).collect()
}

/// Generate C++ files from a given list of rust files, returning the generated paths
fn gen_cxx_files(rs_source: &[String]) -> Vec<String> {
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");

    let path = format!("{}/target/cxx-qt/include", dir_manifest);
    std::fs::create_dir_all(path).expect("Could not create cxx-qt include dir");

    let path = format!("{}/target/cxx-qt/src", dir_manifest);
    std::fs::create_dir_all(path).expect("Could not create cxx-qt src dir");

    let mut cpp_files = Vec::new();

    for rs_path in rs_source {
        let path = format!("{}/src/{}", dir_manifest, rs_path);
        let content = std::fs::read_to_string(path).expect("Could not read Rust file");

        let tokens = TokenStream::from_str(&content).expect("Could not parse Rust file");
        let opt = cxx_gen::Opt::default();
        let gen_result = cxx_gen::generate_header_and_cc(tokens, &opt)
            .expect("Could not generate C++ from Rust file");

        let header_path = format!("{}/target/cxx-qt/include/{}.h", dir_manifest, rs_path);
        let mut header = File::create(header_path).expect("Could not create header file");
        header
            .write_all(&gen_result.header)
            .expect("Could not write header file");

        let cpp_path = format!("{}/target/cxx-qt/src/{}.cpp", dir_manifest, rs_path);
        let mut cpp = File::create(&cpp_path).expect("Could not create cpp file");
        cpp.write_all(&gen_result.implementation)
            .expect("Could not write cpp file");

        cpp_files.push(cpp_path);
    }

    cpp_files
}

/// Write the list of C++ paths to the file
fn write_cpp_sources(paths: &[String]) {
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");

    let path = format!("{}/target/cxx-qt", dir_manifest);
    std::fs::create_dir_all(path).expect("Could not create target dir");

    let path = format!("{}/target/cxx-qt/cpp_sources.txt", dir_manifest);
    let mut file = File::create(&path).expect("Could not create cpp_sources file");

    for path in paths {
        writeln!(file, "{}", path).unwrap();
    }
}

/// Describes a cxx Qt builder which helps parse and generate sources for cxx-qt
#[derive(Default)]
pub struct CxxQtBuilder {
    cpp_format: Option<ClangFormatStyle>,
}

impl CxxQtBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self { cpp_format: None }
    }

    /// Choose the ClangFormatStyle to use for generated C++ files
    pub fn cpp_format(mut self, format: ClangFormatStyle) -> CxxQtBuilder {
        self.cpp_format = Some(format);
        self
    }

    /// Perform the build task, for example parsing and generating sources
    pub fn build(self) {
        // Set clang-format format
        if generate_format(self.cpp_format).is_err() {
            panic!("Failed to set clang-format.");
        }

        // Read sources
        let rs_sources = read_rs_sources();

        // Generate files
        let cpp_paths = gen_cxx_files(&rs_sources);

        // TODO: find a way to only do this when cargo is called during the config stage of CMake
        write_cpp_sources(&cpp_paths);
    }
}
