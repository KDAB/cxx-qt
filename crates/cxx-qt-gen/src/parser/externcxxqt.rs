// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    parser::signals::ParsedSignal,
    syntax::{attribute::attribute_find_path, safety::Safety},
};
use syn::{spanned::Spanned, Attribute, Error, ForeignItem, ItemForeignMod, Result, Token};

/// Representation of an extern "C++Qt" block
#[derive(Default)]
// might need to add a vec of methods, confirm with Leon
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
            match item {
                ForeignItem::Fn(mut foreign_fn) => {
                    // Test if the function is a signal
                    if let Some(index) = attribute_find_path(&foreign_fn.attrs, &["qsignal"]) {
                        // Remove the signals attribute
                        foreign_fn.attrs.remove(index);

                        let mut signal = ParsedSignal::parse(foreign_fn, safe_call)?;
                        // extern "C++Qt" signals are always inherit = true
                        // as they always exist on an existing QObject
                        signal.inherit = true;
                        extern_cxx_block.signals.push(signal);
                    } else {
                        extern_cxx_block
                            .passthrough_items
                            .push(ForeignItem::Fn(foreign_fn));
                    }
                }
                ForeignItem::Type(mut foreign_ty) => {
                    // Test that there is a #[qobject] attribute on any type
                    if let Some(index) = attribute_find_path(&foreign_ty.attrs, &["qobject"]) {
                        // Remove the #[qobject] attribute
                        foreign_ty.attrs.remove(index);

                        // Pass through the item as it's the same
                        extern_cxx_block
                            .passthrough_items
                            .push(ForeignItem::Type(foreign_ty));
                    } else {
                        return Err(Error::new(
                            foreign_ty.span(),
                            "Types in extern \"C++Qt\" blocks must be tagged with #[qobject], use a extern \"C++\" block for non QObject types",
                        ));
                    }
                }
                others => {
                    extern_cxx_block.passthrough_items.push(others);
                }
            }
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
                #[qobject]
                type QPushButton;

                fn method(self: Pin<&mut QPushButton>);

                #[qsignal]
                fn clicked(self: Pin<&mut QPushButton>, checked: bool);
            }
        })
        .unwrap();

        assert_eq!(extern_cxx_qt.attrs.len(), 1);
        assert_eq!(extern_cxx_qt.passthrough_items.len(), 2);
        assert_eq!(extern_cxx_qt.signals.len(), 1);
        assert!(extern_cxx_qt.unsafety.is_some());
    }

    #[test]
    fn test_extern_cxxqt_type_missing_qobject() {
        let extern_cxx_qt = ParsedExternCxxQt::parse(parse_quote! {
            unsafe extern "C++Qt" {
                type QPushButton;
            }
        });
        assert!(extern_cxx_qt.is_err());
    }

    #[test]
    fn test_extern_cxxqt_type_qobject_attr() {
        let extern_cxx_qt = ParsedExternCxxQt::parse(parse_quote! {
            extern "C++Qt" {
                #[qobject]
                type QPushButton;
            }
        })
        .unwrap();

        assert_eq!(extern_cxx_qt.attrs.len(), 0);
        assert_eq!(extern_cxx_qt.passthrough_items.len(), 1);
        // Check that the attribute is removed
        if let ForeignItem::Type(foreign_ty) = &extern_cxx_qt.passthrough_items[0] {
            assert_eq!(foreign_ty.attrs.len(), 0);
        } else {
            panic!("Item should be ForeignItem::Type");
        }
        assert_eq!(extern_cxx_qt.signals.len(), 0);
        assert!(extern_cxx_qt.unsafety.is_none());
    }
}
