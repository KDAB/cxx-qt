// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::qnamespace::ParsedQNamespace;
use super::trait_impl::TraitImpl;
use crate::naming::cpp::err_unsupported_item;
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
use syn::{ForeignItem, Ident, Item, ItemEnum, ItemForeignMod, ItemImpl, Result};
use syn::{ItemMacro, Meta};

pub struct ParsedCxxQtData {
    /// Map of the QObjects defined in the module that will be used for code generation
    //
    // We have to use a BTreeMap here, instead of a HashMap, to keep the order of QObjects stable.
    // Otherwise, the output order would be different, depending on the environment, which makes it hard to test/debug.
    pub qobjects: Vec<ParsedQObject>,
    /// List of QEnums defined in the module, that aren't associated with a QObject
    pub qenums: Vec<ParsedQEnum>,
    /// List of methods and Q_INVOKABLES found
    pub methods: Vec<ParsedMethod>,
    /// List of the Q_SIGNALS found
    pub signals: Vec<ParsedSignal>,
    /// List of the inherited methods found
    pub inherited_methods: Vec<ParsedInheritedMethod>,
    /// List of QNamespace declarations
    pub qnamespaces: Vec<ParsedQNamespace>,
    /// Blocks of extern "C++Qt"
    pub extern_cxxqt_blocks: Vec<ParsedExternCxxQt>,
    /// The namespace of the CXX-Qt module
    pub namespace: Option<String>,
    /// All trait implementations found
    pub trait_impls: Vec<TraitImpl>,
    /// The ident of the module, used for mappings
    pub module_ident: Ident,
}

impl ParsedCxxQtData {
    /// Create a ParsedCxxQtData from a given module and namespace
    pub fn new(module_ident: Ident, namespace: Option<String>) -> Self {
        Self {
            qobjects: Vec::new(),
            qenums: vec![],
            methods: vec![],
            signals: vec![],
            inherited_methods: vec![],
            qnamespaces: vec![],
            trait_impls: vec![],
            extern_cxxqt_blocks: Vec::<ParsedExternCxxQt>::default(),
            module_ident,
            namespace,
        }
    }

    /// Determine if the given [syn::Item] is a CXX-Qt related item
    /// If it is then add the [syn::Item] into qobjects BTreeMap
    /// Otherwise return the [syn::Item] to pass through to CXX
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
        let namespace = attribute_find_path(&foreign_mod.attrs, &["namespace"])
            .map(|index| expr_to_string(&foreign_mod.attrs[index].meta.require_name_value()?.value))
            .transpose()?
            .or_else(|| self.namespace.clone());

        let safe_call = if foreign_mod.unsafety.is_some() {
            Safety::Safe
        } else {
            Safety::Unsafe
        };

        for item in foreign_mod.items.drain(..) {
            match item {
                ForeignItem::Fn(mut foreign_fn) => {
                    // Test if the function is a signal
                    if attribute_take_path(&mut foreign_fn.attrs, &["qsignal"]).is_some() {
                        let parsed_signal_method = ParsedSignal::parse(foreign_fn, safe_call)?;
                        self.signals.push(parsed_signal_method);

                        // Test if the function is an inheritance method
                        //
                        // Note that we need to test for qsignal first as qsignals have their own inherit meaning
                    } else if attribute_take_path(&mut foreign_fn.attrs, &["inherit"]).is_some() {
                        let parsed_inherited_method =
                            ParsedInheritedMethod::parse(foreign_fn, safe_call)?;

                        self.inherited_methods.push(parsed_inherited_method);
                        // Remaining methods are either C++ methods or invokables
                    } else {
                        let parsed_method = ParsedMethod::parse(foreign_fn, safe_call)?;
                        self.methods.push(parsed_method);
                    }
                }
                ForeignItem::Verbatim(tokens) => {
                    let foreign_alias: ForeignTypeIdentAlias = syn::parse2(tokens.clone())?;

                    // Load the QObject
                    let qobject = ParsedQObject::parse(
                        foreign_alias,
                        namespace.as_deref(),
                        &self.module_ident,
                    )?;

                    // Note that we assume a compiler error will occur later
                    // if you had two structs with the same name
                    self.qobjects.push(qobject);
                }
                // Const Macro, Type are unsupported in extern "RustQt" for now
                _ => return Err(err_unsupported_item(&item)),
            }
        }
        Ok(())
    }

    /// Parse a [syn::ItemImpl] into the qobjects if it's a CXX-Qt implementation
    /// otherwise return as a [syn::Item] to pass through.
    fn parse_impl(&mut self, imp: ItemImpl) -> Result<Option<Item>> {
        // If it is a trait impl compared to a regular impl block
        // This allows the cxx shim trait feature
        if imp.trait_.is_some() {
            self.trait_impls.push(TraitImpl::parse(imp)?);
            Ok(None)
        } else {
            Ok(Some(Item::Impl(imp)))
        }
    }

    #[cfg(test)]
    fn find_object(&self, id: &Ident) -> Option<&ParsedQObject> {
        self.qobjects
            .iter()
            .find(|obj| obj.name.rust_unqualified() == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::structuring::Structures;
    use crate::{naming::Name, parser::qobject::tests::create_parsed_qobject};
    use quote::format_ident;
    use syn::parse_quote;

    /// Creates a ParsedCxxQtData with a QObject definition already found
    pub fn create_parsed_cxx_qt_data() -> ParsedCxxQtData {
        let mut cxx_qt_data = ParsedCxxQtData::new(format_ident!("ffi"), None);
        cxx_qt_data.qobjects.push(create_parsed_qobject());
        cxx_qt_data.qobjects.push(create_parsed_qobject());
        cxx_qt_data
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
    fn test_parse_unnamed_extern_mod() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            unsafe extern {
                type A;
            }
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_impl_non_path() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            impl &MyStruct {}
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_impl_non_ident() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            impl ::MyStruct {}
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invokable_wrong_safety() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            extern "RustQt" {
                #[qinvokable]
                fn invokable(self: &MyObject);
            }
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_err());
    }

    #[test]
    fn test_invokable_disallowed_namespace() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            unsafe extern "RustQt" {
                #[qinvokable]
                #[namespace = "disallowed"]
                fn invokable(self: &MyObject);
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

        let structures = Structures::new(&cxxqtdata).unwrap();

        let qobject = structures.qobjects.first().unwrap();

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
    fn test_parse_qsignals_unsafe() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();
        let block: Item = parse_quote! {
            extern "RustQt" {
                #[qsignal]
                unsafe fn unsafe_signal(self: Pin<&mut MyObject>, arg: *mut T);
            }
        };
        cxxqtdata.parse_cxx_qt_item(block).unwrap();

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
        assert!(cxxqtdata.trait_impls.is_empty());

        let threading_block: Item = parse_quote! {
            impl cxx_qt::Threading for MyObject {}
        };
        let result = cxxqtdata.parse_cxx_qt_item(threading_block).unwrap();
        assert!(result.is_none());
        assert!(!cxxqtdata.trait_impls.is_empty());
    }

    #[test]
    fn test_passthrough_non_trait_impl() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();

        let result = cxxqtdata
            .parse_cxx_qt_item(parse_quote! {
                impl T {}
            })
            .unwrap();
        assert!(result.is_some());
        assert!(matches!(result, Some(Item::Impl(_))));
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

    #[test]
    fn test_parse_unsupported_type() {
        let mut cxx_qt_data = ParsedCxxQtData::new(format_ident!("ffi"), None);
        let extern_rust_qt: Item = parse_quote! {
            extern "RustQt" {
                static COUNTER: usize;
            }
        };
        assert!(cxx_qt_data.parse_cxx_qt_item(extern_rust_qt).is_err());
    }

    #[test]
    fn test_qobjects() {
        let mut parsed_cxxqtdata = ParsedCxxQtData::new(format_ident!("ffi"), None);
        let extern_rust_qt: Item = parse_quote! {
            extern "RustQt" {
                #[qobject]
                type MyObject = super::T;
                #[qobject]
                type MyOtherObject = super::MyOtherT;
            }
        };

        parsed_cxxqtdata.parse_cxx_qt_item(extern_rust_qt).unwrap();
        assert_eq!(parsed_cxxqtdata.qobjects.len(), 2);

        assert!(parsed_cxxqtdata
            .find_object(&format_ident!("MyObject"))
            .is_some());
        assert!(parsed_cxxqtdata
            .find_object(&format_ident!("MyOtherObject"))
            .is_some());
    }

    #[test]
    fn test_qobject_namespaces() {
        let mut parsed_cxxqtdata = ParsedCxxQtData::new(format_ident!("ffi"), None);
        let extern_rust_qt: Item = parse_quote! {
            #[namespace="b"]
            extern "RustQt" {
                #[qobject]
                #[namespace="a"]
                type MyObject = super::T;
                #[qobject]
                type MyOtherObject = super::MyOtherT;
            }
        };

        parsed_cxxqtdata.parse_cxx_qt_item(extern_rust_qt).unwrap();
        assert_eq!(parsed_cxxqtdata.qobjects.len(), 2);
        assert_eq!(
            parsed_cxxqtdata
                .find_object(&format_ident!("MyObject"))
                .unwrap()
                .name
                .namespace(),
            Some("a")
        );
        assert_eq!(
            parsed_cxxqtdata
                .find_object(&format_ident!("MyOtherObject"))
                .unwrap()
                .name
                .namespace(),
            Some("b")
        );
    }
}
