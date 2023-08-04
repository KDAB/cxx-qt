// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
mod generator;
mod parser;
mod syntax;
mod writer;

pub use generator::{
    cpp::{fragment::CppFragment, GeneratedCppBlocks},
    rust::GeneratedRustBlocks,
};
pub use parser::{qobject::QmlElementMetadata, Parser};
pub use syntax::{parse_qt_file, CxxQtItem};
pub use writer::{cpp::write_cpp, rust::write_rust};

use std::{fs::File, io::Write, path::Path};
pub use syn::{Error, Result};

/// Write the cxx-qt-gen headers to the specified directory.
pub fn write_headers(directory: impl AsRef<Path>) {
    let directory = directory.as_ref();
    std::fs::create_dir_all(directory).expect("Could not create cxx-qt-gen header directory");
    for (file_contents, file_name) in [
        (
            include_str!("../include/cxxqt_locking.h"),
            "cxxqt_locking.h",
        ),
        (
            include_str!("../include/cxxqt_maybelockguard.h"),
            "cxxqt_maybelockguard.h",
        ),
        (include_str!("../include/cxxqt_thread.h"), "cxxqt_thread.h"),
        (
            include_str!("../include/cxxqt_threading.h"),
            "cxxqt_threading.h",
        ),
        (include_str!("../include/cxxqt_type.h"), "cxxqt_type.h"),
    ] {
        // Note that we do not need rerun-if-changed for these files
        // as include_str causes a rerun when the header changes
        // and the files are always written to the target.
        let h_path = format!("{}/{file_name}", directory.display());
        let mut header = File::create(h_path).expect("Could not create cxx-qt-gen header");
        write!(header, "{file_contents}").expect("Could not write cxx-qt-gen header");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use clang_format::{clang_format_with_style, ClangFormatStyle};
    use generator::{cpp::GeneratedCppBlocks, rust::GeneratedRustBlocks};
    use parser::Parser;
    use pretty_assertions::assert_str_eq;
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use std::{
        env,
        fs::OpenOptions,
        io::Write,
        path::{Path, PathBuf},
    };
    use writer::{cpp::write_cpp, rust::write_rust};

    /// Helper to ensure that a given syn item is the same as the given TokenStream
    pub fn assert_tokens_eq<T: ToTokens>(item: &T, tokens: TokenStream) {
        assert_str_eq!(item.to_token_stream().to_string(), tokens.to_string());
    }

    /// Helper for formating C++ code
    pub(crate) fn format_cpp(cpp_code: &str) -> String {
        clang_format_with_style(cpp_code, &ClangFormatStyle::File).unwrap()
    }

    /// Helper for format Rust code
    fn format_rs_source(rs_code: &str) -> String {
        // NOTE: this error handling is pretty rough so should only used for tests
        let mut command = std::process::Command::new("rustfmt");
        let mut child = command
            .args(["--emit", "stdout"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        // Scope stdin to force an automatic flush
        {
            let mut stdin = child.stdin.take().unwrap();
            write!(stdin, "{rs_code}").unwrap();
        }

        let output = child.wait_with_output().unwrap();
        let output = String::from_utf8(output.stdout).unwrap();

        // Quote does not retain empty lines so we throw them away in the case of the
        // reference string as to not cause clashes
        output.replace("\n\n", "\n")
    }

    fn sanitize_code(mut code: String) -> String {
        code.retain(|c| c != '\r');
        code
    }

    fn update_expected_file(path: PathBuf, source: &str) {
        println!("Updating expected file: {:?}", path);

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();
        file.write_all(source.as_bytes()).unwrap();
    }

    fn update_expected(test_name: &str, rust: &str, header: &str, source: &str) -> bool {
        // Ideally we'd be able to get the path from `file!()`, but that unfortunately only
        // gives us a relative path, which isn't really all that useful.
        // So require the path of the crate to be set via an environment variable.
        //
        // In the simplest case this can be achieved by running:
        //
        //      CXXQT_UPDATE_EXPECTED=$(pwd) cargo test
        //
        if let Ok(path) = env::var("CXXQT_UPDATE_EXPECTED") {
            let output_folder = Path::new(&path);
            let output_folder = output_folder.join("test_outputs");

            let update = |file_ending, contents| {
                update_expected_file(
                    output_folder.join(format!("{test_name}.{file_ending}")),
                    contents,
                );
            };
            update("rs", rust);
            update("h", header);
            update("cpp", source);

            true
        } else {
            false
        }
    }

    fn test_code_generation_internal(
        test_name: &str,
        input: &str,
        expected_rust_output: &str,
        expected_cpp_header: &str,
        expected_cpp_source: &str,
    ) {
        let parser = Parser::from(syn::parse_str(input).unwrap()).unwrap();

        let generated_cpp = GeneratedCppBlocks::from(&parser).unwrap();
        let (header, source) =
            if let CppFragment::Pair { header, source } = write_cpp(&generated_cpp) {
                (sanitize_code(header), sanitize_code(source))
            } else {
                panic!("Expected CppFragment::Pair")
            };

        let generated_rust = GeneratedRustBlocks::from(&parser).unwrap();
        let rust = sanitize_code(format_rs_source(&write_rust(&generated_rust).to_string()));

        if !update_expected(test_name, &rust, &header, &source) {
            assert_str_eq!(sanitize_code(expected_cpp_header.to_owned()), header);
            assert_str_eq!(sanitize_code(expected_cpp_source.to_owned()), source);
            assert_str_eq!(sanitize_code(expected_rust_output.to_owned()), rust);
        }
    }

    /// Helper for testing if a given input Rust file generates the expected C++ & Rust code
    /// This needs to be a macro rather than a function because include_str needs the file path at compile time.
    macro_rules! test_code_generation {
        ( $file_stem:literal ) => {
            test_code_generation_internal(
                $file_stem,
                include_str!(concat!("../test_inputs/", $file_stem, ".rs")),
                include_str!(concat!("../test_outputs/", $file_stem, ".rs")),
                include_str!(concat!("../test_outputs/", $file_stem, ".h")),
                include_str!(concat!("../test_outputs/", $file_stem, ".cpp")),
            );
        };
    }

    #[test]
    fn generates_invokables() {
        test_code_generation!("invokables");
    }

    #[test]
    fn generates_passthrough_and_naming() {
        test_code_generation!("passthrough_and_naming");
    }

    #[test]
    fn generates_properties() {
        test_code_generation!("properties");
    }

    #[test]
    fn generates_signals() {
        test_code_generation!("signals");
    }

    #[test]
    fn generates_inheritance() {
        test_code_generation!("inheritance");
    }
}
