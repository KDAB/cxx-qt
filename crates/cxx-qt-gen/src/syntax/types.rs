// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::path::*;
use syn::{GenericArgument, PathArguments, Type, TypePath, TypeReference};

/// Checks if the given type is a `Pin<&Self>` or `Pin<&mut Self>`.
/// `Pin<Box<Self>>` is currently not supported.
pub fn is_pin_of_self(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if path_compare_str(&type_path.path, &["Pin"]) {
            if let PathArguments::AngleBracketed(angles) =
                &type_path.path.segments.first().unwrap().arguments
            {
                if let [GenericArgument::Type(Type::Reference(TypeReference {
                    elem: type_elem,
                    ..
                }))] = *angles.args.iter().collect::<Vec<_>>()
                {
                    if let Type::Path(TypePath {
                        path: self_path, ..
                    }) = &**type_elem
                    {
                        if path_compare_str(self_path, &["Self"]) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {

    use crate::tests::tokens_to_syn;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::Type;

    fn assert_pin_of_self(tokens: TokenStream) {
        let pin: Type = tokens_to_syn(tokens);
        assert!(super::is_pin_of_self(&pin));
    }

    fn assert_not_pin_of_self(tokens: TokenStream) {
        let pin: Type = tokens_to_syn(tokens);
        assert!(!super::is_pin_of_self(&pin));
    }

    #[test]
    fn test_is_pin_of_self() {
        assert_pin_of_self(quote! { Pin<&Self> });
        assert_pin_of_self(quote! { Pin<&mut Self> });

        // `Pin<Box<Self>>` is currently not supported because it can't be used with
        // Opaque C++ types. Use UniquePtr<Self> instead.
        assert_not_pin_of_self(quote! { Pin<Box<Self>> });
        assert_not_pin_of_self(quote! { Pin<&Self, Foo> });
        assert_not_pin_of_self(quote! { Pin });
        assert_not_pin_of_self(quote! { Pin<Self> });
        assert_not_pin_of_self(quote! { Pin<&Foo> });
        assert_not_pin_of_self(quote! { Pin<&mut Foo> });
    }
}
