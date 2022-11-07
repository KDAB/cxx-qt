// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use super::*;

use clang_format::{clang_format, ClangFormatStyle, CLANG_FORMAT_STYLE};
use generator::{cpp::GeneratedCppBlocks, rust::GeneratedRustBlocks};
use parser::Parser;
use pretty_assertions::assert_str_eq;
use writer::{cpp::write_cpp, rust::write_rust};

pub mod utils;

#[ctor::ctor]
fn init_tests() {
    // Set the ClangFormatStyle to be Mozilla for our tests
    // so that when they fail the format in the assertions is the same as the files.
    assert!(CLANG_FORMAT_STYLE.set(ClangFormatStyle::Mozilla).is_ok());
}

/// Helper for testing if a given input Rust file generates the expected C++ & Rust code
/// This needs to be a macro rather than a function because include_str needs the file path at compile time.
macro_rules! test_code_generation {
    ( $file_stem:literal ) => {
        let parser = Parser::from(
            syn::parse_str(include_str!(concat!(
                "../../test_inputs/",
                $file_stem,
                ".rs"
            )))
            .unwrap(),
        )
        .unwrap();

        let generated_cpp = GeneratedCppBlocks::from(&parser).unwrap();
        let cpp = write_cpp(&generated_cpp);
        let expected_cpp_header = clang_format(include_str!(concat!(
            "../../test_outputs/",
            $file_stem,
            ".h"
        )))
        .unwrap();
        let expected_cpp_source = clang_format(include_str!(concat!(
            "../../test_outputs/",
            $file_stem,
            ".cpp"
        )))
        .unwrap();
        assert_str_eq!(cpp.header, expected_cpp_header);
        assert_str_eq!(cpp.source, expected_cpp_source);

        let generated_rust = GeneratedRustBlocks::from(&parser).unwrap();
        let rust = utils::format_rs_source(&write_rust(&generated_rust).to_string());
        let expected_rust_output = utils::format_rs_source(include_str!(concat!(
            "../../test_outputs/",
            $file_stem,
            ".rs"
        )));
        assert_str_eq!(rust, expected_rust_output);
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
