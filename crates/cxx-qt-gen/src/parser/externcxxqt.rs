// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    parser::signals::ParsedSignal,
    syntax::{attribute::attribute_find_path, safety::Safety},
};
use syn::{Attribute, ForeignItem, ItemForeignMod, Result, Token};

/// Representation of an extern "C++Qt" block
#[derive(Default)]
pub struct ParsedExternCxxQt {
    /// Attributes for the extern "C++Qt" block
    pub attrs: Vec<Attribute>,
    /// Whether this block has an unsafe token
    pub unsafety: Option<Token![unsafe]>,
    /// Items which can be passed into the extern "C++Qt" block
    pub passthrough_items: Vec<ForeignItem>,
    /// Signals that need generation in the extern "C++Qt" block
    pub signals: Vec<ParsedSignal>,
}

impl ParsedExternCxxQt {
    pub fn parse(mut foreign_mod: ItemForeignMod) -> Result<Self> {
        let mut extern_cxx_block = ParsedExternCxxQt {
            attrs: foreign_mod.attrs.clone(),
            unsafety: foreign_mod.unsafety,
            ..Default::default()
        };

        let safe_call = if foreign_mod.unsafety.is_some() {
            Safety::Safe
        } else {
            Safety::Unsafe
        };

        // Parse any signals, other items are passed through
        for item in foreign_mod.items.drain(..) {
            if let ForeignItem::Fn(foreign_fn) = &item {
                // Test if the function is a signal
                if let Some(index) = attribute_find_path(&foreign_fn.attrs, &["qsignal"]) {
                    let mut foreign_fn = foreign_fn.clone();
                    // Remove the signals attribute
                    foreign_fn.attrs.remove(index);

                    extern_cxx_block
                        .signals
                        .push(ParsedSignal::parse(foreign_fn, safe_call)?);
                    continue;
                }
            }

            extern_cxx_block.passthrough_items.push(item);
        }

        Ok(extern_cxx_block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::parse_quote;

    #[test]
    fn test_find_and_merge_cxx_qt_item_extern_cxx_qt() {
        let extern_cxx_qt = ParsedExternCxxQt::parse(parse_quote! {
            #[namespace = "rust"]
            unsafe extern "C++Qt" {
                type QPushButton;

                #[qsignal]
                fn clicked(self: Pin<&mut QPushButton>, checked: bool);
            }
        })
        .unwrap();

        assert_eq!(extern_cxx_qt.attrs.len(), 1);
        assert_eq!(extern_cxx_qt.passthrough_items.len(), 1);
        assert_eq!(extern_cxx_qt.signals.len(), 1);
        assert!(extern_cxx_qt.unsafety.is_some());
    }
}
