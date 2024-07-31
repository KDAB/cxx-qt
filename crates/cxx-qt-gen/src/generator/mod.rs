// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::fragment::CppNamedType;
use crate::naming::cpp::syn_type_to_cpp_type;
use crate::naming::TypeNames;
use crate::parser::parameter::ParsedFunctionParameter;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, FnArg, ForeignItemFn, Pat, PatIdent, PatType, Result};

pub mod cpp;
pub mod naming;
pub mod rust;
pub mod structuring;

pub fn get_params_tokens(
    mutable: bool,
    parameters: &[ParsedFunctionParameter],
    class_name: &Ident,
) -> TokenStream {
    let struct_sig = if mutable {
        quote! { Pin<&mut #class_name> }
    } else {
        quote! { &#class_name }
    };
    if parameters.is_empty() {
        quote! { self: #struct_sig }
    } else {
        let parameters = parameters
            .iter()
            .map(|parameter| {
                let ident = &parameter.ident;
                let ty = &parameter.ty;
                quote! { #ident: #ty }
            })
            .collect::<Vec<TokenStream>>();
        quote! { self: #struct_sig, #(#parameters),* }
    }
}

/// Returns a vector of the names and types ([CppNamedType] of the parameters of this method, used in cpp generation step
pub fn get_cpp_params(method: &ForeignItemFn, type_names: &TypeNames) -> Result<Vec<CppNamedType>> {
    method
        .sig
        .inputs
        .iter()
        .map(|input| {
            // Match parameters to extract their idents
            if let FnArg::Typed(PatType { pat, ty, .. }) = input {
                let ident = if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                    ident
                } else {
                    return Err(Error::new(input.span(), "Unknown pattern for type"));
                };

                // If the name of the argument is self then ignore,
                // as this is likely the self: Pin<T>
                if ident == "self" {
                    Ok(None)
                } else {
                    Ok(Some(CppNamedType {
                        ident: ident.to_string(),
                        ty: syn_type_to_cpp_type(ty, type_names)?,
                    }))
                }
            } else {
                Ok(None)
            }
        })
        .filter_map(|result| result.map_or_else(|e| Some(Err(e)), |v| v.map(Ok)))
        .collect()
}
