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
    cpp::{fragment::CppFragmentPair, GeneratedCppBlocks},
    rust::GeneratedRustBlocks,
};
pub use parser::Parser;
pub use syntax::{parse_qt_file, CxxQtItem};
pub use writer::{cpp::write_cpp, rust::write_rust};

#[cfg(test)]
mod tests {
    use super::*;

    use clang_format::{clang_format, ClangFormatStyle, CLANG_FORMAT_STYLE};
    use generator::{cpp::GeneratedCppBlocks, rust::GeneratedRustBlocks};
    use parser::Parser;
    use pretty_assertions::assert_str_eq;
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use std::io::Write;
    use writer::{cpp::write_cpp, rust::write_rust};

    #[ctor::ctor]
    fn init_tests() {
        // Set the ClangFormatStyle to be Mozilla for our tests
        // so that when they fail the format in the assertions is the same as the files.
        assert!(CLANG_FORMAT_STYLE.set(ClangFormatStyle::Mozilla).is_ok());
    }

    /// Helper to ensure that a given syn item is the same as the given TokenStream
    pub fn assert_tokens_eq<T: ToTokens>(item: &T, tokens: TokenStream) {
        assert_str_eq!(item.to_token_stream().to_string(), tokens.to_string());
    }

    /// Helper for format Rust code
    fn format_rs_source(rs_code: &str) -> String {
        // NOTE: this error handling is pretty rough so should only used for tests
        let mut command = std::process::Command::new("rustfmt");
        let mut child = command
            .args(&["--emit", "stdout"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        // Scope stdin to force an automatic flush
        {
            let mut stdin = child.stdin.take().unwrap();
            write!(stdin, "{}", rs_code).unwrap();
        }

        let output = child.wait_with_output().unwrap();
        let output = String::from_utf8(output.stdout).unwrap();

        // Quote does not retain empty lines so we throw them away in the case of the
        // reference string as to not cause clashes
        output.replace("\n\n", "\n")
    }

    /// Helper to parse a quote TokenStream into a given syn item
    pub fn tokens_to_syn<T: syn::parse::Parse>(tokens: proc_macro2::TokenStream) -> T {
        syn::parse2(tokens.into_token_stream()).unwrap()
    }

    #[test]
    fn generates_invokables_cpp() {
        let parser =
            Parser::from(syn::parse_str(include_str!("../test_inputs/invokables.rs")).unwrap())
                .unwrap();
        let generated = GeneratedCppBlocks::from(&parser).unwrap();
        let cpp = write_cpp(&generated);

        let expected_header = clang_format(include_str!("../test_outputs/invokables.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/invokables.cpp")).unwrap();
        assert_str_eq!(cpp.header, expected_header);
        assert_str_eq!(cpp.source, expected_source);
    }

    #[test]
    fn generates_naming_cpp() {
        let parser =
            Parser::from(syn::parse_str(include_str!("../test_inputs/naming.rs")).unwrap())
                .unwrap();
        let generated = GeneratedCppBlocks::from(&parser).unwrap();
        let cpp = write_cpp(&generated);

        let expected_header = clang_format(include_str!("../test_outputs/naming.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/naming.cpp")).unwrap();
        assert_str_eq!(cpp.header, expected_header);
        assert_str_eq!(cpp.source, expected_source);
    }

    #[test]
    fn generates_properties_cpp() {
        let parser =
            Parser::from(syn::parse_str(include_str!("../test_inputs/properties.rs")).unwrap())
                .unwrap();
        let generated = GeneratedCppBlocks::from(&parser).unwrap();
        let cpp = write_cpp(&generated);

        let expected_header = clang_format(include_str!("../test_outputs/properties.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/properties.cpp")).unwrap();
        assert_str_eq!(cpp.header, expected_header);
        assert_str_eq!(cpp.source, expected_source);
    }

    #[test]
    fn generates_signals_cpp() {
        let parser =
            Parser::from(syn::parse_str(include_str!("../test_inputs/signals.rs")).unwrap())
                .unwrap();
        let generated = GeneratedCppBlocks::from(&parser).unwrap();
        let cpp = write_cpp(&generated);

        let expected_header = clang_format(include_str!("../test_outputs/signals.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/signals.cpp")).unwrap();
        assert_str_eq!(cpp.header, expected_header);
        assert_str_eq!(cpp.source, expected_source);
    }

    #[test]
    fn generates_types_primitive_property_cpp() {
        let parser = Parser::from(
            syn::parse_str(include_str!("../test_inputs/types_primitive_property.rs")).unwrap(),
        )
        .unwrap();
        let generated = GeneratedCppBlocks::from(&parser).unwrap();
        let cpp = write_cpp(&generated);

        let expected_header =
            clang_format(include_str!("../test_outputs/types_primitive_property.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/types_primitive_property.cpp")).unwrap();
        assert_str_eq!(cpp.header, expected_header);
        assert_str_eq!(cpp.source, expected_source);
    }

    #[test]
    fn generates_types_qt_property_cpp() {
        let parser = Parser::from(
            syn::parse_str(include_str!("../test_inputs/types_qt_property.rs")).unwrap(),
        )
        .unwrap();
        let generated = GeneratedCppBlocks::from(&parser).unwrap();
        let cpp = write_cpp(&generated);

        let expected_header =
            clang_format(include_str!("../test_outputs/types_qt_property.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/types_qt_property.cpp")).unwrap();
        assert_str_eq!(cpp.header, expected_header);
        assert_str_eq!(cpp.source, expected_source);
    }

    #[test]
    fn generates_types_qt_invokable_cpp() {
        let parser = Parser::from(
            syn::parse_str(include_str!("../test_inputs/types_qt_invokable.rs")).unwrap(),
        )
        .unwrap();
        let generated = GeneratedCppBlocks::from(&parser).unwrap();
        let cpp = write_cpp(&generated);

        let expected_header =
            clang_format(include_str!("../test_outputs/types_qt_invokable.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/types_qt_invokable.cpp")).unwrap();
        assert_str_eq!(cpp.header, expected_header);
        assert_str_eq!(cpp.source, expected_source);
    }

    #[test]
    fn generates_custom_default_rust() {
        let parser =
            Parser::from(syn::parse_str(include_str!("../test_inputs/custom_default.rs")).unwrap())
                .unwrap();
        let generated = GeneratedRustBlocks::from(&parser).unwrap();
        let rust = format_rs_source(&write_rust(&generated).to_string());

        let expected_output = format_rs_source(include_str!("../test_outputs/custom_default.rs"));
        assert_str_eq!(rust, expected_output);
    }

    #[test]
    fn generates_invokables_rust() {
        let parser =
            Parser::from(syn::parse_str(include_str!("../test_inputs/invokables.rs")).unwrap())
                .unwrap();
        let generated = GeneratedRustBlocks::from(&parser).unwrap();
        let rust = format_rs_source(&write_rust(&generated).to_string());

        let expected_output = format_rs_source(include_str!("../test_outputs/invokables.rs"));
        assert_str_eq!(rust, expected_output);
    }

    #[test]
    fn generates_naming_rust() {
        let parser =
            Parser::from(syn::parse_str(include_str!("../test_inputs/naming.rs")).unwrap())
                .unwrap();
        let generated = GeneratedRustBlocks::from(&parser).unwrap();
        let rust = format_rs_source(&write_rust(&generated).to_string());

        let expected_output = format_rs_source(include_str!("../test_outputs/naming.rs"));
        assert_str_eq!(rust, expected_output);
    }

    #[test]
    fn generates_passthrough_rust() {
        let parser =
            Parser::from(syn::parse_str(include_str!("../test_inputs/passthrough.rs")).unwrap())
                .unwrap();
        let generated = GeneratedRustBlocks::from(&parser).unwrap();
        let rust = format_rs_source(&write_rust(&generated).to_string());

        let expected_output = format_rs_source(include_str!("../test_outputs/passthrough.rs"));
        assert_str_eq!(rust, expected_output);
    }

    #[test]
    fn generates_properties_rust() {
        let parser =
            Parser::from(syn::parse_str(include_str!("../test_inputs/properties.rs")).unwrap())
                .unwrap();
        let generated = GeneratedRustBlocks::from(&parser).unwrap();
        let rust = format_rs_source(&write_rust(&generated).to_string());

        let expected_output = format_rs_source(include_str!("../test_outputs/properties.rs"));
        assert_str_eq!(rust, expected_output);
    }

    #[test]
    fn generates_signals_rust() {
        let parser =
            Parser::from(syn::parse_str(include_str!("../test_inputs/signals.rs")).unwrap())
                .unwrap();
        let generated = GeneratedRustBlocks::from(&parser).unwrap();
        let rust = format_rs_source(&write_rust(&generated).to_string());

        let expected_output = format_rs_source(include_str!("../test_outputs/signals.rs"));
        assert_str_eq!(rust, expected_output);
    }

    #[test]
    fn generates_types_primitive_property_rust() {
        let parser = Parser::from(
            syn::parse_str(include_str!("../test_inputs/types_primitive_property.rs")).unwrap(),
        )
        .unwrap();
        let generated = GeneratedRustBlocks::from(&parser).unwrap();
        let rust = format_rs_source(&write_rust(&generated).to_string());

        let expected_output =
            format_rs_source(include_str!("../test_outputs/types_primitive_property.rs"));
        assert_str_eq!(rust, expected_output);
    }

    #[test]
    fn generates_types_qt_property_rust() {
        let parser = Parser::from(
            syn::parse_str(include_str!("../test_inputs/types_qt_property.rs")).unwrap(),
        )
        .unwrap();
        let generated = GeneratedRustBlocks::from(&parser).unwrap();
        let rust = format_rs_source(&write_rust(&generated).to_string());

        let expected_output =
            format_rs_source(include_str!("../test_outputs/types_qt_property.rs"));
        assert_str_eq!(rust, expected_output);
    }

    #[test]
    fn generates_types_qt_invokable_rust() {
        let parser = Parser::from(
            syn::parse_str(include_str!("../test_inputs/types_qt_invokable.rs")).unwrap(),
        )
        .unwrap();
        let generated = GeneratedRustBlocks::from(&parser).unwrap();
        let rust = format_rs_source(&write_rust(&generated).to_string());

        let expected_output =
            format_rs_source(include_str!("../test_outputs/types_qt_invokable.rs"));
        assert_str_eq!(rust, expected_output);
    }
}
