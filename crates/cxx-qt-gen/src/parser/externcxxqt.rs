// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    parser::signals::ParsedSignal,
    syntax::{attribute::attribute_find_path, safety::Safety},
};
use syn::{spanned::Spanned, Error, ForeignItem, ItemForeignMod, Result, Token};

use crate::parser::check_attribute_validity;
use crate::syntax::expr::expr_to_string;
#[cfg(test)]
use syn::ForeignItemType;

/// Representation of an extern "C++Qt" block
#[derive(Default)]
pub struct ParsedExternCxxQt {
    /// The namespace of the type in C++.
    pub namespace: Option<String>,
    /// Whether this block has an unsafe token
    pub unsafety: Option<Token![unsafe]>,
    /// Items which can be passed into the extern "C++Qt" block
    pub passthrough_items: Vec<ForeignItem>,
    /// Signals that need generation in the extern "C++Qt" block
    pub signals: Vec<ParsedSignal>,
}

impl ParsedExternCxxQt {
    const ALLOWED_ATTRS: [&'static str; 1] = ["namespace"];

    pub fn parse(mut foreign_mod: ItemForeignMod) -> Result<Self> {
        check_attribute_validity(&foreign_mod.attrs, &Self::ALLOWED_ATTRS)?;

        let namespace = if let Some(index) = attribute_find_path(&foreign_mod.attrs, &["namespace"])
        {
            Some(expr_to_string(
                &foreign_mod.attrs[index].meta.require_name_value()?.value,
            )?)
        } else {
            None
        };

        let mut extern_cxx_block = ParsedExternCxxQt {
            namespace,
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
                            "Types in extern \"C++Qt\" blocks must be tagged with #[qobject]!, use an extern \"C++\" block for non QObject types",
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

    #[cfg(test)]
    fn get_passthrough_foreign_type(&self) -> Result<ForeignItemType> {
        if let ForeignItem::Type(foreign_ty) = &self.passthrough_items[0] {
            Ok(foreign_ty.clone())
        } else {
            Err(Error::new_spanned(
                &self.passthrough_items[0],
                "Item should be `ForeignItem::Type`!",
            ))
        }
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

        assert!(extern_cxx_qt.namespace.is_some());
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

        assert!(extern_cxx_qt.namespace.is_none());
        assert_eq!(extern_cxx_qt.passthrough_items.len(), 1);
        // Check that the attribute is removed
        let foreign_ty = extern_cxx_qt.get_passthrough_foreign_type();
        assert!(foreign_ty.unwrap().attrs.is_empty());

        assert_eq!(extern_cxx_qt.signals.len(), 0);
        assert!(extern_cxx_qt.unsafety.is_none());
    }

    #[test]
    fn test_extern_cxxqt_type_non_type() {
        let extern_cxx_qt = ParsedExternCxxQt::parse(parse_quote! {
            extern "C++Qt" {
                fn myFunction();
            }
        })
        .unwrap();
        // Check that the non Type object is detected and error
        let foreign_ty = extern_cxx_qt.get_passthrough_foreign_type();
        assert!(foreign_ty.is_err());

        assert_eq!(extern_cxx_qt.signals.len(), 0);
        assert!(extern_cxx_qt.unsafety.is_none());
    }
}
