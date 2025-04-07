// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::method::MethodFields;
use proc_macro2::Ident;
use std::ops::DerefMut;
use syn::spanned::Spanned;
use syn::Error;

/// Inline any `Self` types in the methods signatures with the Ident of a qobject passed in
///
/// If there are unresolved methods in the list, but inline is false, it will error,
/// as the self inlining is only available if there is exactly one `QObject` in the block,
/// and this indicates that no inlining can be done, but some `Self` types were present.
pub fn try_inline_self_invokables(
    inline: bool,
    type_to_inline: &Option<Ident>,
    invokables: &mut [impl DerefMut<Target = MethodFields>],
) -> syn::Result<()> {
    for method in invokables.iter_mut() {
        if method.self_unresolved {
            if inline {
                if let Some(inline_type) = type_to_inline.clone() {
                    method.qobject_ident = inline_type;
                } else {
                    return Err(Error::new(
                        method.method.span(),
                        "Expected a type to inline, no `qobject` typename was passed!",
                    ));
                }
            } else {
                return Err(Error::new(
                    method.method.span(),
                    "`Self` type can only be inferred if the extern block contains only one `qobject`.",
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::cxxqtdata::ParsedCxxQtData;
    use crate::parser::method::ParsedMethod;
    use crate::tests::assert_parse_errors;
    use quote::format_ident;
    use syn::{parse_quote, Item};

    #[test]
    fn test_self_inlining_ref() {
        let mut parsed_cxxqtdata = ParsedCxxQtData::new(format_ident!("ffi"), None);
        let extern_rust_qt: Item = parse_quote! {
            unsafe extern "RustQt" {
                #[qobject]
                type MyObject = super::T;

                fn my_method(&self);

                #[inherit]
                fn my_inherited_method(&self);
            }
        };

        parsed_cxxqtdata.parse_cxx_qt_item(extern_rust_qt).unwrap();
    }

    #[test]
    fn test_self_inlining_pin() {
        let mut parsed_cxxqtdata = ParsedCxxQtData::new(format_ident!("ffi"), None);
        let extern_rust_qt: Item = parse_quote! {
            unsafe extern "RustQt" {
                #[qobject]
                type MyObject = super::T;

                #[qsignal]
                fn my_signal(self: Pin<&mut Self>);
            }
        };

        let extern_cpp_qt: Item = parse_quote! {
            unsafe extern "C++Qt" {
                #[qobject]
                type MyObject;

                #[qsignal]
                fn my_signal(self: Pin<&mut Self>);
            }
        };

        parsed_cxxqtdata.parse_cxx_qt_item(extern_rust_qt).unwrap();
        parsed_cxxqtdata.parse_cxx_qt_item(extern_cpp_qt).unwrap();
    }

    #[test]
    fn test_self_inlining_methods_invalid() {
        assert_parse_errors! {
            |item| ParsedCxxQtData::new(format_ident!("ffi"), None).parse_cxx_qt_item(item) =>
            // No QObject in block
            {
                extern "RustQt" {
                    fn my_method(&self);
                }
            }

            {
                extern "RustQt" {
                    fn my_method(self: Pin<&mut Self>);
                }
            }
            // More than 1 QObjects in block
            {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::T;

                    #[qobject]
                    type MyOtherObject = super::S;

                    fn my_method(&self);
                }
            }
        }
    }

    #[test]
    fn test_invalid_inline_call() {
        let method_sig = parse_quote! {
            fn test(&self);
        };
        let mut methods = vec![ParsedMethod::mock_qinvokable(&method_sig)];

        // If inlining is set to take place, an Ident is required to inline, here it is `None`
        let data = try_inline_self_invokables(true, &None, &mut methods);
        assert!(data.is_err());
    }
}
