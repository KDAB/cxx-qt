// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::CxxQtItem;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::{AttrStyle, Attribute, Result};

pub struct CxxQtFile {
    pub attrs: Vec<Attribute>,
    pub items: Vec<CxxQtItem>,
}

impl Parse for CxxQtFile {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(CxxQtFile {
            attrs: input.call(Attribute::parse_inner)?,
            items: {
                let mut items = Vec::new();
                while !input.is_empty() {
                    items.push(input.parse()?);
                }
                items
            },
        })
    }
}

impl ToTokens for CxxQtFile {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(
            self.attrs
                .iter()
                .filter(|attr| matches!(attr.style, AttrStyle::Inner(_))),
        );
        tokens.append_all(&self.items);
    }
}

pub fn parse_qt_file(path: &impl AsRef<std::path::Path>) -> Result<CxxQtFile> {
    let source = std::fs::read_to_string(path.as_ref()).expect("Could not read path {} to string");

    // We drop the shebang from the generated Rust code
    if source.starts_with("#!") && !source.starts_with("#![") {
        let shebang_end = source.find('\n').unwrap_or(source.len());
        syn::parse_str(&source[shebang_end..])
    } else {
        syn::parse_str(&source)
    }
}
