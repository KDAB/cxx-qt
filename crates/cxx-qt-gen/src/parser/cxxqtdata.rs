// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::qnamespace::ParsedQNamespace;
use super::trait_impl::TraitImpl;
use crate::parser::externrustqt::ParsedExternRustQt;
use crate::{
    parser::{
        externcxxqt::ParsedExternCxxQt, inherit::ParsedInheritedMethod, method::ParsedMethod,
        qenum::ParsedQEnum, qobject::ParsedQObject, signals::ParsedSignal,
    },
    syntax::{attribute::attribute_get_path, path::path_compare_str},
};
use syn::{Ident, Item, ItemEnum, ItemForeignMod, ItemImpl, ItemMacro, Meta, Result};

pub struct ParsedCxxQtData {
    /// List of QEnums defined in the module, that aren't associated with a QObject
    pub qenums: Vec<ParsedQEnum>,
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
    /// All the `extern "RustQt"` blocks
    pub extern_rustqt_blocks: Vec<ParsedExternRustQt>,
}

impl ParsedCxxQtData {
    /// Create a ParsedCxxQtData from a given module and namespace
    pub fn new(module_ident: Ident, namespace: Option<String>) -> Self {
        Self {
            module_ident,
            namespace,
            qenums: vec![],
            qnamespaces: vec![],
            trait_impls: vec![],
            extern_cxxqt_blocks: vec![],
            extern_rustqt_blocks: vec![],
        }
    }

    /// Flatten a vector from each rust block into one larger vector,
    /// e.g. all the methods in every block.
    fn flatten_rust_blocks<T>(&self, accessor: fn(&ParsedExternRustQt) -> &Vec<T>) -> Vec<&T> {
        self.extern_rustqt_blocks
            .iter()
            .flat_map(accessor)
            .collect()
    }

    pub fn methods(&self) -> Vec<&ParsedMethod> {
        self.flatten_rust_blocks(|block| &block.methods)
    }

    pub fn signals(&self) -> Vec<&ParsedSignal> {
        self.flatten_rust_blocks(|block| &block.signals)
    }

    pub fn inherited_methods(&self) -> Vec<&ParsedInheritedMethod> {
        self.flatten_rust_blocks(|block| &block.inherited_methods)
    }

    pub fn qobjects(&self) -> Vec<&ParsedQObject> {
        self.flatten_rust_blocks(|block| &block.qobjects)
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

    fn parse_enum(&mut self, item: ItemEnum) -> Result<Option<Item>> {
        if let Some(qenum_attribute) = attribute_get_path(&item.attrs, &["qenum"]) {
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
                    self.extern_rustqt_blocks.push(ParsedExternRustQt::parse(
                        foreign_mod,
                        &self.module_ident,
                        self.namespace.as_deref(),
                    )?);
                    return Ok(None);
                }
                "C++Qt" => {
                    self.extern_cxxqt_blocks.push(ParsedExternCxxQt::parse(
                        foreign_mod,
                        &self.module_ident,
                        self.namespace.as_deref(),
                    )?);
                    return Ok(None);
                }
                _others => {}
            }
        }

        Ok(Some(Item::ForeignMod(foreign_mod)))
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
        self.qobjects()
            .into_iter()
            .find(|obj| obj.name.rust_unqualified() == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::structuring::Structures;
    use crate::parser::qobject::tests::create_parsed_qobject;
    use quote::format_ident;
    use syn::parse_quote;

    /// Creates a ParsedCxxQtData with a QObject definition already found
    pub fn create_parsed_cxx_qt_data() -> ParsedCxxQtData {
        let mut cxx_qt_data = ParsedCxxQtData::new(format_ident!("ffi"), None);
        cxx_qt_data.extern_rustqt_blocks.push(ParsedExternRustQt {
            unsafety: None,
            qobjects: vec![create_parsed_qobject(), create_parsed_qobject()],
            methods: vec![],
            signals: vec![],
            inherited_methods: vec![],
        });
        cxx_qt_data
    }

    use crate::tests::assert_parse_errors;

    #[test]
    fn test_parse_invalid() {
        assert_parse_errors! {
            |item| create_parsed_cxx_qt_data().parse_cxx_qt_item(item) =>

            // Qenum without namespace
            {
                #[qenum]
                enum MyEnum {
                    A,
                    B
                }
            }

            // Unsupported name for case conversion
            {
                #[auto_cxx_name = Foo]
                extern "RustQt" {}
            }

            // Auto case uses ident not string
            {
                #[auto_cxx_name = "Camel"]
                extern "RustQt" {}
            }

            // Unsupported format for case conversion macro
            {
                #[auto_cxx_name(a, b)]
                extern "RustQt" {}
            }
        }
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
    fn test_parse_auto_case_rustqt() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            #[auto_cxx_name]
            unsafe extern "RustQt" {
                fn foo_bar(self: &MyObject);
            }
        };
        cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert_eq!(cxx_qt_data.methods().len(), 1);
        assert_eq!(cxx_qt_data.methods()[0].name.cxx_unqualified(), "fooBar");
    }

    #[test]
    fn test_parse_auto_case_explicit() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            #[auto_cxx_name = Snake]
            #[auto_rust_name = Snake]
            unsafe extern "RustQt" {
                fn fooBar(self: &MyObject);
            }
        };
        cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        let methods = cxx_qt_data.methods();
        assert_eq!(methods.len(), 1);
        assert_eq!(methods[0].name.cxx_unqualified(), "foo_bar");
        assert_eq!(methods[0].name.rust_unqualified(), "foo_bar");
    }

    #[test]
    fn test_parse_auto_case_override() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            #[auto_cxx_name]
            unsafe extern "RustQt" {
                #[cxx_name = "renamed"]
                fn foo_bar(self: &MyObject);
            }
        };
        cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert_eq!(cxx_qt_data.methods().len(), 1);
        assert_eq!(cxx_qt_data.methods()[0].name.cxx_unqualified(), "renamed");
    }

    #[test]
    fn test_parse_auto_case_foreign() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            #[auto_rust_name]
            unsafe extern "C++Qt" {
                #[qobject]
                type MyObject;

                #[qsignal]
                fn fooBar(self: Pin<&mut MyObject>);
            }
        };
        cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert_eq!(cxx_qt_data.extern_cxxqt_blocks.len(), 1);
        assert_eq!(cxx_qt_data.extern_cxxqt_blocks[0].signals.len(), 1);
        assert_eq!(
            cxx_qt_data.extern_cxxqt_blocks[0].signals[0]
                .name
                .rust_unqualified(),
            "foo_bar"
        );
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

        let qenum: Item = parse_quote! {
            #[qenum]
            #[namespace="my_namespace"]
            enum MyEnum {
                A,
                B
            }
        };
        assert!(cxxqtdata.parse_cxx_qt_item(qenum).unwrap().is_none());
        assert_eq!(1, cxxqtdata.qenums.len());

        let qenum = &cxxqtdata.qenums[0];
        assert_eq!("my_namespace", qenum.name.namespace().unwrap());

        cxxqtdata.namespace = Some("other_namespace".to_owned());

        assert_eq!(1, cxxqtdata.qenums.len());
    }

    #[test]
    fn test_find_qobjects() {
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
        assert_eq!(parsed_cxxqtdata.qobjects().len(), 2);

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
        assert_eq!(parsed_cxxqtdata.qobjects().len(), 2);
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
