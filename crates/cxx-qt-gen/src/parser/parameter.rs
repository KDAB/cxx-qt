// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::types;
use syn::{
    spanned::Spanned, Error, FnArg, Ident, Pat, PatIdent, PatType, Receiver, Result, Signature,
    Type,
};

/// Describes a single parameter for a function
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedFunctionParameter {
    /// The [syn::Ident] of the parameter
    pub ident: Ident,
    /// The [syn::Type] of the parameter
    pub ty: Type,
}

impl ParsedFunctionParameter {
    fn parse_remaining<'a>(
        iter: impl Iterator<Item = &'a FnArg>,
    ) -> Result<Vec<ParsedFunctionParameter>> {
        iter.map(|input| {
            match input {
                FnArg::Typed(type_pattern) => {
                    let parameter = ParsedFunctionParameter::parse(type_pattern)?;

                    Ok(Some(parameter))
                }
                // Ignore self as a parameter
                FnArg::Receiver(_) => Ok(None),
            }
        })
        .filter_map(|result| result.map_or_else(|e| Some(Err(e)), |v| v.map(Ok)))
        .collect::<Result<Vec<ParsedFunctionParameter>>>()
    }

    pub fn parse_all_ignoring_receiver(
        signature: &Signature,
    ) -> Result<Vec<ParsedFunctionParameter>> {
        let mut iter = signature.inputs.iter();
        // whilst we can ignore the receiver argument, make sure it actually exists
        if iter.next().is_none() {
            return Err(Error::new_spanned(signature, "Missing receiver argument!"));
        }

        Self::parse_remaining(iter)
    }

    /// This function parses the list of arguments
    pub fn parse_all_without_receiver(
        signature: &Signature,
    ) -> Result<Vec<ParsedFunctionParameter>> {
        let mut iter = signature.inputs.iter();

        let missing_self_arg = "First argument must be a supported `self` receiver type!\nUse `&self` or `self: Pin<&mut Self>` instead.";
        match iter.next() {
            Some(FnArg::Receiver(Receiver {
                reference: None,
                ty,
                ..
            })) if types::is_pin_of_self(ty) => Ok(()), // Okay, found a Pin<&Self> or Pin<&mut Self>
            Some(FnArg::Receiver(Receiver {
                reference: Some(_), // `self` needs to be by-ref, by-value is not supported.
                mutability: None,   // Mutable `&mut self` references are not supported.
                                    // Use `Pin<&mut  Self>` instead.
                ..
            })) => Ok(()), // Okay, found a &self reference
            Some(arg) => Err(Error::new_spanned(arg, missing_self_arg)),
            None => Err(Error::new_spanned(signature, "Missing 'self' receiver!")),
        }?;

        Self::parse_remaining(iter)
    }

    pub fn parse(type_pattern: &PatType) -> Result<Self> {
        let ident = if let Pat::Ident(PatIdent { ident, .. }) = &*type_pattern.pat {
            ident.clone()
        } else {
            return Err(Error::new(
                type_pattern.span(),
                "Invalid argument ident format.",
            ));
        };

        Ok(ParsedFunctionParameter {
            ident,
            ty: (*type_pattern.ty).clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::{parse_quote, ForeignItemFn};

    use super::*;

    #[test]
    fn test_parse_remove_receiver() {
        let function: ForeignItemFn = syn::parse_quote! {
            fn foo(self: Pin<&mut Self>);
        };

        let parameters =
            ParsedFunctionParameter::parse_remaining(function.sig.inputs.iter()).unwrap();
        assert_eq!(parameters.len(), 0)
    }

    #[test]
    fn test_parse_non_ident_type_pat() {
        let type_pattern: PatType = parse_quote!( (a, b): (i32, i32) );
        assert!(ParsedFunctionParameter::parse(&type_pattern).is_err())
    }

    #[test]
    fn test_parse_all_without_receiver() {
        let function: ForeignItemFn = syn::parse_quote! {
            fn foo(&self, a: i32, b: String);
        };

        let parameters =
            ParsedFunctionParameter::parse_all_without_receiver(&function.sig).unwrap();
        assert_eq!(parameters.len(), 2);
        assert_eq!(parameters[0].ident, "a");
        assert_eq!(parameters[0].ty.to_token_stream().to_string(), "i32");
        assert_eq!(parameters[1].ident, "b");
        assert_eq!(parameters[1].ty.to_token_stream().to_string(), "String");
    }

    #[test]
    fn test_parse_all_without_receiver_invalid_self() {
        fn assert_parse_error(function: ForeignItemFn) {
            assert!(ParsedFunctionParameter::parse_all_without_receiver(&function.sig).is_err());
        }
        // Missing self
        assert_parse_error(syn::parse_quote! {
            fn foo(a: i32, b: String);
        });
        // self parameter points to non-self type
        assert_parse_error(syn::parse_quote! {
            fn foo(self: T);
        });
        // self parameter is a non-self pin
        assert_parse_error(syn::parse_quote! {
            fn foo(self: Pin<&mut T>);
        })
    }

    #[test]
    fn test_parse_all_ignoring_receiver() {
        // This supports using a type as `self` that's not "Self".
        let function: ForeignItemFn = syn::parse_quote! {
            fn foo(self: T, a: i32, b: String);
        };

        let parameters =
            ParsedFunctionParameter::parse_all_ignoring_receiver(&function.sig).unwrap();
        assert_eq!(parameters.len(), 2);
        assert_eq!(parameters[0].ident, "a");
        assert_eq!(parameters[0].ty.to_token_stream().to_string(), "i32");
        assert_eq!(parameters[1].ident, "b");
        assert_eq!(parameters[1].ty.to_token_stream().to_string(), "String");
    }

    #[test]
    fn test_parse_all_ignoring_receiver_invalid() {
        // missing receiver type
        let function: ForeignItemFn = syn::parse_quote! {
            fn foo();
        };

        assert!(ParsedFunctionParameter::parse_all_ignoring_receiver(&function.sig).is_err())
    }

    #[test]
    fn test_parse_all_without_receiver_invalid() {
        // missing receiver type
        let function: ForeignItemFn = syn::parse_quote! {
            fn foo();
        };

        assert!(ParsedFunctionParameter::parse_all_without_receiver(&function.sig).is_err())
    }
}
