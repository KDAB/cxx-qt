// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{generator::naming::CombinedIdent, parser::invokable::ParsedQInvokable};
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::{Ident, ImplItemMethod};

/// Names for parts of a Q_INVOKABLE
pub struct QInvokableName {
    pub name: CombinedIdent,
    pub wrapper: CombinedIdent,
}

impl From<&ParsedQInvokable> for QInvokableName {
    fn from(invokable: &ParsedQInvokable) -> Self {
        Self::from(&invokable.method)
    }
}

impl From<&ImplItemMethod> for QInvokableName {
    fn from(method: &ImplItemMethod) -> Self {
        let ident = &method.sig.ident;
        Self {
            name: name_from_ident(ident),
            wrapper: wrapper_from_ident(ident),
        }
    }
}

/// For a given ident generate the Rust and C++ names
fn name_from_ident(ident: &Ident) -> CombinedIdent {
    CombinedIdent {
        cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
        rust: ident.clone(),
    }
}

/// For a given ident generate the Rust and C++ wrapper names
fn wrapper_from_ident(ident: &Ident) -> CombinedIdent {
    let ident = format_ident!("{}_wrapper", ident);
    CombinedIdent {
        cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
        rust: ident,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;

    #[test]
    fn test_from_impl_method() {
        let item: ImplItemMethod = tokens_to_syn(quote! {
            fn my_invokable() {

            }
        });
        let parsed = ParsedQInvokable {
            method: item,
            return_cxx_type: None,
        };

        let invokable = QInvokableName::from(&parsed);
        assert_eq!(invokable.name.cpp, format_ident!("myInvokable"));
        assert_eq!(invokable.name.rust, format_ident!("my_invokable"));
        assert_eq!(invokable.wrapper.cpp, format_ident!("myInvokableWrapper"));
        assert_eq!(
            invokable.wrapper.rust,
            format_ident!("my_invokable_wrapper")
        );
    }
}
