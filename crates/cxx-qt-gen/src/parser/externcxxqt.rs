// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::attribute::{ParsedAttribute, ParsedAttributes};
use crate::{
    parser::{externqobject::ParsedExternQObject, signals::ParsedSignal, CaseConversion},
    syntax::{attribute::attribute_get_path, expr::expr_to_string},
};
use proc_macro2::Span;
use syn::{
    spanned::Spanned, Error, ForeignItem, ForeignItemFn, Ident, ItemForeignMod, Result, Token,
};

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
        // TODO: support cfg on foreign mod blocks
        let attrs = ParsedAttributes::require_attributes(
            foreign_mod.attrs,
            &["namespace", "auto_cxx_name", "auto_rust_name"],
        )?;

        let auto_case = CaseConversion::from_attrs(&attrs)?;

        let namespace = match attrs.require_one("namespace") {
            ParsedAttribute::Single(attr) => {
                expr_to_string(&attr.meta.require_name_value()?.value).ok()
            }
            ParsedAttribute::Absent => None,
            ParsedAttribute::MultipleDisallowed(_) => {
                Err(Error::new(
                    Span::call_site(),
                    "There must be at most one namespace attribute",
                ))? // TODO: ATTR use real span
            }
            _ => {
                // CODECOV_EXCLUDE_START
                unreachable!(
                    "Namepsace is not an allowed duplicate, nor required so this block should be unreachable"
                )
                // CODECOV_EXCLUDE_STOP
            }
        };

        let mut extern_cxx_block = ParsedExternCxxQt {
            namespace,
            unsafety: foreign_mod.unsafety,
            ..Default::default()
        };

        let mut qobjects = vec![];

        // Parse any signals, other items are passed through
        for item in foreign_mod.items.drain(..) {
            match item {
                ForeignItem::Fn(foreign_fn) => {
                    extern_cxx_block.parse_invokable(foreign_fn, auto_case)?;
                }
                ForeignItem::Type(foreign_ty) => {
                    // Test that there is a #[qobject] attribute on any type
                    //
                    // TODO: what happens to any docs here?
                    if attribute_get_path(&foreign_ty.attrs, &["qobject"]).is_some() {
                        let extern_ty =
                            ParsedExternQObject::parse(foreign_ty, module_ident, parent_namespace)?;
                        // Pass through types separately for generation
                        qobjects.push(extern_ty);
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

        extern_cxx_block.qobjects.extend(qobjects);

        Ok(extern_cxx_block)
    }

    fn parse_invokable(
        &mut self,
        foreign_fn: ForeignItemFn,
        auto_case: CaseConversion,
    ) -> Result<()> {
        // We need to check that any safe functions are defined inside an unsafe block
        // as with C++Qt blocks we directly copy the unsafetyness into the generated
        // extern C++ block
        if foreign_fn.sig.unsafety.is_none() && self.unsafety.is_none() {
            return Err(Error::new(foreign_fn.span(), "block must be declared `unsafe extern \"C++Qt\"` if it contains any safe-to-call C++ functions"));
        }

        // Test if the function is a signal
        if attribute_get_path(&foreign_fn.attrs, &["qsignal"]).is_some() {
            if attribute_get_path(&foreign_fn.attrs, &["inherit"]).is_some() {
                return Err(Error::new(foreign_fn.span(), "#[inherit] is not allowed or necessary in extern \"C++Qt\" blocks, as all signals are inherited by default"));
            }
            let mut signal = ParsedSignal::parse(foreign_fn, auto_case)?;
            // extern "C++Qt" signals are always inherit = true
            // as they always exist on an existing QObject
            signal.inherit = true;
            self.signals.push(signal);
        } else {
            self.passthrough_items.push(ForeignItem::Fn(foreign_fn));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::format_ident;

    use crate::tests::assert_parse_errors;
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
                unsafe extern "C++Qt" {
                    fn myFunction();
                }
            },
            &format_ident!("qobject"),
            None,
        )
        .unwrap();
        assert!(extern_cxx_qt.qobjects.is_empty());
        assert!(extern_cxx_qt.signals.is_empty());
        assert!(extern_cxx_qt.unsafety.is_some());
    }

    #[test]
    fn test_parse_invalid() {
        assert_parse_errors!(
            |item| ParsedExternCxxQt::parse(item, &format_ident!("qobject"), None) =>

            // Inherit is not allowed in "C++Qt" blocks
            {
                unsafe extern "C++Qt" {
                    #[qsignal]
                    #[inherit]
                    fn myFunction(self: Pin<&mut MyObject>);
                }
            }

            // "C++Qt" blocks must be unsafe
            {
                extern "C++Qt" {
                    fn myFunction();
                }
            }

            // All types in "C++Qt" blocks must be marked as QObjects
            {
                unsafe extern "C++Qt" {
                    type QPushButton;
                }
            }

            // Duplicate base attr is an error
            {
                extern "C++Qt" {
                    #[base = QPushButton]
                    #[base = QPushButton]
                    #[qobject]
                    type QPushButtonChild;
                }
            }

            // All types in "C++Qt" blocks must be marked as QObjects
            {
                #[namespace = "My namespace"]
                #[namespace = "My other namespace"]
                unsafe extern "C++Qt" {
                    #[qobject]
                    type QPushButton;
                }
            }
        );
    }
}
