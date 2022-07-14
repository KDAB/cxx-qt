// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::token::Brace;
use syn::{braced, Abi, AttrStyle, Attribute, Item, Result};

#[derive(Clone, PartialEq)]
pub struct CxxQtItemForeignQtMod {
    pub attrs: Vec<Attribute>,
    pub abi: Abi,
    pub brace_token: Brace,
    pub items: Vec<Item>,
}

impl std::fmt::Debug for CxxQtItemForeignQtMod {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut formatter = formatter.debug_struct("CxxQtItemForeignQtMod");
        formatter.field("attrs", &self.attrs);
        formatter.field("abi", &self.abi);
        formatter.field("brace_token", &self.brace_token);
        formatter.field("items", &self.items);
        formatter.finish()
    }
}

impl Parse for CxxQtItemForeignQtMod {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;
        let abi: Abi = input.parse()?;

        let content;
        let brace_token = braced!(content in input);
        attrs.extend(syn::Attribute::parse_inner(&content)?);
        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse()?);
        }

        Ok(CxxQtItemForeignQtMod {
            attrs,
            abi,
            brace_token,
            items,
        })
    }
}

impl ToTokens for CxxQtItemForeignQtMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(
            self.attrs
                .iter()
                .filter(|attr| matches!(attr.style, AttrStyle::Outer)),
        );
        self.abi.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            tokens.append_all(
                self.attrs
                    .iter()
                    .filter(|attr| matches!(attr.style, AttrStyle::Inner(_))),
            );
            tokens.append_all(&self.items);
        });
    }
}
