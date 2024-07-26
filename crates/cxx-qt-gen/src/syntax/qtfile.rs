// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::CxxQtItem;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::{AttrStyle, Attribute, Result};

/// Representation of a CxxQtFile as Syn items
pub struct CxxQtFile {
    /// A vector of [Attribute] in the file
    pub attrs: Vec<Attribute>,
    /// A vector of [CxxQtItem] items in the file
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

/// Parse the given [std::path::Path] into a [CxxQtFile]
pub fn parse_qt_file(path: impl AsRef<std::path::Path>) -> Result<CxxQtFile> {
    let source = std::fs::read_to_string(path.as_ref()).unwrap_or_else(|err| {
        // todo: fixme with a proper error propagation
        panic!("Failed to read file {:?}: {}", path.as_ref(), err);
    });

    // We drop the shebang from the generated Rust code
    if source.starts_with("#!") && !source.starts_with("#![") {
        let shebang_end = source.find('\n').unwrap_or(source.len());
        syn::parse_str(&source[shebang_end..])
    } else {
        syn::parse_str(&source)
    }
}
