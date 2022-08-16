// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::Ident;

/// For a given module ident generate the file stem
//
// TODO: for now the QObject ident is passed to this
pub fn cxx_stem_from_ident(ident: &Ident) -> Ident {
    format_ident!("{}", ident.to_string().to_case(Case::Snake))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cxx_stem_from_ident() {
        assert_eq!(
            cxx_stem_from_ident(&format_ident!("MyObject")),
            format_ident!("my_object")
        );
    }
}
