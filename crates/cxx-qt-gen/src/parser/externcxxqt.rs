// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    parser::{
        externqobject::ParsedExternQObject, require_attributes, signals::ParsedSignal,
        CaseConversion,
    },
    syntax::{attribute::attribute_get_path, expr::expr_to_string, safety::Safety},
};
use convert_case::Case;
use syn::{spanned::Spanned, Error, ForeignItem, Ident, ItemForeignMod, Result, Token};

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
    /// QObject types that need generation in the extern "C++Qt" block
    pub qobjects: Vec<ParsedExternQObject>,
}

impl ParsedExternCxxQt {
    pub fn parse(
        mut foreign_mod: ItemForeignMod,
        module_ident: &Ident,
        parent_namespace: Option<&str>,
    ) -> Result<Self> {
        let attrs = require_attributes(&foreign_mod.attrs, &["namespace", "auto_case"])?;

        let auto_case = match attrs.get("auto_case") {
            Some(_) => CaseConversion::new(None, Some(Case::Snake)), // For extern C++ and C++Qt blocks, we want to convert to snake
            _ => CaseConversion::none(),
        };

        let namespace = attrs
            .get("namespace")
            .map(|attr| -> Result<String> {
                expr_to_string(&attr.meta.require_name_value()?.value)
            })
            .transpose()?;

        let mut extern_cxx_block = ParsedExternCxxQt {
            namespace: namespace.clone(),
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
                ForeignItem::Fn(foreign_fn) => {
                    // Test if the function is a signal
                    if attribute_get_path(&foreign_fn.attrs, &["qsignal"]).is_some() {
                        let mut signal = ParsedSignal::parse(foreign_fn, safe_call, auto_case)?;
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
                ForeignItem::Type(foreign_ty) => {
                    // Test that there is a #[qobject] attribute on any type
                    if attribute_get_path(&foreign_ty.attrs, &["qobject"]).is_some() {
                        let extern_ty =
                            ParsedExternQObject::parse(foreign_ty, module_ident, parent_namespace)?;
                        // Pass through types separately for generation
                        extern_cxx_block.qobjects.push(extern_ty);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::format_ident;

    use syn::parse_quote;

    #[test]
    fn test_find_and_merge_cxx_qt_item_extern_cxx_qt() {
        let extern_cxx_qt = ParsedExternCxxQt::parse(
            parse_quote! {
                #[namespace = "rust"]
                unsafe extern "C++Qt" {
                    #[qobject]
                    type QPushButton;

                    fn method(self: Pin<&mut QPushButton>);

                    #[qsignal]
                    fn clicked(self: Pin<&mut QPushButton>, checked: bool);
                }
            },
            &format_ident!("qobject"),
            None,
        )
        .unwrap();

        assert!(extern_cxx_qt.namespace.is_some());
        assert_eq!(extern_cxx_qt.passthrough_items.len(), 1);
        assert_eq!(extern_cxx_qt.qobjects.len(), 1);
        assert_eq!(extern_cxx_qt.signals.len(), 1);
        assert!(extern_cxx_qt.unsafety.is_some());
    }

    #[test]
    fn test_extern_cxxqt_type_missing_qobject() {
        let extern_cxx_qt = ParsedExternCxxQt::parse(
            parse_quote! {
                unsafe extern "C++Qt" {
                    type QPushButton;
                }
            },
            &format_ident!("qobject"),
            None,
        );
        assert!(extern_cxx_qt.is_err());
    }

    #[test]
    fn test_extern_cxxqt_type_qobject_attr() {
        let extern_cxx_qt = ParsedExternCxxQt::parse(
            parse_quote! {
                extern "C++Qt" {
                    #[qobject]
                    type QPushButton;
                }
            },
            &format_ident!("qobject"),
            None,
        )
        .unwrap();

        assert!(extern_cxx_qt.namespace.is_none());
        assert_eq!(extern_cxx_qt.qobjects.len(), 1);

        assert_eq!(extern_cxx_qt.signals.len(), 0);
        assert!(extern_cxx_qt.unsafety.is_none());
    }

    #[test]
    fn test_extern_cxxqt_type_non_type() {
        let extern_cxx_qt = ParsedExternCxxQt::parse(
            parse_quote! {
                extern "C++Qt" {
                    fn myFunction();
                }
            },
            &format_ident!("qobject"),
            None,
        )
        .unwrap();
        // Check that the non Type object is detected and error
        assert!(extern_cxx_qt.qobjects.is_empty());
        assert!(extern_cxx_qt.signals.is_empty());
        assert!(extern_cxx_qt.unsafety.is_none());
    }
}
