// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{GenericArgument, PathArguments, Type, TypeReference};

/// Return if the type is unsafe for CXX bridges
pub(crate) fn syn_type_is_cxx_bridge_unsafe(ty: &syn::Type) -> bool {
    match ty {
        Type::Path(ty_path) => {
            ty_path
                .path
                .segments
                .iter()
                .any(|segment| match &segment.arguments {
                    PathArguments::AngleBracketed(angled) => {
                        angled.args.iter().any(|generic| match generic {
                            GenericArgument::Type(ty) => syn_type_is_cxx_bridge_unsafe(ty),
                            _others => false,
                        })
                    }
                    _others => false,
                })
        }
        Type::Ptr(..) => true,
        Type::Reference(TypeReference { elem, .. }) => syn_type_is_cxx_bridge_unsafe(elem),
        _others => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::parse_quote;

    #[test]
    fn test_syn_type_is_cxx_bridge_unsafe_path() {
        assert!(!syn_type_is_cxx_bridge_unsafe(&parse_quote! { i32 }));
    }

    #[test]
    fn test_syn_type_is_cxx_bridge_unsafe_path_template() {
        assert!(!syn_type_is_cxx_bridge_unsafe(
            &parse_quote! { Vector<i32> }
        ));
        assert!(syn_type_is_cxx_bridge_unsafe(
            &parse_quote! { Vector<*mut T> }
        ));
    }

    #[test]
    fn test_syn_type_is_cxx_bridge_unsafe_ptr() {
        assert!(syn_type_is_cxx_bridge_unsafe(&parse_quote! { *mut T }));
    }

    #[test]
    fn test_syn_type_is_cxx_bridge_unsafe_reference() {
        assert!(!syn_type_is_cxx_bridge_unsafe(&parse_quote! { &i32 }));
        assert!(syn_type_is_cxx_bridge_unsafe(&parse_quote! { &*mut T }));
        assert!(!syn_type_is_cxx_bridge_unsafe(
            &parse_quote! { &Vector<i32> }
        ));
        assert!(syn_type_is_cxx_bridge_unsafe(
            &parse_quote! { &Vector<*mut T> }
        ));
    }
}
