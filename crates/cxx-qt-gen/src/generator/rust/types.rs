// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{GenericArgument, PathArguments, Type, TypeReference};

/// Return if the type is unsafe for CXX bridges
pub fn is_unsafe_cxx_type(ty: &Type) -> bool {
    match ty {
        Type::Path(ty_path) => {
            ty_path
                .path
                .segments
                .iter()
                .any(|segment| match &segment.arguments {
                    PathArguments::AngleBracketed(angled) => {
                        angled.args.iter().any(|generic| match generic {
                            GenericArgument::Type(ty) => is_unsafe_cxx_type(ty),
                            _others => false,
                        })
                    }
                    _others => false,
                })
        }
        Type::Ptr(..) => true,
        Type::Reference(TypeReference { elem, .. }) => is_unsafe_cxx_type(elem),
        _others => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;

    #[test]
    fn test_is_unsafe_cxx_type_path() {
        assert!(!is_unsafe_cxx_type(&tokens_to_syn(quote! { i32 })));
    }

    #[test]
    fn test_is_unsafe_cxx_type_path_template() {
        assert!(!is_unsafe_cxx_type(&tokens_to_syn(quote! { Vector<i32> })));
        assert!(is_unsafe_cxx_type(&tokens_to_syn(
            quote! { Vector<*mut T> }
        )));
    }

    #[test]
    fn test_is_unsafe_cxx_type_ptr() {
        assert!(is_unsafe_cxx_type(&tokens_to_syn(quote! { *mut T })));
    }

    #[test]
    fn test_is_unsafe_cxx_type_reference() {
        assert!(!is_unsafe_cxx_type(&tokens_to_syn(quote! { &i32 })));
        assert!(is_unsafe_cxx_type(&tokens_to_syn(quote! { &*mut T })));
        assert!(!is_unsafe_cxx_type(&tokens_to_syn(quote! { &Vector<i32> })));
        assert!(is_unsafe_cxx_type(&tokens_to_syn(
            quote! { &Vector<*mut T> }
        )));
    }
}
