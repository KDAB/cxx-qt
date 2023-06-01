// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::parameter::ParsedFunctionParameter;
use crate::syntax::attribute::attribute_find_path;
use crate::syntax::expr::expr_to_string;
use crate::syntax::foreignmod;
use crate::syntax::safety::Safety;
use crate::{generator::naming::CombinedIdent, syntax::types};
use quote::format_ident;
use syn::Attribute;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, ForeignItem, ForeignItemFn, Ident, Item, ItemForeignMod, LitStr, Result, Token,
};

/// Used when parsing a syn::Item::Verbatim, that we suspect may be a `#[cxx_qt::qsignals]` block,
/// but we don't yet know whether this is actually the case.
/// This is the case if `#[cxx_qt::qsignals]` is used with `unsafe extern "C++"`.
pub enum MaybeSignalMethods {
    /// We found a `#[cxx_qt::qsignals]` block
    Found(SignalMethods),
    /// `#[cxx_qt::qsignals]` block not found, pass this Item through to outside code!
    PassThrough(Item),
}

impl Parse for MaybeSignalMethods {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.fork();
        if let Ok(attribute) = lookahead.call(Attribute::parse_outer) {
            if attribute_find_path(attribute.as_slice(), &["cxx_qt", "qsignals"]).is_some() {
                input.call(Attribute::parse_outer)?;
                let methods = input.parse::<SignalMethods>()?;
                return Ok(Self::Found(methods));
            }
        }

        Ok(Self::PassThrough(input.parse()?))
    }
}

/// This type is used when parsing the `#[cxx_qt::qsignals]` macro contents into raw ForeignItemFn items
pub struct SignalMethods {
    pub safety: Safety,
    pub base_functions: Vec<ForeignItemFn>,
}

impl Parse for SignalMethods {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut base_functions = Vec::new();

        // Ensure that any attributes on the block have been removed
        //
        // Otherwise parsing of unsafe can fail due to #[doc]
        let attrs = input.call(Attribute::parse_outer)?;
        if !attrs.is_empty() {
            return Err(Error::new(
                attrs.first().span(),
                "Unexpected attribute on #[cxx_qt::inherit] block.",
            ));
        }

        // This looks somewhat counter-intuitive, but if we add `unsafe`
        // to the `extern "C++"` block, the contained functions will be safe to call.
        let safety = if input.peek(Token![unsafe]) {
            Safety::Safe
        } else {
            Safety::Unsafe
        };
        if safety == Safety::Safe {
            input.parse::<Token![unsafe]>()?;
        }

        let extern_block = input.parse::<ItemForeignMod>()?;
        if extern_block.abi.name != Some(LitStr::new("C++", extern_block.abi.span())) {
            return Err(Error::new(
                extern_block.abi.span(),
                "qsignals blocks must be marked with `extern \"C++\"`",
            ));
        }

        for item in extern_block.items {
            match item {
                ForeignItem::Fn(function) => {
                    base_functions.push(function);
                }
                _ => {
                    return Err(Error::new(
                        item.span(),
                        "Only functions are allowed in #[cxx_qt::qsignals] blocks",
                    ))
                }
            }
        }

        Ok(SignalMethods {
            safety,
            base_functions,
        })
    }
}

/// Describes an individual Signal
pub struct ParsedSignal {
    /// The original [syn::ForeignItemFn] of the signal declaration
    pub method: ForeignItemFn,
    /// The type of the self argument
    pub qobject_ident: Ident,
    /// whether the signal is marked as mutable
    pub mutable: bool,
    /// Whether the method is safe to call.
    pub safe: bool,
    /// The parameters of the signal
    pub parameters: Vec<ParsedFunctionParameter>,
    /// The name of the signal
    pub ident: CombinedIdent,
    /// If the signal is defined in the base class
    pub inherit: bool,
}

impl ParsedSignal {
    /// Builds a signal from a given property method
    pub fn from_property_method(
        method: ForeignItemFn,
        ident: CombinedIdent,
        qobject_ident: Ident,
    ) -> Self {
        Self {
            method,
            qobject_ident,
            mutable: true,
            safe: true,
            parameters: vec![],
            ident,
            inherit: false,
        }
    }

    pub fn parse(mut method: ForeignItemFn, safety: Safety) -> Result<Self> {
        if safety == Safety::Unsafe && method.sig.unsafety.is_none() {
            return Err(Error::new(
                method.span(),
                "qsignals methods must be marked as unsafe or wrapped in an `unsafe extern \"C++\"` block!",
            ));
        }

        let mut inherit = false;

        let self_receiver = foreignmod::self_type_from_foreign_fn(&method.sig)?;
        let (qobject_ident, mutability) = types::extract_qobject_ident(&self_receiver.ty)?;
        let mutable = mutability.is_some();
        if !mutable {
            return Err(Error::new(
                method.span(),
                "signals must be mutable, use Pin<&mut T> instead of T for the self type",
            ));
        }

        let parameters = ParsedFunctionParameter::parse_all_ignoring_receiver(&method.sig)?;

        let mut ident = CombinedIdent::from_rust_function(method.sig.ident.clone());

        if let Some(index) = attribute_find_path(&method.attrs, &["cxx_name"]) {
            ident.cpp = format_ident!(
                "{}",
                expr_to_string(&method.attrs[index].meta.require_name_value()?.value)?
            );

            method.attrs.remove(index);
        }

        if let Some(index) = attribute_find_path(&method.attrs, &["inherit"]) {
            inherit = true;

            method.attrs.remove(index);
        }

        let safe = method.sig.unsafety.is_none();

        Ok(Self {
            method,
            qobject_ident,
            mutable,
            parameters,
            ident,
            safe,
            inherit,
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    use crate::parser::tests::f64_type;

    #[test]
    fn test_parse_signal() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(self: Pin<&mut qobject::MyObject>);
        };
        let signal = ParsedSignal::parse(method.clone(), Safety::Safe).unwrap();
        assert_eq!(signal.method, method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters, vec![]);
        assert_eq!(
            signal.ident,
            CombinedIdent {
                cpp: format_ident!("ready"),
                rust: format_ident!("ready")
            }
        );
        assert!(signal.safe);
        assert!(!signal.inherit);
    }

    #[test]
    fn test_parse_signal_cxx_name() {
        let method: ForeignItemFn = parse_quote! {
            #[cxx_name = "cppReady"]
            fn ready(self: Pin<&mut qobject::MyObject>);
        };
        let signal = ParsedSignal::parse(method, Safety::Safe).unwrap();

        let expected_method: ForeignItemFn = parse_quote! {
            fn ready(self: Pin<&mut qobject::MyObject>);
        };
        assert_eq!(signal.method, expected_method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters, vec![]);
        assert_eq!(
            signal.ident,
            CombinedIdent {
                cpp: format_ident!("cppReady"),
                rust: format_ident!("ready")
            }
        );
        assert!(signal.safe);
        assert!(!signal.inherit);
    }

    #[test]
    fn test_parse_signal_inherit() {
        let method: ForeignItemFn = parse_quote! {
            #[inherit]
            fn ready(self: Pin<&mut qobject::MyObject>);
        };
        let signal = ParsedSignal::parse(method, Safety::Safe).unwrap();

        let expected_method: ForeignItemFn = parse_quote! {
            fn ready(self: Pin<&mut qobject::MyObject>);
        };
        assert_eq!(signal.method, expected_method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters, vec![]);
        assert_eq!(
            signal.ident,
            CombinedIdent {
                cpp: format_ident!("ready"),
                rust: format_ident!("ready")
            }
        );
        assert!(signal.safe);
        assert!(signal.inherit);
    }

    #[test]
    fn test_parse_signal_mutable_err() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(self: &qobject::MyObject);
        };
        // Can't be immutable
        assert!(ParsedSignal::parse(method, Safety::Safe).is_err());
    }

    #[test]
    fn test_parse_signal_parameters() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(self: Pin<&mut qobject::MyObject>, x: f64, y: f64);
        };
        let signal = ParsedSignal::parse(method.clone(), Safety::Safe).unwrap();
        assert_eq!(signal.method, method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters.len(), 2);
        assert_eq!(signal.parameters[0].ident, format_ident!("x"));
        assert_eq!(signal.parameters[0].ty, f64_type());
        assert_eq!(signal.parameters[1].ident, format_ident!("y"));
        assert_eq!(signal.parameters[1].ty, f64_type());
        assert_eq!(
            signal.ident,
            CombinedIdent {
                cpp: format_ident!("ready"),
                rust: format_ident!("ready")
            }
        );
        assert!(signal.safe);
        assert!(!signal.inherit);
    }

    #[test]
    fn test_parse_signal_qobject_self_missing() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(x: f64);
        };
        // Can't have a missing self
        assert!(ParsedSignal::parse(method, Safety::Safe).is_err());
    }

    #[test]
    fn test_parse_signal_qobject_ident_missing() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(&self);
        };
        // Can't have a missing ident
        assert!(ParsedSignal::parse(method, Safety::Safe).is_err());
    }

    #[test]
    fn test_parse_signal_unsafe() {
        let method: ForeignItemFn = parse_quote! {
            unsafe fn ready(self: Pin<&mut qobject::MyObject>);
        };
        let signal = ParsedSignal::parse(method.clone(), Safety::Unsafe).unwrap();
        assert_eq!(signal.method, method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters, vec![]);
        assert_eq!(
            signal.ident,
            CombinedIdent {
                cpp: format_ident!("ready"),
                rust: format_ident!("ready")
            }
        );
        assert!(!signal.safe);
        assert!(!signal.inherit);
    }

    #[test]
    fn test_parse_signal_unsafe_error() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(self: Pin<&mut qobject::MyObject>);
        };
        // Can't be safe on the block and the method
        assert!(ParsedSignal::parse(method, Safety::Unsafe).is_err());
    }
}
