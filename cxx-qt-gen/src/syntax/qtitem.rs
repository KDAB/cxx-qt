// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{CxxQtItemForeignQtMod, CxxQtItemMod};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{token, Attribute, Item, LitStr, Result, Token};

#[derive(Clone, PartialEq)]
pub enum CxxQtItem {
    Item(Item),
    ItemQtMod(CxxQtItemMod),
    ForeignQtMod(CxxQtItemForeignQtMod),
}

impl std::fmt::Debug for CxxQtItem {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CxxQtItem::Item(v0) => {
                let mut formatter = formatter.debug_tuple("Item");
                formatter.field(v0);
                formatter.finish()
            }
            CxxQtItem::ItemQtMod(v0) => {
                let mut formatter = formatter.debug_tuple("ItemQtMod");
                formatter.field(v0);
                formatter.finish()
            }
            CxxQtItem::ForeignQtMod(v0) => {
                let mut formatter = formatter.debug_tuple("ForeignQtMod");
                formatter.field(v0);
                formatter.finish()
            }
        }
    }
}

impl Parse for CxxQtItem {
    fn parse(input: ParseStream) -> Result<Self> {
        // Fork and skip over the attributes as we want to read the next token
        let ahead = input.fork();
        ahead.call(Attribute::parse_outer)?;

        println!("parsing CxxQtItem...");

        // See if the next token is an extern
        let lookahead = ahead.lookahead1();
        if lookahead.peek(Token![extern]) {
            println!("found extern block...");
            // Move past the extern
            ahead.parse::<Token![extern]>()?;

            // See if the next token is a literal string
            let lookahead = ahead.lookahead1();
            if lookahead.peek(LitStr) {
                println!("found litstr...");
                // Move past the literal string
                let string = ahead.parse::<LitStr>()?;

                // See if the next token is a brace and that the ABI was "Qt"
                let lookahead = ahead.lookahead1();
                if string.value() == "Qt" && lookahead.peek(token::Brace) {
                    println!("found brace and Qt...");
                    return input.parse().map(CxxQtItem::ForeignQtMod);
                }
            }
        } else if lookahead.peek(Token![mod]) {
            return input.parse().map(CxxQtItem::ItemQtMod);
        }

        // Fallback to using normal Item
        input.parse().map(CxxQtItem::Item)
    }
}

impl ToTokens for CxxQtItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CxxQtItem::Item(item) => {
                item.to_tokens(tokens);
            }
            CxxQtItem::ItemQtMod(module) => {
                module.to_tokens(tokens);
            }
            CxxQtItem::ForeignQtMod(foreign) => {
                foreign.to_tokens(tokens);
            }
        }
    }
}
