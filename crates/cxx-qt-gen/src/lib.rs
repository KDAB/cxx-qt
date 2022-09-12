// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
mod extract;
mod gen_cpp;
mod gen_rs;
mod generator;
mod parser;
mod syntax;
mod writer;

pub use extract::{extract_qobject, QObject};
pub use gen_cpp::{generate_format, generate_qobject_cpp, CppObject};
pub use gen_rs::{generate_qobject_cxx, generate_qobject_rs};
pub use syntax::{parse_qt_file, CxxQtItem};

#[cfg(test)]
mod tests {
    use super::*;

    use clang_format::{clang_format, ClangFormatStyle};
    use generator::cpp::GeneratedCppBlocks;
    use parser::Parser;
    use pretty_assertions::assert_str_eq;
    use quote::ToTokens;
    use writer::cpp::write_cpp;

    #[ctor::ctor]
    fn init_tests() {
        // Set the ClangFormatStyle to be Mozilla for our tests
        // so that when they fail the format in the assertions is the same as the files.
        assert!(generate_format(Some(ClangFormatStyle::Mozilla)).is_ok());
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
}
