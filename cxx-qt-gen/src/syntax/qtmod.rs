// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::CxxQtItem;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Mod, Semi};
use syn::{braced, token, AttrStyle, Attribute, Ident, Result, Token, Visibility};

#[derive(Clone, PartialEq)]
pub struct CxxQtItemMod {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mod_token: Mod,
    pub ident: Ident,
    pub content: Option<(Brace, Vec<CxxQtItem>)>,
    pub semi: Option<Semi>,
}

impl std::fmt::Debug for CxxQtItemMod {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut formatter = formatter.debug_struct("CxxQtItemMod");
        formatter.field("attrs", &self.attrs);
        formatter.field("vis", &self.vis);
        formatter.field("mod_token", &self.mod_token);
        formatter.field("ident", &self.ident);
        formatter.field("content", &self.content);
        formatter.field("semi", &self.semi);
        formatter.finish()
    }
}

impl Parse for CxxQtItemMod {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let mod_token: Token![mod] = input.parse()?;
        let ident: Ident = input.parse()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![;]) {
            Ok(CxxQtItemMod {
                attrs,
                vis,
                mod_token,
                ident,
                content: None,
                semi: Some(input.parse()?),
            })
        } else if lookahead.peek(token::Brace) {
            let content;
            let brace_token = braced!(content in input);
            attrs.extend(syn::Attribute::parse_inner(&content)?);

            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse()?);
            }

            Ok(CxxQtItemMod {
                attrs,
                vis,
                mod_token,
                ident,
                content: Some((brace_token, items)),
                semi: None,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for CxxQtItemMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(
            self.attrs
                .iter()
                .filter(|attr| matches!(attr.style, AttrStyle::Outer)),
        );
        self.vis.to_tokens(tokens);
        self.mod_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        if let Some((brace, items)) = &self.content {
            brace.surround(tokens, |tokens| {
                tokens.append_all(
                    self.attrs
                        .iter()
                        .filter(|attr| matches!(attr.style, AttrStyle::Inner(_))),
                );
                tokens.append_all(items);
            });
        } else {
            // TokensOrDefault is private in syn, so we implement ourselves
            //
            // TokensOrDefault(&self.semi).to_tokens(tokens);
            match &self.semi {
                Some(t) => t.to_tokens(tokens),
                None => Semi::default().to_tokens(tokens),
            }
        }
    }
}
