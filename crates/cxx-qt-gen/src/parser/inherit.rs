// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::naming::CombinedIdent,
    parser::parameter::ParsedFunctionParameter,
    syntax::{
        attribute::{attribute_find_path, attribute_tokens_to_value},
        foreignmod, types,
    },
};
use quote::format_ident;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Attribute, Error, ForeignItem, ForeignItemFn, Ident, Item, ItemForeignMod, LitStr, Result,
    Token,
};

/// Used when parsing a syn::Item::Verbatim, that we suspect may be a `#[cxx_qt::inherit]` block,
/// but we don't yet know whether this is actually the case.
/// This is the case if `#[cxx_qt::inherit]` is used with `unsafe extern "C++"`.
pub enum MaybeInheritMethods {
    /// We found a `#[cxx_qt::inherit]` block
    Found(InheritMethods),
    /// `#[cxx_qt::inherit]` block not found, pass this Item through to outside code!
    PassThrough(Item),
}

impl Parse for MaybeInheritMethods {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.fork();
        if let Ok(attribute) = lookahead.call(Attribute::parse_outer) {
            if attribute_find_path(attribute.as_slice(), &["cxx_qt", "inherit"]).is_some() {
                input.call(Attribute::parse_outer)?;
                let methods = input.parse::<InheritMethods>()?;
                return Ok(Self::Found(methods));
            }
        }

        Ok(Self::PassThrough(input.parse()?))
    }
}

/// This type is used when parsing the `#[cxx_qt::inherit]` macro contents into raw ForeignItemFn items
pub struct InheritMethods {
    pub is_safe: bool,
    pub base_functions: Vec<ForeignItemFn>,
}

impl Parse for InheritMethods {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut base_functions = Vec::new();

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
                    base_functions.push(function);
                }
                _ => {
                    return Err(Error::new(
                        item.span(),
                        "Only functions are allowed in #[cxx_qt::inherit] blocks",
                    ))
                }
            }
        }

        Ok(InheritMethods {
            is_safe,
            base_functions,
        })
    }
}

/// Describes a method found in #[cxx_qt::inherit]
pub struct ParsedInheritedMethod {
    /// The original [syn::ForeignItemFn] of the inherited method declaration
    pub method: ForeignItemFn,
    /// The type of the self argument
    pub qobject_ident: Ident,
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
        let self_receiver = foreignmod::self_type_from_foreign_fn(&method.sig)?;
        let (qobject_ident, mutability) = types::extract_qobject_ident(&self_receiver.typ)?;
        let mutable = mutability.is_some();

        let parameters = ParsedFunctionParameter::parse_all_ignoring_receiver(&method.sig)?;

        let mut ident = CombinedIdent::from_rust_function(method.sig.ident.clone());
        for attribute in &method.attrs {
            if !attribute.path.is_ident(&format_ident!("cxx_name")) {
                return Err(Error::new(
                    attribute.span(),
                    "Unsupported attribute in #[cxx_qt::inherit]",
                ));
            }

            let name = attribute_tokens_to_value::<LitStr>(attribute)?;

            ident.cpp = format_ident!("{}", name.value());
        }
        let wrapper_ident = format_ident!("{}_cxxqt_inherit", &ident.cpp);
        let safe = method.sig.unsafety.is_none();

        Ok(Self {
            method,
            qobject_ident,
            mutable,
            parameters,
            ident,
            wrapper_ident,
            safe,
        })
    }
}
