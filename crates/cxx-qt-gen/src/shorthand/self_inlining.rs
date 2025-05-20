// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module contains functions which can inline a Self type to the appropriate `QObject`.
//!
//! If ***exactly one*** `QObject` exists in a particular extern block, the self shorthand can be used,
//! and this modules functions will replace this `Self` type with the appropriate name.
//! `qualify_self_types` is called at the highest level, on an already built parser

use crate::parser::method::MethodFields;
use crate::Parser;
use proc_macro2::Ident;
use quote::format_ident;
use std::ops::DerefMut;
use syn::spanned::Spanned;
use syn::{Error, Result};

/// Inline any `Self` types in the methods signatures with the Ident of a qobject passed in
///
/// If there are unresolved methods in the list, but inline is false, it will error,
/// as the self inlining is only available if there is exactly one `QObject` in the block,
/// and this indicates that no inlining can be done, but some `Self` types were present.
pub fn try_inline_self_invokables(
    type_to_inline: &Option<Ident>,
    invokables: &mut [impl DerefMut<Target = MethodFields>],
) -> Result<()> {
    for method in invokables.iter_mut() {
        if method.self_unresolved() {
            if let Some(inline_type) = type_to_inline.clone() {
                method.qobject_ident = inline_type;
            } else {
                return Err(Error::new(
                    method.method.span(),
                    "`Self` type can only be inferred if the extern block contains exactly one `qobject`.",
                ));
            }
        }
    }
    Ok(())
}

/// For a given parser, attempt to inline the `Self` type used in any of the blocks with that blocks unique QObject
pub fn qualify_self_types(parser: &mut Parser) -> Result<()> {
    // Inlining `extern "RustQt"` blocks
    for rust_block in &mut parser.cxx_qt_data.extern_rustqt_blocks {
        let mut iter = rust_block.qobjects.iter();
        let mut inline_ident = iter
            .next()
            .map(|obj| format_ident!("{}", obj.declaration.ident_left));

        if iter.next().is_some() {
            inline_ident = None;
        }

        try_inline_self_invokables(&inline_ident, &mut rust_block.methods)?;
        try_inline_self_invokables(&inline_ident, &mut rust_block.signals)?;
        try_inline_self_invokables(&inline_ident, &mut rust_block.inherited_methods)?;
    }

    // Inlining `extern "C++Qt"` blocks
    for cpp_block in &mut parser.cxx_qt_data.extern_cxxqt_blocks {
        let mut iter = cpp_block.qobjects.iter();
        let mut inline_ident = iter
            .next()
            .map(|obj| format_ident!("{}", obj.declaration.ident));

        if iter.next().is_some() {
            inline_ident = None;
        }

        try_inline_self_invokables(&inline_ident, &mut cpp_block.signals)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_errors;
    use syn::parse_quote;

    #[test]
    fn test_self_inlining_ref() {
        let module = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                unsafe extern "RustQt" {
                    #[qobject]
                    type MyObject = super::T;

                    fn my_method(&self);

                    #[inherit]
                    fn my_inherited_method(&self);
                }
            }
        };
        let mut parser = Parser::from(module).unwrap();
        assert!(qualify_self_types(&mut parser).is_ok());
    }

    #[test]
    fn test_self_inlining_pin() {
        let module = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                unsafe extern "RustQt" {
                    #[qobject]
                    type MyObject = super::T;

                    #[qsignal]
                    fn my_signal(self: Pin<&mut Self>);
                }

                unsafe extern "C++Qt" {
                    #[qobject]
                    type MyOtherObject;

                    #[qsignal]
                    fn my_signal(self: Pin<&mut Self>);
                }
            }
        };
        let mut parser = Parser::from(module).unwrap();
        assert!(qualify_self_types(&mut parser).is_ok());
    }

    #[test]
    fn test_self_inlining_methods_invalid() {
        assert_parse_errors! {
            |item| qualify_self_types(&mut Parser::from(item)?) =>
            // No QObject in block
            {
                #[cxx_qt::bridge]
                mod ffi {
                    extern "RustQt" {
                        fn my_method(&self);
                    }
                }
            }

            {
                #[cxx_qt::bridge]
                mod ffi {
                    extern "RustQt" {
                        fn my_method(self: Pin<&mut Self>);
                    }
                }
            }
            // More than 1 QObjects in block
            {
                #[cxx_qt::bridge]
                mod ffi {
                    unsafe extern "C++Qt" {
                        #[qobject]
                        type MyObject = super::T;

                        #[qobject]
                        type SecondObject = super::S;

                        #[qsignal]
                        fn my_method(self: Pin<&mut Self>);
                    }
                }
            }

            // More than one qobject, in RustQt
            {
                #[cxx_qt::bridge]
                mod ffi {
                    extern "RustQt" {
                        #[qobject]
                        type MyObject = super::T;

                        #[qobject]
                        type SecondObject = super::S;

                        fn my_method(&self);
                    }
                }
            }
        }
    }
}
