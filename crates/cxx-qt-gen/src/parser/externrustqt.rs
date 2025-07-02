// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::naming::cpp::err_unsupported_item;
use crate::parser::inherit::ParsedInheritedMethod;
use crate::parser::method::ParsedMethod;
use crate::parser::qobject::ParsedQObject;
use crate::parser::signals::ParsedSignal;
use crate::parser::{require_attributes, CaseConversion};
use crate::syntax::attribute::attribute_get_path;
use crate::syntax::expr::expr_to_string;
use crate::syntax::foreignmod::ForeignTypeIdentAlias;
use proc_macro2::Ident;
use syn::spanned::Spanned;
use syn::{Error, ForeignItem, ForeignItemFn, ItemForeignMod, Result, Token};

/// Representation of an extern "RustQt" block
#[derive(Default)]
pub struct ParsedExternRustQt {
    /// Whether this block has an unsafe token
    pub unsafety: Option<Token![unsafe]>,
    /// List of QObjects defined in the module
    pub qobjects: Vec<ParsedQObject>,
    /// List of methods and Q_INVOKABLES found
    pub methods: Vec<ParsedMethod>,
    /// List of the Q_SIGNALS found
    pub signals: Vec<ParsedSignal>,
    /// List of the inherited methods found
    pub inherited_methods: Vec<ParsedInheritedMethod>,
}

impl ParsedExternRustQt {
    pub fn parse(
        mut foreign_mod: ItemForeignMod,
        module_ident: &Ident,
        parent_namespace: Option<&str>,
    ) -> Result<Self> {
        // TODO: (cfg everywhere) support cfg on foreign mod blocks
        let (attrs, _common_attrs) = require_attributes(
            &foreign_mod.attrs,
            &["namespace", "auto_cxx_name", "auto_rust_name"],
        )?;

        let auto_case = CaseConversion::from_attrs(&attrs)?;

        let mut extern_rustqt_block = Self {
            unsafety: foreign_mod.unsafety,
            ..Default::default()
        };

        let namespace = attrs
            .get("namespace")
            .map(|attr| expr_to_string(&attr.meta.require_name_value()?.value))
            .transpose()?
            .or_else(|| parent_namespace.map(String::from));

        for item in foreign_mod.items.drain(..) {
            match item {
                ForeignItem::Fn(foreign_fn) => {
                    extern_rustqt_block.parse_invokable(foreign_fn, auto_case)?;
                }
                ForeignItem::Verbatim(tokens) => {
                    let foreign_alias: ForeignTypeIdentAlias = syn::parse2(tokens.clone())?;

                    // Load the QObject
                    let qobject = ParsedQObject::parse(
                        foreign_alias,
                        namespace.as_deref(),
                        module_ident,
                        auto_case,
                    )?;

                    // Note that we assume a compiler error will occur later
                    // if you had two structs with the same name
                    extern_rustqt_block.qobjects.push(qobject);
                }
                // Const, Macro, Type are unsupported in extern "RustQt" for now
                _ => return Err(err_unsupported_item(&item)),
            }
        }

        Ok(extern_rustqt_block)
    }

    fn parse_invokable(
        &mut self,
        foreign_fn: ForeignItemFn,
        auto_case: CaseConversion,
    ) -> Result<()> {
        // Test if the function is a signal
        if attribute_get_path(&foreign_fn.attrs, &["qsignal"]).is_some() {
            let parsed_signal_method = ParsedSignal::parse(foreign_fn.clone(), auto_case)?;
            if parsed_signal_method.inherit
                && foreign_fn.sig.unsafety.is_none()
                && self.unsafety.is_none()
            {
                return Err(Error::new(foreign_fn.span(), "block must be declared `unsafe extern \"RustQt\"` if it contains any safe-to-call #[inherit] qsignals"));
            }

            self.signals.push(parsed_signal_method);

            // Test if the function is an inheritance method
            //
            // Note that we need to test for qsignal first as qsignals have their own inherit meaning
        } else if attribute_get_path(&foreign_fn.attrs, &["inherit"]).is_some() {
            // We need to check that any safe functions are defined inside an unsafe block
            // as with inherit we cannot fully prove the implementation and we can then
            // directly copy the unsafety into the generated extern C++ block
            if foreign_fn.sig.unsafety.is_none() && self.unsafety.is_none() {
                return Err(Error::new(foreign_fn.span(), "block must be declared `unsafe extern \"RustQt\"` if it contains any safe-to-call #[inherit] functions"));
            }

            let parsed_inherited_method = ParsedInheritedMethod::parse(foreign_fn, auto_case)?;

            self.inherited_methods.push(parsed_inherited_method);
            // Remaining methods are either C++ methods or invokables
        } else {
            let parsed_method =
                ParsedMethod::parse(foreign_fn, auto_case, self.unsafety.is_some())?;
            self.methods.push(parsed_method);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::naming::Name;
    use crate::tests::assert_parse_errors;
    use quote::format_ident;
    use syn::parse_quote;

    #[test]
    fn test_parse_qsignals_safe() {
        let block: ItemForeignMod = parse_quote! {
            unsafe extern "RustQt" {
                #[qsignal]
                fn ready(self: Pin<&mut MyObject>);

                #[cxx_name="cppDataChanged"]
                #[inherit]
                #[qsignal]
                fn data_changed(self: Pin<&mut MyObject>, data: i32);
            }
        };
        let parsed_rust_qt =
            ParsedExternRustQt::parse(block, &format_ident!("qobject"), None).unwrap();
        let signals = parsed_rust_qt.signals;
        assert_eq!(signals.len(), 2);
        assert!(signals[0].mutable);
        assert!(signals[1].mutable);
        assert!(signals[0].safe);
        assert!(signals[1].safe);
        assert_eq!(signals[0].parameters.len(), 0);
        assert_eq!(signals[1].parameters.len(), 1);
        assert_eq!(signals[1].parameters[0].ident, "data");
        assert_eq!(signals[0].name, Name::new(format_ident!("ready")));
        assert_eq!(
            signals[1].name,
            Name::mock_name_with_cxx("data_changed", "cppDataChanged")
        );
        assert!(!signals[0].inherit);
        assert!(signals[1].inherit);
    }

    #[test]
    fn test_parse_qsignals_unsafe() {
        let block: ItemForeignMod = parse_quote! {
            extern "RustQt" {
                #[qsignal]
                #[cxx_name = "unsafeSignal"]
                unsafe fn unsafe_signal(self: Pin<&mut MyObject>, arg: *mut T);
            }
        };
        let parsed_rust_qt =
            ParsedExternRustQt::parse(block, &format_ident!("qobject"), None).unwrap();
        let signals = parsed_rust_qt.signals;
        assert_eq!(signals.len(), 1);
        assert!(signals[0].mutable);
        assert!(!signals[0].safe);
        assert_eq!(signals[0].parameters.len(), 1);
        assert_eq!(signals[0].parameters[0].ident, "arg");
        assert_eq!(
            signals[0].name,
            Name::mock_name_with_cxx("unsafe_signal", "unsafeSignal")
        );
        assert!(!signals[0].inherit);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_valid_qobject() {
        let block: ItemForeignMod = parse_quote! {
            unsafe extern "RustQt" {
                #[qinvokable]
                fn invokable(self: &MyObject);

                fn cpp_context(self: &MyObject);
            }
        };
        let parsed_rust_qt =
            ParsedExternRustQt::parse(block, &format_ident!("qobject"), None).unwrap();

        let methods = parsed_rust_qt.methods;
        assert_eq!(methods.len(), 2);
        assert!(methods[0].is_qinvokable);
        assert!(!methods[1].is_qinvokable);
    }

    #[test]
    fn test_parse_invalid() {
        assert_parse_errors!(
            |item| ParsedExternRustQt::parse(item, &format_ident!("qobject"), None) =>

            // Invalid QObject
            {
                unsafe extern "RustQt" {
                    #[qinvokable]
                    fn invokable(self: &MyObject::Bad);
                }
            }

            // Namespaces aren't allowed on qinvokables
            {
                unsafe extern "RustQt" {
                    #[qinvokable]
                    #[namespace = "disallowed"]
                    fn invokable(self: &MyObject);
                }
            }

            // Block or fn must be unsafe for inherit methods
            {
                extern "RustQt" {
                    #[inherit]
                    fn invokable(self: &MyObject);
                }
            }

            // Block or fn must be unsafe for inherit qsignals
            {
                extern "RustQt" {
                    #[inherit]
                    #[qsignal]
                    fn signal(self: Pin<&mut MyObject>);
                }
            }

            // Unsupported Item
            {
                extern "RustQt" {
                    static COUNTER: usize;
                }
            }
        );
    }
}
