// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{spanned::Spanned, Error, FnArg, Ident, Pat, PatIdent, PatType, Result, Signature, Type};

/// Describes a single parameter for a function
pub struct ParsedFunctionParameter {
    /// The [syn::Ident] of the parameter
    pub ident: Ident,
    /// The [syn::Type] of the parameter
    pub ty: Type,
    /// The name of the C++ type if one has been specified
    pub cxx_type: Option<String>,
}

impl ParsedFunctionParameter {
    pub fn parse_all_without_receiver(
        signature: &Signature,
    ) -> Result<Vec<ParsedFunctionParameter>> {
        signature
            .inputs
            .iter()
            .map(|input| {
                match input {
                    FnArg::Typed(type_pattern) => {
                        let parameter = ParsedFunctionParameter::parse(type_pattern)?;

                        // Ignore self as a parameter
                        if parameter.ident == "self" {
                            return Ok(None);
                        }
                        Ok(Some(parameter))
                    }
                    // Ignore self as a parameter
                    FnArg::Receiver(_) => Ok(None),
                }
            })
            .filter_map(|result| result.map_or_else(|e| Some(Err(e)), |v| v.map(Ok)))
            .collect::<Result<Vec<ParsedFunctionParameter>>>()
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
            // TODO: later we might support cxx_type for parameters in invokables
            cxx_type: None,
        })
    }
}
