// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
mod extract;
mod gen_cpp;
mod gen_rs;
mod parser;
mod syntax;
mod utils;

pub use extract::{extract_qobject, QObject};
pub use gen_cpp::{generate_format, generate_qobject_cpp, CppObject};
pub use gen_rs::{generate_qobject_cxx, generate_qobject_rs};
pub use syntax::{parse_qt_file, CxxQtItem};

#[cfg(test)]
mod tests {
    use super::*;

    use clang_format::ClangFormatStyle;
    use quote::ToTokens;

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
}
