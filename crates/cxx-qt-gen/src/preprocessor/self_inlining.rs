// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::cxxqtdata::ParsedCxxQtData;
use crate::parser::inherit::ParsedInheritedMethod;
use crate::parser::method::{MethodFields, ParsedMethod};
use crate::parser::qobject::ParsedQObject;
use crate::parser::signals::ParsedSignal;
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

/// A collection of items found in an extern block when parsing
type BlockComponents<'a> = (
    &'a mut Vec<ParsedQObject>,
    &'a mut Vec<ParsedMethod>,
    &'a mut Vec<ParsedInheritedMethod>,
    &'a mut Vec<ParsedSignal>,
);

// Separates the parsed data by block and returns tuples of the components
fn separate_blocks(data: &mut ParsedCxxQtData) -> Vec<BlockComponents> {
    data.qobjects
        .iter_mut()
        .zip(data.methods.iter_mut())
        .zip(data.inherited_methods.iter_mut())
        .zip(data.signals.iter_mut())
        .map(|(((qobject, method), inherited_method), signal)| {
            (qobject, method, inherited_method, signal)
        })
        .collect()
}

/// For a given parser, attempt to inline the `Self` type used in any of the blocks with that blocks unique QObject
pub fn qualify_self_types(parser: &mut Parser) -> Result<()> {
    for (qobjects, methods, inherited, signals) in separate_blocks(&mut parser.cxx_qt_data) {
        let inline_self = qobjects.len() == 1;
        let inline_ident = qobjects
            .last()
            .map(|obj| format_ident!("{}", obj.declaration.ident_left));

        try_inline_self_invokables(inline_self, &inline_ident, methods)?;
        try_inline_self_invokables(inline_self, &inline_ident, signals)?;
        try_inline_self_invokables(inline_self, &inline_ident, inherited)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::method::ParsedMethod;
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
