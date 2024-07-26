// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::attribute::{attribute_find_path, attribute_take_path};
use crate::syntax::foreignmod::ForeignTypeIdentAlias;
use crate::syntax::path::path_compare_str;
use crate::syntax::safety::Safety;
use crate::{
    parser::{
        externcxxqt::ParsedExternCxxQt, inherit::ParsedInheritedMethod, method::ParsedMethod,
        qenum::ParsedQEnum, qobject::ParsedQObject, signals::ParsedSignal,
    },
    syntax::expr::expr_to_string,
};
use std::collections::BTreeMap;
use syn::{
    spanned::Spanned, Error, ForeignItem, Ident, Item, ItemEnum, ItemForeignMod, ItemImpl, Result,
    Type, TypePath,
};
use syn::{ItemMacro, Meta};

use super::qnamespace::ParsedQNamespace;

pub struct ParsedCxxQtData {
    /// Map of the QObjects defined in the module that will be used for code generation
    //
    // We have to use a BTreeMap here, instead of a HashMap, to keep the order of QObjects stable.
    // Otherwise, the output order would be different, depending on the environment, which makes it hard to test/debug.
    pub qobjects: BTreeMap<Ident, ParsedQObject>,
    /// List of QEnums defined in the module, that aren't associated with a QObject
    pub qenums: Vec<ParsedQEnum>,
    /// List of methods and Q_INVOKABLES found
    pub methods: Vec<ParsedMethod>,
    /// List of the Q_SIGNALS found
    pub signals: Vec<ParsedSignal>,
    /// List of QNamespace declarations
    pub qnamespaces: Vec<ParsedQNamespace>,
    /// Blocks of extern "C++Qt"
    pub extern_cxxqt_blocks: Vec<ParsedExternCxxQt>,
    /// The namespace of the CXX-Qt module
    pub namespace: Option<String>,
    /// The ident of the module, used for mappings
    pub module_ident: Ident,
}

impl ParsedCxxQtData {
    /// Create a ParsedCxxQtData from a given module and namespace
    pub fn new(module_ident: Ident, namespace: Option<String>) -> Self {
        Self {
            qobjects: BTreeMap::<Ident, ParsedQObject>::default(),
            qenums: vec![],
            methods: vec![],
            signals: vec![],
            qnamespaces: vec![],
            extern_cxxqt_blocks: Vec::<ParsedExternCxxQt>::default(),
            module_ident,
            namespace,
        }
    }

    /// Find the QObjects within the module and add into the qobjects BTreeMap
    pub fn find_qobject_types(&mut self, items: &[Item]) -> Result<()> {
        for item in items {
            if let Item::ForeignMod(foreign_mod) = item {
                if foreign_mod.abi.name.as_ref().map(|lit_str| lit_str.value())
                    == Some("RustQt".to_string())
                {
                    // Find the namespace on the foreign mod block if there is one
                    let namespace = attribute_find_path(&foreign_mod.attrs, &["namespace"])
                        .map(|index| {
                            expr_to_string(
                                &foreign_mod.attrs[index].meta.require_name_value()?.value,
                            )
                        })
                        .transpose()?
                        .or_else(|| self.namespace.clone());

                    for foreign_item in &foreign_mod.items {
                        match foreign_item {
                            // Fn are parsed later in parse_foreign_mod_rust_qt
                            ForeignItem::Fn(_) => {}
                            ForeignItem::Verbatim(tokens) => {
                                let mut foreign_alias: ForeignTypeIdentAlias =
                                    syn::parse2(tokens.clone())?;

                                // Check this type is tagged with a #[qobject]
                                let has_qobject_macro =
                                    attribute_take_path(&mut foreign_alias.attrs, &["qobject"])
                                        .is_some();

                                // Load the QObject
                                let mut qobject = ParsedQObject::parse(
                                    foreign_alias,
                                    namespace.as_deref(),
                                    &self.module_ident,
                                )?;
                                qobject.has_qobject_macro = has_qobject_macro;

                                // Ensure that the base class attribute is not empty, as this is not valid in both cases
                                // - when there is a qobject macro it is not valid
                                // - when there is not a qobject macro it is not valid
                                if qobject
                                    .base_class
                                    .as_ref()
                                    .is_some_and(|base| base.is_empty())
                                {
                                    return Err(Error::new(
                                        foreign_item.span(),
                                        "The #[base] attribute cannot be empty",
                                    ));
                                }

                                // Ensure that if there is no qobject macro that a base class is specificed
                                //
                                // Note this assumes the check above
                                if !qobject.has_qobject_macro && qobject.base_class.is_none() {
                                    return Err(Error::new(foreign_item.span(), "A type without a #[qobject] attribute must specify a #[base] attribute"));
                                }

                                // Note that we assume a compiler error will occur later
                                // if you had two structs with the same name
                                self.qobjects
                                    .insert(qobject.name.rust_unqualified().clone(), qobject);
                            }
                            // Const Macro, Type are unsupported in extern "RustQt" for now
                            _others => {
                                return Err(Error::new(foreign_item.span(), "Unsupported item"))
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Determine if the given [Item] is a CXX-Qt related item
    /// If it is then add the [Item] into qobjects BTreeMap
    /// Otherwise return the [Item] to pass through to CXX
    pub fn parse_cxx_qt_item(&mut self, item: Item) -> Result<Option<Item>> {
        match item {
            Item::Impl(imp) => self.parse_impl(imp),
            Item::ForeignMod(foreign_mod) => self.parse_foreign_mod(foreign_mod),
            Item::Enum(enum_item) => self.parse_enum(enum_item),
            Item::Macro(mac) => self.parse_macro(mac),
            _ => Ok(Some(item)),
        }
    }

    fn parse_enum(&mut self, mut item: ItemEnum) -> Result<Option<Item>> {
        if let Some(qenum_attribute) = attribute_take_path(&mut item.attrs, &["qenum"]) {
            // A Meta::Path indicates no arguments were provided to the enum
            // It only contains the "qenum" path and nothing else.
            let qobject: Option<Ident> = if let Meta::Path(_) = qenum_attribute.meta {
                None
            } else {
                Some(qenum_attribute.parse_args()?)
            };

            let qenum =
                ParsedQEnum::parse(item, qobject, self.namespace.as_deref(), &self.module_ident)?;

            self.qenums.push(qenum);
            Ok(None)
        } else {
            Ok(Some(Item::Enum(item)))
        }
    }

    fn parse_macro(&mut self, item: ItemMacro) -> Result<Option<Item>> {
        if path_compare_str(&item.mac.path, &["qnamespace"]) {
            let qnamespace = ParsedQNamespace::parse(item)?;
            self.qnamespaces.push(qnamespace);
            Ok(None)
        } else {
            Ok(Some(Item::Macro(item)))
        }
    }

    fn parse_foreign_mod(&mut self, foreign_mod: ItemForeignMod) -> Result<Option<Item>> {
        if let Some(lit_str) = &foreign_mod.abi.name {
            match lit_str.value().as_str() {
                "RustQt" => {
                    self.parse_foreign_mod_rust_qt(foreign_mod)?;
                    return Ok(None);
                }
                "C++Qt" => {
                    self.extern_cxxqt_blocks
                        .push(ParsedExternCxxQt::parse(foreign_mod)?);
                    return Ok(None);
                }
                _others => {}
            }
        }

        Ok(Some(Item::ForeignMod(foreign_mod)))
    }

    fn parse_foreign_mod_rust_qt(&mut self, mut foreign_mod: ItemForeignMod) -> Result<()> {
        let safe_call = if foreign_mod.unsafety.is_some() {
            Safety::Safe
        } else {
            Safety::Unsafe
        };

        for item in foreign_mod.items.drain(..) {
            if let ForeignItem::Fn(mut foreign_fn) = item {
                // Test if the function is a signal
                if attribute_take_path(&mut foreign_fn.attrs, &["qsignal"]).is_some() {
                    let parsed_signal_method = ParsedSignal::parse(foreign_fn.clone(), safe_call)?;

                    // TODO: Eventually the with_qobject use can be removed from here and below in methods, and use just the Structure approach
                    let parsed_signal_method_self = ParsedSignal::parse(foreign_fn, safe_call)?;
                    self.signals.push(parsed_signal_method_self);

                    self.with_qobject(&parsed_signal_method.qobject_ident)?
                        .signals
                        .push(parsed_signal_method);
                    // Test if the function is an inheritance method
                    //
                    // Note that we need to test for qsignal first as qsignals have their own inherit meaning
                } else if attribute_take_path(&mut foreign_fn.attrs, &["inherit"]).is_some() {
                    let parsed_inherited_method =
                        ParsedInheritedMethod::parse(foreign_fn, safe_call)?;

                    self.with_qobject(&parsed_inherited_method.qobject_ident)?
                        .inherited_methods
                        .push(parsed_inherited_method);
                    // Remaining methods are either C++ methods or invokables
                } else {
                    let parsed_method = ParsedMethod::parse(foreign_fn.clone(), safe_call)?;

                    let parsed_method_self = ParsedMethod::parse(foreign_fn, safe_call)?;
                    self.methods.push(parsed_method_self);

                    self.with_qobject(&parsed_method.qobject_ident)?
                        .methods
                        .push(parsed_method);
                }
            }
        }
        Ok(())
    }

    /// Parse a [ItemImpl] into the qobjects if it's a CXX-Qt implementation
    /// otherwise return as a [Item] to pass through.
    fn parse_impl(&mut self, imp: ItemImpl) -> Result<Option<Item>> {
        // If the implementation has a T
        // then this is the block of methods to be implemented on the C++ object
        if let Type::Path(TypePath { path, .. }) = imp.self_ty.as_ref() {
            // If this path is an ident then try to match to a QObject
            if let Some(ident) = path.get_ident() {
                // Find if we are an impl block for a qobject
                if let Some(qobject) = self.qobjects.get_mut(ident) {
                    // If we are a trait then process it otherwise add to others
                    if imp.trait_.is_some() {
                        qobject.parse_trait_impl(imp)?;
                        return Ok(None);
                    }
                }
            }
        }

        Ok(Some(Item::Impl(imp)))
    }

    fn with_qobject(&mut self, qobject_ident: &Ident) -> Result<&mut ParsedQObject> {
        if let Some(qobject) = self.qobjects.get_mut(qobject_ident) {
            Ok(qobject)
        } else {
            Err(Error::new_spanned(
                qobject_ident,
                "No QObject with this name found.",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{naming::Name, parser::qobject::tests::create_parsed_qobject};
    use quote::format_ident;
    use syn::{parse_quote, ItemMod};

    /// The QObject ident used in these tests as the ident that already
    /// has been found.
    fn qobject_ident() -> Ident {
        format_ident!("MyObject")
    }

    /// Creates a ParsedCxxQtData with a QObject definition already found
    pub fn create_parsed_cxx_qt_data() -> ParsedCxxQtData {
        let mut cxx_qt_data = ParsedCxxQtData::new(format_ident!("ffi"), None);
        cxx_qt_data
            .qobjects
            .insert(qobject_ident(), create_parsed_qobject());
        cxx_qt_data
    }

    #[test]
    fn test_find_qobjects_one_qobject() {
        let mut cxx_qt_data = ParsedCxxQtData::new(format_ident!("ffi"), None);

        let module: ItemMod = parse_quote! {
            mod module {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let result = cxx_qt_data.find_qobject_types(&module.content.unwrap().1);
        assert!(result.is_ok());
        assert_eq!(cxx_qt_data.qobjects.len(), 1);
        assert!(cxx_qt_data.qobjects.contains_key(&qobject_ident()));
        assert!(
            cxx_qt_data
                .qobjects
                .get(&qobject_ident())
                .unwrap()
                .has_qobject_macro
        );
    }

    #[test]
    fn test_find_qobjects_multiple_qobject() {
        let mut cxx_qt_data = ParsedCxxQtData::new(format_ident!("ffi"), None);

        let module: ItemMod = parse_quote! {
            mod module {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                    #[qobject]
                    type SecondObject = super::SecondObjectRust;
                    #[qobject]
                    #[rust_name="ThirdObjectQt"]
                    type ThirdObject = super::ThirdObjectRust;
                }
            }
        };
        let result = cxx_qt_data.find_qobject_types(&module.content.unwrap().1);
        assert!(result.is_ok());
        let qobjects = &cxx_qt_data.qobjects;
        assert_eq!(qobjects.len(), 3);
        assert!(qobjects.contains_key(&qobject_ident()));
        assert!(qobjects.contains_key(&format_ident!("SecondObject")));
        // Ensure the rust_name attribute is used as the key.
        assert!(qobjects.contains_key(&format_ident!("ThirdObjectQt")));
    }

    #[test]
    fn test_find_qobjects_namespace() {
        let mut cxx_qt_data =
            ParsedCxxQtData::new(format_ident!("ffi"), Some("bridge_namespace".to_string()));

        let module: ItemMod = parse_quote! {
            mod module {
                extern "RustQt" {
                    #[qobject]
                    #[namespace = "qobject_namespace"]
                    type MyObject = super::MyObjectRust;
                    #[qobject]
                    type SecondObject = super::SecondObjectRust;
                }
            }
        };
        cxx_qt_data
            .find_qobject_types(&module.content.unwrap().1)
            .unwrap();
        assert_eq!(cxx_qt_data.qobjects.len(), 2);
        assert_eq!(
            cxx_qt_data
                .qobjects
                .get(&format_ident!("MyObject"))
                .unwrap()
                .name
                .namespace()
                .unwrap(),
            "qobject_namespace"
        );
        assert_eq!(
            cxx_qt_data
                .qobjects
                .get(&format_ident!("SecondObject"))
                .unwrap()
                .name
                .namespace()
                .unwrap(),
            "bridge_namespace"
        );
    }

    #[test]
    fn test_find_qobjects_no_qobject_no_base() {
        let mut cxx_qt_data = ParsedCxxQtData::new(format_ident!("ffi"), None);

        let module: ItemMod = parse_quote! {
            mod module {
                extern "RustQt" {
                    type Other = super::OtherRust;
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let result = cxx_qt_data.find_qobject_types(&module.content.unwrap().1);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_qobjects_no_qobject_with_base() {
        let mut cxx_qt_data = ParsedCxxQtData::new(format_ident!("ffi"), None);

        let module: ItemMod = parse_quote! {
            mod module {
                extern "RustQt" {
                    #[base = "OtherBase"]
                    type Other = super::OtherRust;
                    #[base = "MyObjectBase"]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let result = cxx_qt_data.find_qobject_types(&module.content.unwrap().1);
        assert!(result.is_ok());
        assert_eq!(cxx_qt_data.qobjects.len(), 2);
        assert!(
            !cxx_qt_data
                .qobjects
                .get(&format_ident!("Other"))
                .unwrap()
                .has_qobject_macro
        );
        assert!(
            !cxx_qt_data
                .qobjects
                .get(&format_ident!("MyObject"))
                .unwrap()
                .has_qobject_macro
        );
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_struct_qobject_passthrough() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            pub struct MyObject;
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_struct_passthrough() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            struct Unknown;
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_valid_qobject() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            unsafe extern "RustQt" {
                #[qinvokable]
                fn invokable(self: &MyObject);

                fn cpp_context(self: &MyObject);
            }
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].methods.len(), 2);
        assert!(cxx_qt_data.qobjects[&qobject_ident()].methods[0].is_qinvokable);
        assert!(!cxx_qt_data.qobjects[&qobject_ident()].methods[1].is_qinvokable);

        // TODO: Check only the ones associated with MyObject in tests
        assert_eq!(cxx_qt_data.methods.len(), 2);
        assert!(cxx_qt_data.methods[0].is_qinvokable);
        assert!(!cxx_qt_data.methods[1].is_qinvokable)
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_invalid_qobject() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            unsafe extern "RustQt" {
                #[qinvokable]
                fn invokable(self: &MyObject::Bad);
            }
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_unknown_qobject() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            impl UnknownObj {
                #[qinvokable]
                fn invokable() {}
            }
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_valid_rustobj() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            impl MyObject {
                fn method() {}
            }
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_passthrough() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            extern "Rust" {
                fn test();
            }
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_threading() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();
        assert!(!cxx_qt_data.qobjects[&qobject_ident()].threading);

        let item: Item = parse_quote! {
            impl cxx_qt::Threading for MyObject {}
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert!(cxx_qt_data.qobjects[&qobject_ident()].threading);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_extern_cxx_qt() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            #[namespace = "rust"]
            unsafe extern "C++Qt" {
                #[qobject]
                type QPushButton;

                #[qsignal]
                fn clicked(self: Pin<&mut QPushButton>, checked: bool);
            }
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());

        assert_eq!(cxx_qt_data.extern_cxxqt_blocks.len(), 1);
        assert_eq!(cxx_qt_data.extern_cxxqt_blocks[0].attrs.len(), 1);
        assert_eq!(
            cxx_qt_data.extern_cxxqt_blocks[0].passthrough_items.len(),
            1
        );
        assert_eq!(cxx_qt_data.extern_cxxqt_blocks[0].signals.len(), 1);
        assert!(cxx_qt_data.extern_cxxqt_blocks[0].unsafety.is_some());
    }

    #[test]
    fn test_parse_inherited_methods() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();

        let unsafe_block: Item = parse_quote! {
            unsafe extern "RustQt" {
                #[inherit]
                fn test(self: &MyObject);

                #[inherit]
                fn with_args(self: &MyObject, arg: i32);
            }
        };
        let safe_block: Item = parse_quote! {
            extern "RustQt" {
                #[cxx_name="withRename"]
                #[inherit]
                unsafe fn with_rename(self: Pin<&mut MyObject>, arg: i32);
            }
        };

        cxxqtdata.parse_cxx_qt_item(unsafe_block).unwrap();
        cxxqtdata.parse_cxx_qt_item(safe_block).unwrap();

        let qobject = cxxqtdata.qobjects.get(&qobject_ident()).unwrap();

        let inherited = &qobject.inherited_methods;
        assert_eq!(inherited.len(), 3);
        assert!(!inherited[0].mutable);
        assert!(!inherited[1].mutable);
        assert!(inherited[2].mutable);
        assert!(inherited[0].safe);
        assert!(inherited[1].safe);
        assert!(!inherited[2].safe);
        assert_eq!(inherited[0].parameters.len(), 0);
        assert_eq!(inherited[1].parameters.len(), 1);
        assert_eq!(inherited[1].parameters[0].ident, "arg");
        assert_eq!(inherited[2].parameters.len(), 1);
        assert_eq!(inherited[2].parameters[0].ident, "arg");
    }

    #[test]
    fn test_parse_qsignals_safe() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();
        let block: Item = parse_quote! {
            unsafe extern "RustQt" {
                #[qsignal]
                fn ready(self: Pin<&mut MyObject>);

                #[cxx_name="cppDataChanged"]
                #[inherit]
                #[qsignal]
                fn data_changed(self: Pin<&mut MyObject>, data: i32);
            }
        };
        cxxqtdata.parse_cxx_qt_item(block).unwrap();

        // let qobject = cxxqtdata.qobjects.get(&qobject_ident()).unwrap();

        // let signals = &qobject.signals;
        // TODO: check only the signals associated with MyObject
        let signals = &cxxqtdata.signals;
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
            Name::new(format_ident!("data_changed")).with_cxx_name("cppDataChanged".to_owned())
        );
        assert!(!signals[0].inherit);
        assert!(signals[1].inherit);
    }

    #[test]
    fn test_parse_qsignals_unknown_obj() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();
        let block: Item = parse_quote! {
            unsafe extern "RustQt" {
                #[qsignal]
                fn ready(self: Pin<&mut UnknownObj>);
            }
        };
        let parsed_block = cxxqtdata.parse_cxx_qt_item(block);
        assert!(parsed_block.is_err());
    }

    #[test]
    fn test_parse_qsignals_unsafe() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();
        let block: Item = parse_quote! {
            extern "RustQt" {
                #[qsignal]
                unsafe fn unsafe_signal(self: Pin<&mut MyObject>, arg: *mut T);
            }
        };
        cxxqtdata.parse_cxx_qt_item(block).unwrap();

        // let qobject = cxxqtdata.qobjects.get(&qobject_ident()).unwrap();

        // let signals = &qobject.signals;
        // TODO: ensure signals are only checked if associated with MyObject
        let signals = &cxxqtdata.signals;
        assert_eq!(signals.len(), 1);
        assert!(signals[0].mutable);
        assert!(!signals[0].safe);
        assert_eq!(signals[0].parameters.len(), 1);
        assert_eq!(signals[0].parameters[0].ident, "arg");
        assert_eq!(
            signals[0].name,
            Name::new(format_ident!("unsafe_signal")).with_cxx_name("unsafeSignal".to_owned())
        );
        assert!(!signals[0].inherit);
    }

    #[test]
    fn test_parse_threading() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();

        let qobject = cxxqtdata.qobjects.get(&qobject_ident()).unwrap();
        assert!(!qobject.threading);

        let threading_block: Item = parse_quote! {
            impl cxx_qt::Threading for MyObject {}
        };

        cxxqtdata.parse_cxx_qt_item(threading_block).unwrap();

        let qobject = cxxqtdata.qobjects.get(&qobject_ident()).unwrap();
        assert!(qobject.threading);
    }

    #[test]
    fn test_parse_namespaced_qenum() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();

        assert!(cxxqtdata.qenums.is_empty());

        let qenum_without_namespace: Item = parse_quote! {
            #[qenum]
            enum MyEnum {
                A,
                B
            }
        };

        assert!(cxxqtdata
            .parse_cxx_qt_item(qenum_without_namespace.clone())
            .is_err());

        let qenum_with_namespace: Item = parse_quote! {
            #[qenum]
            #[namespace="my_namespace"]
            enum MyEnum {
                A,
                B
            }
        };
        assert!(cxxqtdata
            .parse_cxx_qt_item(qenum_with_namespace)
            .unwrap()
            .is_none());
        assert_eq!(1, cxxqtdata.qenums.len());

        let qenum = &cxxqtdata.qenums[0];
        assert_eq!("my_namespace", qenum.name.namespace().unwrap());

        cxxqtdata.namespace = Some("other_namespace".to_string());

        assert!(cxxqtdata
            .parse_cxx_qt_item(qenum_without_namespace)
            .unwrap()
            .is_none());

        assert_eq!(2, cxxqtdata.qenums.len());
        assert_eq!(
            "other_namespace",
            cxxqtdata.qenums[1].name.namespace().unwrap()
        );
    }
}
