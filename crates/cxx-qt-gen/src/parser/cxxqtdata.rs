// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::foreignmod::foreign_mod_to_foreign_item_types;
use crate::syntax::{attribute::attribute_find_path, path::path_to_single_ident};
use crate::{
    parser::{
        inherit::{InheritMethods, ParsedInheritedMethod},
        qobject::ParsedQObject,
        signals::{ParsedSignal, SignalMethods},
    },
    syntax::expr::expr_to_string,
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::BTreeMap;
use syn::{
    spanned::Spanned, Attribute, Error, Ident, Item, ItemForeignMod, ItemImpl, Result, Type,
    TypePath,
};

#[derive(Default)]
pub struct ParsedCxxMappings {
    /// Map of the cxx_name of any types defined in CXX extern blocks
    ///
    /// This is used in the C++ generation to map the Rust type name to the C++ name
    pub cxx_names: BTreeMap<String, String>,
    /// Map of the namespace of any types or methods defined in CXX extern blocks
    ///
    /// This is used in the C++ generation to map the Rust type name to the C++ name
    pub namespaces: BTreeMap<String, String>,
}

impl ParsedCxxMappings {
    /// For a given rust ident return the CXX name with its namespace
    pub fn cxx(&self, ident: &str) -> String {
        // Check if there is a cxx_name or namespace to handle
        let cxx_name = self
            .cxx_names
            .get(ident)
            .cloned()
            .unwrap_or_else(|| ident.to_owned());

        if let Some(namespace) = self.namespaces.get(ident) {
            format!("::{namespace}::{cxx_name}")
        } else {
            cxx_name
        }
    }
}

#[derive(Default)]
pub struct ParsedCxxQtData {
    /// Mappings for
    pub cxx_mappings: ParsedCxxMappings,
    /// Map of the QObjects defined in the module that will be used for code generation
    //
    // We have to use a BTreeMap here, instead of a HashMap, to keep the order of QObjects stable.
    // Otherwise, the output order would be different, depending on the environment, which makes it hard to test/debug.
    pub qobjects: BTreeMap<Ident, ParsedQObject>,
    /// The namespace of the CXX-Qt module
    pub namespace: String,
    /// Any `use` statements end up in the CXX-Qt generated module
    pub uses: Vec<Item>,
}

impl ParsedCxxQtData {
    /// Find the QObjects within the module and add into the qobjects BTreeMap
    pub fn find_qobject_structs(&mut self, items: &[Item]) -> Result<()> {
        for item in items {
            if let Item::Struct(qobject_struct) = item {
                if let Some(index) =
                    attribute_find_path(&qobject_struct.attrs, &["cxx_qt", "qobject"])
                {
                    // Load the QObject
                    let mut qobject = ParsedQObject::from_struct(qobject_struct, index)?;

                    // Inject the bridge namespace if the qobject one is empty
                    if qobject.namespace.is_empty() && !self.namespace.is_empty() {
                        qobject.namespace = self.namespace.clone();
                    }

                    // Note that we assume a compiler error will occur later
                    // if you had two structs with the same name
                    self.qobjects.insert(qobject_struct.ident.clone(), qobject);
                }
            }
        }

        Ok(())
    }

    /// Search through Item's and look for a cxx_name or namespace attribute on a type
    ///
    /// We need to know this as it affects the type name used in the C++ generation
    pub fn populate_cxx_mappings_from_item(
        &mut self,
        item: &Item,
        bridge_namespace: &str,
    ) -> Result<()> {
        // Consider if shared types have mappings
        match item {
            Item::Enum(item) => {
                self.populate_cxx_mappings(&item.ident, &item.attrs, bridge_namespace)?;
            }
            Item::Struct(item) => {
                self.populate_cxx_mappings(&item.ident, &item.attrs, bridge_namespace)?;
            }
            _others => {}
        }

        // If there is a foreign mod then process it
        if let Item::ForeignMod(foreign_mod) = &item {
            // Retrieve a namespace from the mod or the bridge
            let block_namespace =
                if let Some(index) = attribute_find_path(&foreign_mod.attrs, &["namespace"]) {
                    expr_to_string(&foreign_mod.attrs[index].meta.require_name_value()?.value)?
                } else {
                    bridge_namespace.to_owned()
                };

            // Read each of the types in the mod (type A;)
            for foreign_type in foreign_mod_to_foreign_item_types(foreign_mod)? {
                self.populate_cxx_mappings(
                    &foreign_type.ident,
                    &foreign_type.attrs,
                    &block_namespace,
                )?;
            }
        }

        Ok(())
    }

    /// Helper which adds cxx_name and namespace mappings from the ident, attrs, and parent namespace
    fn populate_cxx_mappings(
        &mut self,
        ident: &Ident,
        attrs: &[Attribute],
        parent_namespace: &str,
    ) -> Result<()> {
        // Retrieve the namespace for the type itself if there is one
        let namespace = if let Some(index) = attribute_find_path(attrs, &["namespace"]) {
            expr_to_string(&attrs[index].meta.require_name_value()?.value)?
        } else {
            parent_namespace.to_string()
        };

        // There is a cxx_name attribute
        if let Some(index) = attribute_find_path(attrs, &["cxx_name"]) {
            self.cxx_mappings.cxx_names.insert(
                ident.to_string(),
                expr_to_string(&attrs[index].meta.require_name_value()?.value)?,
            );
        }

        // There is a namespace
        if !namespace.is_empty() {
            self.cxx_mappings
                .namespaces
                .insert(ident.to_string(), namespace);
        }

        Ok(())
    }

    /// Determine if the given [syn::Item] is a CXX-Qt related item
    /// If it is then add the [syn::Item] into qobjects BTreeMap
    /// Otherwise return the [syn::Item] to pass through to CXX
    pub fn parse_cxx_qt_item(&mut self, item: Item) -> Result<Option<Item>> {
        match item {
            Item::Impl(imp) => self.parse_impl(imp),
            // Ignore structs which are qobjects
            Item::Struct(s) if self.qobjects.contains_key(&s.ident) => Ok(None),
            Item::Use(_) => {
                // Any use statements go into the CXX-Qt generated block
                self.uses.push(item);
                Ok(None)
            }
            Item::ForeignMod(foreign_mod) => self.parse_foreign_mod(foreign_mod),
            _ => Ok(Some(item)),
        }
    }

    fn parse_inherit_mod(&mut self, tokens: TokenStream) -> Result<()> {
        let inherited: InheritMethods = syn::parse2(tokens)?;

        self.add_inherited_methods(inherited)
    }

    fn add_inherited_methods(&mut self, inherited: InheritMethods) -> Result<()> {
        for method in inherited.base_functions.into_iter() {
            let parsed_inherited_method = ParsedInheritedMethod::parse(method, inherited.safety)?;

            if let Some(ref mut qobject) = self
                .qobjects
                .get_mut(&parsed_inherited_method.qobject_ident)
            {
                qobject.inherited_methods.push(parsed_inherited_method);
            } else {
                return Err(Error::new_spanned(
                    parsed_inherited_method.qobject_ident,
                    "No QObject with this name found.",
                ));
            }
        }
        Ok(())
    }

    fn parse_signals_mod(&mut self, tokens: TokenStream) -> Result<()> {
        let signals: SignalMethods = syn::parse2(tokens)?;

        self.add_signal_methods(signals)
    }

    fn add_signal_methods(&mut self, signals: SignalMethods) -> Result<()> {
        for method in signals.base_functions.into_iter() {
            let parsed_signal_method = ParsedSignal::parse(method, signals.safety)?;

            if let Some(ref mut qobject) =
                self.qobjects.get_mut(&parsed_signal_method.qobject_ident)
            {
                qobject.signals.push(parsed_signal_method);
            } else {
                return Err(Error::new_spanned(
                    parsed_signal_method.qobject_ident,
                    "No QObject with this name found.",
                ));
            }
        }
        Ok(())
    }

    fn parse_foreign_mod(&mut self, mut foreign_mod: ItemForeignMod) -> Result<Option<Item>> {
        // Check if the foreign mod has cxx_qt::inherit on it
        if let Some(index) = attribute_find_path(&foreign_mod.attrs, &["cxx_qt", "inherit"]) {
            // Remove the inherit attribute
            foreign_mod.attrs.remove(index);

            self.parse_inherit_mod(foreign_mod.into_token_stream())?;
            return Ok(None);
        }

        // Check if the foreign mod has cxx_qt::qsignals on it
        if let Some(index) = attribute_find_path(&foreign_mod.attrs, &["cxx_qt", "qsignals"]) {
            // Remove the signals attribute
            foreign_mod.attrs.remove(index);

            self.parse_signals_mod(foreign_mod.into_token_stream())?;
            return Ok(None);
        }

        Ok(Some(Item::ForeignMod(foreign_mod)))
    }

    /// Parse a [syn::ItemImpl] into the qobjects if it's a CXX-Qt implementation
    /// otherwise return as a [syn::Item] to pass through.
    fn parse_impl(&mut self, imp: ItemImpl) -> Result<Option<Item>> {
        // If the implementation has a qobject::T
        // then this is the block of methods to be implemented on the C++ object
        if let Type::Path(TypePath { path, .. }) = imp.self_ty.as_ref() {
            // Find if we are a impl qobject::T
            if path.segments.len() == 2 && path.segments[0].ident == "qobject" {
                if let Some(qobject) = self.qobjects.get_mut(&path.segments[1].ident) {
                    if imp.trait_.is_some() {
                        qobject.parse_trait_impl(imp)?;
                    } else {
                        // Extract the ImplItem's from each Impl block
                        qobject.parse_impl_items(&imp.items)?;
                    }
                } else {
                    return Err(Error::new(
                        imp.span(),
                        "No matching QObject found for the given qobject::T impl block.",
                    ));
                }

                return Ok(None);
            // Find if we are an impl block for a qobject
            } else if let Some(qobject) = self.qobjects.get_mut(&path_to_single_ident(path)?) {
                qobject.others.push(Item::Impl(imp));
                return Ok(None);
            }
        }

        Ok(Some(Item::Impl(imp)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{generator::naming::CombinedIdent, parser::qobject::tests::create_parsed_qobject};
    use quote::format_ident;
    use syn::{parse_quote, ItemMod};

    /// The QObject ident used in these tests as the ident that already
    /// has been found.
    fn qobject_ident() -> Ident {
        format_ident!("MyObject")
    }

    /// Creates a ParsedCxxQtData with a QObject definition already found
    fn create_parsed_cxx_qt_data() -> ParsedCxxQtData {
        let mut cxx_qt_data = ParsedCxxQtData::default();
        cxx_qt_data
            .qobjects
            .insert(qobject_ident(), create_parsed_qobject());
        cxx_qt_data
    }

    #[test]
    fn test_find_qobjects_one_qobject() {
        let mut cxx_qt_data = ParsedCxxQtData::default();

        let module: ItemMod = parse_quote! {
            mod module {
                struct Other;
                #[cxx_qt::qobject]
                pub struct MyObject;
            }
        };
        let result = cxx_qt_data.find_qobject_structs(&module.content.unwrap().1);
        assert!(result.is_ok());
        assert_eq!(cxx_qt_data.qobjects.len(), 1);
        assert!(cxx_qt_data.qobjects.contains_key(&qobject_ident()));
    }

    #[test]
    fn test_find_qobjects_multiple_qobject() {
        let mut cxx_qt_data = ParsedCxxQtData::default();

        let module: ItemMod = parse_quote! {
            mod module {
                pub struct Other;
                #[cxx_qt::qobject]
                pub struct MyObject;
                #[cxx_qt::qobject]
                pub struct SecondObject;
            }
        };
        let result = cxx_qt_data.find_qobject_structs(&module.content.unwrap().1);
        assert!(result.is_ok());
        assert_eq!(cxx_qt_data.qobjects.len(), 2);
        assert!(cxx_qt_data.qobjects.contains_key(&qobject_ident()));
        assert!(cxx_qt_data
            .qobjects
            .contains_key(&format_ident!("SecondObject")));
    }

    #[test]
    fn test_find_qobjects_namespace() {
        let mut cxx_qt_data = ParsedCxxQtData {
            namespace: "bridge_namespace".to_owned(),
            ..Default::default()
        };

        let module: ItemMod = parse_quote! {
            mod module {
                pub struct Other;
                #[cxx_qt::qobject(namespace = "qobject_namespace")]
                pub struct MyObject;
                #[cxx_qt::qobject]
                pub struct SecondObject;
            }
        };
        cxx_qt_data
            .find_qobject_structs(&module.content.unwrap().1)
            .unwrap();
        assert_eq!(cxx_qt_data.qobjects.len(), 2);
        assert_eq!(
            cxx_qt_data
                .qobjects
                .get(&format_ident!("MyObject"))
                .unwrap()
                .namespace,
            "qobject_namespace"
        );
        assert_eq!(
            cxx_qt_data
                .qobjects
                .get(&format_ident!("SecondObject"))
                .unwrap()
                .namespace,
            "bridge_namespace"
        );
    }

    #[test]
    fn test_find_qobjects_no_macro() {
        let mut cxx_qt_data = ParsedCxxQtData::default();

        let module: ItemMod = parse_quote! {
            mod module {
                pub struct Other;
                pub struct MyObject;
            }
        };
        let result = cxx_qt_data.find_qobject_structs(&module.content.unwrap().1);
        assert!(result.is_ok());
        assert_eq!(cxx_qt_data.qobjects.len(), 0);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_struct_qobject() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            #[cxx_qt::qobject]
            pub struct MyObject;
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
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
            impl qobject::MyObject {
                #[qinvokable]
                fn invokable(&self) {}

                fn cpp_context() {}
            }
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].invokables.len(), 1);
        assert_eq!(
            cxx_qt_data.qobjects[&qobject_ident()]
                .passthrough_impl_items
                .len(),
            1
        );
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_invalid_qobject() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            impl qobject::MyObject::Bad {
                #[qinvokable]
                fn invokable() {}
            }
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_unknown_qobject() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            impl qobject::UnknownObj {
                #[qinvokable]
                fn invokable() {}
            }
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_err());
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
        assert!(result.is_none());
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].others.len(), 1);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_uses() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            use std::collections::HashMap;
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert_eq!(cxx_qt_data.uses.len(), 1);
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
            impl cxx_qt::Threading for qobject::MyObject {}
        };
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert!(cxx_qt_data.qobjects[&qobject_ident()].threading);
    }

    #[test]
    fn test_cxx_mappings_cxx_name_empty() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            extern "C++" {
                type A;
            }
        };
        assert!(cxx_qt_data
            .populate_cxx_mappings_from_item(&item, "")
            .is_ok());
        assert!(cxx_qt_data.cxx_mappings.cxx_names.is_empty());
    }

    #[test]
    fn test_cxx_mappings_cxx_name_normal() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            extern "C++" {
                #[cxx_name = "B"]
                type A;
            }
        };
        assert!(cxx_qt_data
            .populate_cxx_mappings_from_item(&item, "")
            .is_ok());
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.len(), 1);
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.get("A").unwrap(), "B");
    }

    #[test]
    fn test_cxx_mappings_cxx_name_unsafe() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            unsafe extern "C++" {
                #[cxx_name = "B"]
                type A = C;
            }
        };
        assert!(cxx_qt_data
            .populate_cxx_mappings_from_item(&item, "")
            .is_ok());
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.len(), 1);
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.get("A").unwrap(), "B");
    }

    #[test]
    fn test_cxx_mappings_cxx_name_namespace_bridge() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            extern "C++" {
                type A;

                #[cxx_name = "C"]
                type B;
            }
        };
        assert!(cxx_qt_data
            .populate_cxx_mappings_from_item(&item, "bridge_namespace")
            .is_ok());
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.len(), 1);
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.get("B").unwrap(), "C");

        assert_eq!(cxx_qt_data.cxx_mappings.namespaces.len(), 2);
        assert_eq!(
            cxx_qt_data.cxx_mappings.namespaces.get("A").unwrap(),
            "bridge_namespace"
        );
        assert_eq!(
            cxx_qt_data.cxx_mappings.namespaces.get("B").unwrap(),
            "bridge_namespace"
        );
    }

    #[test]
    fn test_cxx_mappings_cxx_name_namespace_items() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            #[namespace = "extern_namespace"]
            extern "C++" {
                type A;

                #[namespace = "type_namespace"]
                type B;
            }
        };
        // Also ensure item namespace is chosen instead of bridge namespace
        assert!(cxx_qt_data
            .populate_cxx_mappings_from_item(&item, "namespace")
            .is_ok());
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.len(), 0);

        assert_eq!(cxx_qt_data.cxx_mappings.namespaces.len(), 2);
        assert_eq!(
            cxx_qt_data.cxx_mappings.namespaces.get("A").unwrap(),
            "extern_namespace"
        );
        assert_eq!(
            cxx_qt_data.cxx_mappings.namespaces.get("B").unwrap(),
            "type_namespace"
        );
    }

    #[test]
    fn test_cxx_mappings_cxx_name_normal_namespace_cxx_name() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            #[namespace = "extern_namespace"]
            extern "C++" {
                #[cxx_name = "B"]
                #[namespace = "type_namespace"]
                type A;

                #[cxx_name = "D"]
                type C;
            }
        };
        assert!(cxx_qt_data
            .populate_cxx_mappings_from_item(&item, "")
            .is_ok());
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.len(), 2);
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.get("A").unwrap(), "B");
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.get("C").unwrap(), "D");

        assert_eq!(cxx_qt_data.cxx_mappings.namespaces.len(), 2);
        assert_eq!(
            cxx_qt_data.cxx_mappings.namespaces.get("A").unwrap(),
            "type_namespace"
        );
        assert_eq!(
            cxx_qt_data.cxx_mappings.namespaces.get("C").unwrap(),
            "extern_namespace"
        );
    }

    #[test]
    fn test_cxx_mappings_shared_enum() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            #[namespace = "enum_namespace"]
            #[cxx_name = "EnumB"]
            enum EnumA {
                A,
            }
        };

        assert!(cxx_qt_data
            .populate_cxx_mappings_from_item(&item, "")
            .is_ok());
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.len(), 1);
        assert_eq!(
            cxx_qt_data.cxx_mappings.cxx_names.get("EnumA").unwrap(),
            "EnumB"
        );

        assert_eq!(cxx_qt_data.cxx_mappings.namespaces.len(), 1);
        assert_eq!(
            cxx_qt_data.cxx_mappings.namespaces.get("EnumA").unwrap(),
            "enum_namespace"
        );
    }

    #[test]
    fn test_cxx_mappings_shared_struct() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = parse_quote! {
            #[namespace = "struct_namespace"]
            #[cxx_name = "StructB"]
            struct StructA {
                field: i32,
            }
        };

        assert!(cxx_qt_data
            .populate_cxx_mappings_from_item(&item, "")
            .is_ok());
        assert_eq!(cxx_qt_data.cxx_mappings.cxx_names.len(), 1);
        assert_eq!(
            cxx_qt_data.cxx_mappings.cxx_names.get("StructA").unwrap(),
            "StructB"
        );

        assert_eq!(cxx_qt_data.cxx_mappings.namespaces.len(), 1);
        assert_eq!(
            cxx_qt_data.cxx_mappings.namespaces.get("StructA").unwrap(),
            "struct_namespace"
        );
    }

    #[test]
    fn test_parse_inherited_methods() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();

        let unsafe_block: Item = parse_quote! {
            #[cxx_qt::inherit]
            unsafe extern "C++" {
                fn test(self: &qobject::MyObject);

                fn with_args(self: &qobject::MyObject, arg: i32);
            }
        };
        let safe_block: Item = parse_quote! {
            #[cxx_qt::inherit]
            extern "C++" {
                #[cxx_name="withRename"]
                unsafe fn with_rename(self: Pin<&mut qobject::MyObject>, arg: i32);
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
            #[cxx_qt::qsignals]
            unsafe extern "C++" {
                fn ready(self: Pin<&mut qobject::MyObject>);

                #[cxx_name="cppDataChanged"]
                #[inherit]
                fn data_changed(self: Pin<&mut qobject::MyObject>, data: i32);
            }
        };
        cxxqtdata.parse_cxx_qt_item(block).unwrap();

        let qobject = cxxqtdata.qobjects.get(&qobject_ident()).unwrap();

        let signals = &qobject.signals;
        assert_eq!(signals.len(), 2);
        assert!(signals[0].mutable);
        assert!(signals[1].mutable);
        assert!(signals[0].safe);
        assert!(signals[1].safe);
        assert_eq!(signals[0].parameters.len(), 0);
        assert_eq!(signals[1].parameters.len(), 1);
        assert_eq!(signals[1].parameters[0].ident, "data");
        assert_eq!(
            signals[0].ident,
            CombinedIdent {
                cpp: format_ident!("ready"),
                rust: format_ident!("ready")
            }
        );
        assert_eq!(
            signals[1].ident,
            CombinedIdent {
                cpp: format_ident!("cppDataChanged"),
                rust: format_ident!("data_changed")
            }
        );
        assert!(!signals[0].inherit);
        assert!(signals[1].inherit);
    }

    #[test]
    fn test_parse_qsignals_unknown_obj() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();
        let block: Item = parse_quote! {
            #[cxx_qt::qsignals]
            unsafe extern "C++" {
                fn ready(self: Pin<&mut qobject::UnknownObj>);
            }
        };
        assert!(cxxqtdata.parse_cxx_qt_item(block).is_err());
    }

    #[test]
    fn test_parse_qsignals_unsafe() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();
        let block: Item = parse_quote! {
            #[cxx_qt::qsignals]
            extern "C++" {
                unsafe fn unsafe_signal(self: Pin<&mut qobject::MyObject>, arg: *mut T);
            }
        };
        cxxqtdata.parse_cxx_qt_item(block).unwrap();

        let qobject = cxxqtdata.qobjects.get(&qobject_ident()).unwrap();

        let signals = &qobject.signals;
        assert_eq!(signals.len(), 1);
        assert!(signals[0].mutable);
        assert!(!signals[0].safe);
        assert_eq!(signals[0].parameters.len(), 1);
        assert_eq!(signals[0].parameters[0].ident, "arg");
        assert_eq!(
            signals[0].ident,
            CombinedIdent {
                cpp: format_ident!("unsafeSignal"),
                rust: format_ident!("unsafe_signal")
            }
        );
        assert!(!signals[0].inherit);
    }

    #[test]
    fn test_parse_threading() {
        let mut cxxqtdata = create_parsed_cxx_qt_data();

        let qobject = cxxqtdata.qobjects.get(&qobject_ident()).unwrap();
        assert!(!qobject.threading);

        let threading_block: Item = parse_quote! {
            impl cxx_qt::Threading for qobject::MyObject {}
        };

        cxxqtdata.parse_cxx_qt_item(threading_block).unwrap();

        let qobject = cxxqtdata.qobjects.get(&qobject_ident()).unwrap();
        assert!(qobject.threading);
    }
}
