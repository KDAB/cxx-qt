// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::naming::CombinedIdent,
    parser::parameter::ParsedFunctionParameter,
    syntax::{attribute::attribute_tokens_to_value, implitemmethod::is_method_mutable},
};
use quote::format_ident;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, ForeignItem, ForeignItemFn, Ident, ItemForeignMod, LitStr, Result, Token,
};

/// This type is used when parsing the `cxx_qt::inherit!` macro contents into raw ForeignItemFn items
pub struct InheritMethods {
    pub base_safe_functions: Vec<ForeignItemFn>,
    pub base_unsafe_functions: Vec<ForeignItemFn>,
}

impl Parse for InheritMethods {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut base_safe_functions = Vec::new();
        let mut base_unsafe_functions = Vec::new();

        while !input.is_empty() {
            // base_safe_functions.push(input.parse::<ForeignItemFn>()?);
            // This looks somewhat counter-intuitive, but if we add `unsafe`
            // to the `extern "C++"` block, the contained functions will be safe to call.
            let is_safe = input.peek(Token![unsafe]);
            if is_safe {
                input.parse::<Token![unsafe]>()?;
            }

            let extern_block = input.parse::<ItemForeignMod>()?;
            if extern_block.abi.name != Some(LitStr::new("C++", extern_block.abi.span())) {
                return Err(Error::new(
                    extern_block.abi.span(),
                    "Inherit blocks must be marked with `extern \"C++\"`",
                ));
            }

            for item in extern_block.items {
                match item {
                    ForeignItem::Fn(function) => {
                        if is_safe {
                            base_safe_functions.push(function);
                        } else {
                            base_unsafe_functions.push(function);
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            item.span(),
                            "Only functions are allowed in cxx_qt::inherit! blocks",
                        ))
                    }
                }
            }
        }
        Ok(InheritMethods {
            base_safe_functions,
            base_unsafe_functions,
        })
    }
}

/// Describes a method found in cxx_qt::inherit!
pub struct ParsedInheritedMethod {
    /// The original [syn::ForeignItemFn] of the inherited method declaration
    pub method: ForeignItemFn,
    /// whether the inherited method is marked as mutable
    pub mutable: bool,
    /// Whether the method is safe to call.
    pub safe: bool,
    /// the parameters of the method, without the `self` argument
    pub parameters: Vec<ParsedFunctionParameter>,
    /// the name of the function in Rust, as well as C++
    pub ident: CombinedIdent,
    /// the name of the wrapper function in C++
    pub wrapper_ident: Ident,
}

impl ParsedInheritedMethod {
    pub fn parse_unsafe(method: ForeignItemFn) -> Result<Self> {
        if method.sig.unsafety.is_none() {
            return Err(Error::new(
                method.span(),
                "Inherited methods must be marked as unsafe or wrapped in an `unsafe extern \"C++\"` block!",
            ));
        }
        Self::parse_safe(method)
    }

    pub fn parse_safe(method: ForeignItemFn) -> Result<Self> {
        let mutable = is_method_mutable(&method.sig);

        let parameters = ParsedFunctionParameter::parse_all_without_receiver(&method.sig)?;

        let mut ident = CombinedIdent::from_rust_function(method.sig.ident.clone());
        for attribute in &method.attrs {
            if !attribute.path.is_ident(&format_ident!("cxx_name")) {
                return Err(Error::new(
                    attribute.span(),
                    "Unsupported attribute in cxx_qt::inherit!",
                ));
            }

            let name = attribute_tokens_to_value::<LitStr>(attribute)?;

            ident.cpp = format_ident!("{}", name.value());
        }
        let wrapper_ident = format_ident!("{}_cxxqt_inherit", &ident.cpp);
        let safe = method.sig.unsafety.is_none();

        Ok(Self {
            method,
            mutable,
            parameters,
            ident,
            wrapper_ident,
            safe,
        })
    }
}
