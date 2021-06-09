use cxx_qt_gen::extract_qobject;
use quote::ToTokens;
use std::env;
use std::fs::File;
use std::io::Write;
use syn::*;

use clang_format::ClangFormatStyle;
use cxx_qt_gen::{generate_format, generate_qobject_cxx};

// TODO: we need to eventually support having multiple modules defined in a single file. This
// is currently an issue because we are using the Rust file name to derive the cpp file name
// and are blindly re-writing files.
//
// As we use struct names for the QObject files, we should actually be able to support multiple
// QObject macros and at most one "raw CXX" macro per file already. For now this remains a TODO
// as to keep things simpler. We also want to able to warn users about duplicate names eventually.

/// Retrieve the list of rust sources from the file
fn read_rs_sources() -> Vec<String> {
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");
    let path = format!("{}/target/cxx-qt/rust_sources.txt", dir_manifest);

    let contents = std::fs::read_to_string(path).expect("Could not read list of Rust source files");
    contents.split(';').map(|s| s.to_string()).collect()
}

/// Tests if an attributes matched what is expected for #[cxx::bridge]
fn is_cxx_attr(attr: &Attribute) -> bool {
    let segments = &attr.path.segments;

    if segments.len() != 2 {
        return false;
    }

    if segments[0].ident != "cxx" {
        return false;
    }

    if segments[1].ident != "bridge" {
        return false;
    }

    true
}

/// Tests if an attributes matched what is expected for #[make_qobject]
fn is_cxx_qt_attr(attr: &Attribute) -> bool {
    let segments = &attr.path.segments;

    if segments.len() != 1 {
        return false;
    }

    if segments[0].ident != "make_qobject" {
        return false;
    }

    true
}

/// Represents the cxx or cxx_qt module that could be extracted from a file
#[derive(PartialEq)]
enum ExtractedModule {
    Cxx(ItemMod),
    CxxQt(ItemMod),
    None,
}

/// Extract the cxx or cxx_qt module from a Rust file
fn extract_modules(file_content: &str, rs_path: &str) -> ExtractedModule {
    let file = syn::parse_file(file_content).unwrap();

    // Define a helper function that will ensure that we can extract at most one
    // module for code gen from the items in the file. This function also
    // ensures that the extracted item is wrapped in the correct enum.
    let mut extracted = ExtractedModule::None;
    let mut push_module = |i: ItemMod, qt: bool| {
        if extracted != ExtractedModule::None {
            panic!(
                "Unfortunately only files with either a single cxx or a single cxx_qt module are currently supported.
                The file {} has more than one of these.",
                rs_path);
        }

        if qt {
            extracted = ExtractedModule::CxxQt(i);
        } else {
            extracted = ExtractedModule::Cxx(i);
        }
    };

    // We loop through all the items in the module searching for any that we can
    // generate code from. We do not break out of the loop so that we can detect
    // if the users placed multiple such modules in a single file and give them a
    // warning.
    for i in file.items {
        if let Item::Mod(m) = i {
            let attrs = &m.attrs;
            match attrs.len() {
                0 => continue,
                1 => {}
                _others => panic!("Multiple module attributes are currently not supported."),
            }

            let attr = &attrs[0];
            if is_cxx_attr(attr) {
                push_module(m, false);
            } else if is_cxx_qt_attr(attr) {
                push_module(m, true);
            }
        }
    }

    extracted
}

/// Generate C++ files from a given Rust file, returning the generated path
fn gen_cxx_for_file(rs_path: &str) -> String {
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");

    let path = format!("{}/src/{}", dir_manifest, rs_path);
    let content = std::fs::read_to_string(path).expect("Could not read Rust file");
    let extracted = extract_modules(&content, rs_path);

    let tokens = {
        match extracted {
            ExtractedModule::Cxx(m) => m.into_token_stream(),
            ExtractedModule::CxxQt(m) => {
                let qobject = extract_qobject(m).unwrap();
                generate_qobject_cxx(&qobject).unwrap()
            }
            _others => panic!(
                "No module to generate cxx code from could be found in {}",
                rs_path
            ),
        }
    };

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

    cpp_path
}

/// Generate C++ files from a given list of Rust files, returning the generated paths
fn gen_cxx_for_files(rs_source: &[String]) -> Vec<String> {
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");

    let path = format!("{}/target/cxx-qt/include", dir_manifest);
    std::fs::create_dir_all(path).expect("Could not create cxx-qt include dir");

    let path = format!("{}/target/cxx-qt/src", dir_manifest);
    std::fs::create_dir_all(path).expect("Could not create cxx-qt src dir");

    let mut cpp_files = Vec::new();

    for rs_path in rs_source {
        let cpp_path = gen_cxx_for_file(rs_path);
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
        let cpp_paths = gen_cxx_for_files(&rs_sources);

        // TODO: find a way to only do this when cargo is called during the config stage of CMake
        write_cpp_sources(&cpp_paths);
    }
}
