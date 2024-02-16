// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeMap;

use quote::format_ident;
use syn::{
    spanned::Spanned, token::Brace, Attribute, Ident, Item, ItemEnum, ItemForeignMod, ItemStruct,
    Path, Result,
};

use crate::syntax::{
    attribute::attribute_find_path, expr::expr_to_string,
    foreignmod::foreign_mod_to_foreign_item_types, path::path_from_idents,
};

use super::{cxxqtdata::ParsedCxxQtData, qenum::ParsedQEnum, qobject::ParsedQObject};

/// The purpose of this struct is to store all nameable types.
///
/// This is used by the generator phase to find types by their identifier, such that they can be
/// fully qualified in Rust and C++.
#[derive(Default, Debug)]
pub struct TypeNames {
    /// Map of the cxx_name of any types defined in CXX extern blocks
    ///
    /// This is used in the C++ generation to map the Rust type name to the C++ name
    cxx_names: BTreeMap<String, String>,
    /// Map of the namespace of any types or methods defined in CXX extern blocks
    ///
    /// This is used in the C++ generation to map the Rust type name to the C++ name
    namespaces: BTreeMap<String, String>,
    /// Mappings for CXX types when used outside the bridge
    ///
    /// This is used in the Rust generation to map the bridge type A to ffi::B
    qualified: BTreeMap<Ident, Path>,
}

impl TypeNames {
    /// The "Naming" phase.
    /// Extract all nameable types from the CXX-Qt data and the CXX items.
    ///
    /// This allows the generator to fully-qualify all types in the generated code.
    pub fn from_parsed_data(
        cxx_qt_data: &ParsedCxxQtData,
        cxx_items: &[Item],
        bridge_namespace: &str,
        module_ident: &Ident,
    ) -> Result<Self> {
        let mut type_names = Self::default();

        type_names.populate_from_cxx_items(cxx_items, bridge_namespace, module_ident)?;
        type_names.populate_from_cxx_qt_data(cxx_qt_data, bridge_namespace, module_ident)?;

        Ok(type_names)
    }

    fn populate_from_cxx_items(
        &mut self,
        cxx_items: &[Item],
        bridge_namespace: &str,
        module_ident: &Ident,
    ) -> Result<()> {
        // Load any CXX name mappings
        for item in cxx_items {
            self.populate_from_item(item, bridge_namespace, module_ident)?;
        }
        Ok(())
    }

    fn populate_from_cxx_qt_data(
        &mut self,
        cxx_qt_data: &ParsedCxxQtData,
        bridge_namespace: &str,
        module_ident: &Ident,
    ) -> Result<()> {
        let populate_qenum = |type_names: &mut TypeNames, qenum: &ParsedQEnum| {
            type_names.populate(
                &qenum.ident,
                &qenum.item.attrs,
                bridge_namespace,
                module_ident,
            )
        };

        for qobject in cxx_qt_data.qobjects.values() {
            self.insert_qobject(qobject, bridge_namespace, module_ident);
            for qenum in &qobject.qenums {
                populate_qenum(self, qenum)?;
            }
        }

        for qenum in &cxx_qt_data.qenums {
            populate_qenum(self, qenum)?;
        }

        for extern_cxxqt in &cxx_qt_data.extern_cxxqt_blocks {
            // TODO: Refactor, this is a hack to reconstruct the original ItemForeignMod
            let foreign_mod = ItemForeignMod {
                attrs: extern_cxxqt.attrs.clone(),
                unsafety: None,
                brace_token: Brace::default(),
                items: extern_cxxqt.passthrough_items.clone(),
                abi: syn::Abi {
                    extern_token: syn::parse_quote!(extern),
                    name: None,
                },
            };
            self.populate_from_foreign_mod_item(&foreign_mod, bridge_namespace, module_ident)?;
        }

        Ok(())
    }

    fn insert_qobject(
        &mut self,
        qobject: &ParsedQObject,
        // The QObject is parsed weirdly
        // It will self-assign the bridge namespace if it doesn't have one itself.
        // TODO: Fix this and do that in the naming phase
        _namespace: &str,
        module_ident: &Ident,
    ) {
        self.qualified.insert(
            qobject.qobject_ty.ident_left.clone(),
            path_from_idents(module_ident, &qobject.qobject_ty.ident_left),
        );

        if !qobject.namespace.is_empty() {
            self.namespaces.insert(
                qobject.qobject_ty.ident_left.to_string(),
                qobject.namespace.clone(),
            );
        }
    }

    fn populate_from_foreign_mod_item(
        &mut self,
        foreign_mod: &ItemForeignMod,
        bridge_namespace: &str,
        module_ident: &Ident,
    ) -> Result<()> {
        // Retrieve a namespace from the mod or the bridge
        let block_namespace =
            if let Some(index) = attribute_find_path(&foreign_mod.attrs, &["namespace"]) {
                expr_to_string(&foreign_mod.attrs[index].meta.require_name_value()?.value)?
            } else {
                bridge_namespace.to_owned()
            };

        // Read each of the types in the mod (type A;)
        for foreign_type in foreign_mod_to_foreign_item_types(foreign_mod)? {
            self.populate(
                &foreign_type.ident,
                &foreign_type.attrs,
                &block_namespace,
                module_ident,
            )?;
        }

        Ok(())
    }

    /// Add this item to the available types.
    ///
    /// This will also add any remappings from the attributes (i.e. cxx_name, rust_name,
    /// namespace, etc.)
    fn populate_from_item(
        &mut self,
        item: &Item,
        namespace: &str,
        module_ident: &Ident,
    ) -> Result<()> {
        // Consider if shared types have mappings
        match item {
            Item::Enum(ItemEnum { attrs, ident, .. })
            | Item::Struct(ItemStruct { attrs, ident, .. }) => {
                self.populate(ident, attrs, namespace, module_ident)?;
            }
            _others => {}
        }

        // If there is a foreign mod then process it
        if let Item::ForeignMod(foreign_mod) = &item {
            self.populate_from_foreign_mod_item(foreign_mod, namespace, module_ident)?;
        }

        Ok(())
    }

    /// For a given rust ident return the CXX name with its namespace
    pub fn cxx_qualified(&self, ident: &str) -> String {
        // Check if there is a cxx_name or namespace to handle
        let cxx_name = self
            .cxx_names
            .get(ident)
            .cloned()
            .unwrap_or_else(|| ident.to_owned());

        if let Some(namespace) = self.namespace(ident) {
            format!("::{namespace}::{cxx_name}")
        } else {
            cxx_name
        }
    }

    /// For a given rust ident return the namespace if it's not empty
    pub fn namespace(&self, ident: &str) -> Option<String> {
        self.namespaces
            .get(ident)
            .filter(|namespace| !namespace.is_empty())
            .cloned()
    }

    /// Return a qualified version of the ident that can by used to refer to the type T outside of a CXX bridge
    ///
    /// Eg MyObject -> ffi::MyObject
    ///
    /// Note that this only handles types that are declared inside this bridge.
    /// E.g. UniquePtr -> cxx::UniquePtr isn't handled here.
    pub fn rust_qualified(&self, ident: &Ident) -> syn::Path {
        if let Some(qualified_path) = self.qualified.get(ident) {
            qualified_path.clone()
        } else {
            Path::from(ident.clone())
        }
    }

    /// Helper which builds mappings from namespace, cxx_name, and rust_name attributes
    pub fn populate(
        &mut self,
        ident: &Ident,
        attrs: &[Attribute],
        parent_namespace: &str,
        module_ident: &Ident,
    ) -> Result<()> {
        // Find if there is a namespace (for C++ generation)
        let namespace = if let Some(index) = attribute_find_path(attrs, &["namespace"]) {
            expr_to_string(&attrs[index].meta.require_name_value()?.value)?
        } else {
            parent_namespace.to_string()
        };

        if !namespace.is_empty() {
            self.namespaces.insert(ident.to_string(), namespace);
        }

        // Find if there is a cxx_name mapping (for C++ generation)
        if let Some(index) = attribute_find_path(attrs, &["cxx_name"]) {
            self.cxx_names.insert(
                ident.to_string(),
                expr_to_string(&attrs[index].meta.require_name_value()?.value)?,
            );
        }

        // Find if there is a rust_name mapping
        let rust_ident = if let Some(index) = attribute_find_path(attrs, &["rust_name"]) {
            format_ident!(
                "{}",
                expr_to_string(&attrs[index].meta.require_name_value()?.value)?,
                span = attrs[index].span()
            )
        } else {
            ident.clone()
        };

        // Add the rust_ident to qualified mappings (for Rust generation using ffi::T)
        self.qualified
            .insert(ident.clone(), path_from_idents(module_ident, &rust_ident));

        Ok(())
    }

    #[cfg(test)]
    pub fn num_types(&self) -> usize {
        self.qualified.len()
    }

    #[cfg(test)]
    // This function only exists for testing, to allow mocking of the type names
    pub fn insert(
        &mut self,
        ident: &str,
        qualified: Option<Path>,
        cxx_name: Option<&str>,
        namespace: Option<&str>,
    ) {
        if let Some(qualified) = qualified {
            self.qualified.insert(format_ident!("{}", ident), qualified);
        }
        if let Some(cxx_name) = cxx_name {
            self.cxx_names
                .insert(ident.to_string(), cxx_name.to_string());
        }
        if let Some(namespace) = namespace {
            self.namespaces
                .insert(ident.to_string(), namespace.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quote::format_ident;
    use syn::parse_quote;

    #[test]
    fn test_attribute_none() {
        let mut mappings = TypeNames::default();
        assert!(mappings
            .populate(&format_ident!("A"), &[], "", &format_ident!("ffi"))
            .is_ok());

        assert!(mappings.cxx_names.is_empty());
        assert!(mappings.namespaces.is_empty());
        assert_eq!(mappings.qualified.len(), 1);
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { ffi::A }
        );
        assert!(mappings.namespace("A").is_none());
    }

    #[test]
    fn test_attribute_cxx_name() {
        let mut mappings = TypeNames::default();
        assert!(mappings
            .populate(
                &format_ident!("A"),
                &[parse_quote! { #[cxx_name = "B"] }],
                "",
                &format_ident!("ffi")
            )
            .is_ok());

        assert_eq!(mappings.cxx_names.len(), 1);
        assert_eq!(mappings.cxx_names.get("A").unwrap(), "B");
        assert!(mappings.namespaces.is_empty());
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { ffi::A }
        );
    }

    #[test]
    fn test_attribute_namespace() {
        let mut mappings = TypeNames::default();
        assert!(mappings
            .populate(
                &format_ident!("A"),
                &[parse_quote! { #[namespace = "type_namespace"] }],
                "bridge_namespace",
                &format_ident!("ffi")
            )
            .is_ok());

        assert!(mappings.cxx_names.is_empty());
        assert_eq!(mappings.namespaces.get("A").unwrap(), "type_namespace");
        assert_eq!(mappings.qualified.len(), 1);
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { ffi::A }
        );
        assert_eq!(mappings.namespace("A"), Some("type_namespace".to_owned()));
    }

    #[test]
    fn test_attribute_rust_name() {
        let mut mappings = TypeNames::default();
        assert!(mappings
            .populate(
                &format_ident!("A"),
                &[parse_quote! { #[rust_name = "B"] }],
                "",
                &format_ident!("ffi")
            )
            .is_ok());

        assert!(mappings.cxx_names.is_empty());
        assert!(mappings.namespaces.is_empty());
        assert_eq!(mappings.qualified.len(), 1);
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { ffi::B }
        );
    }

    #[test]
    fn test_parent_namespace() {
        let mut mappings = TypeNames::default();
        assert!(mappings
            .populate(
                &format_ident!("A"),
                &[],
                "bridge_namespace",
                &format_ident!("ffi")
            )
            .is_ok());

        assert!(mappings.cxx_names.is_empty());
        assert_eq!(mappings.namespaces.get("A").unwrap(), "bridge_namespace");
        assert_eq!(mappings.qualified.len(), 1);
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { ffi::A }
        );
        assert_eq!(mappings.namespace("A"), Some("bridge_namespace".to_owned()));
    }

    #[test]
    fn test_qualified() {
        let mut mappings = TypeNames::default();
        assert!(mappings
            .populate(&format_ident!("A"), &[], "", &format_ident!("my_module"))
            .is_ok());

        assert!(mappings.cxx_names.is_empty());
        assert!(mappings.namespaces.is_empty());
        assert_eq!(mappings.qualified.len(), 1);
        assert_eq!(
            mappings.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { my_module::A }
        );
    }

    fn parse_cxx_item(item: Item) -> TypeNames {
        let mut type_names = TypeNames::default();
        assert!(type_names
            .populate_from_cxx_items(&[item], "", &format_ident!("ffi"))
            .is_ok());
        type_names
    }

    #[test]
    fn test_cxx_items_cxx_name() {
        // TODO
        let item: Item = parse_quote! {
            unsafe extern "C++" {
                #[cxx_name = "B"]
                type A = C;
            }
        };

        let type_names = parse_cxx_item(item);
        assert_eq!(type_names.cxx_names.len(), 1);
        assert_eq!(type_names.cxx_names.get("A").unwrap(), "B");

        assert_eq!(type_names.qualified.len(), 1);
        assert_eq!(
            type_names.qualified.get(&format_ident!("A")).unwrap(),
            &parse_quote! { ffi::A }
        );
    }

    #[test]
    fn test_cxx_items_namespacing() {
        let items: [Item; 2] = [
            parse_quote! {
                #[namespace = "extern_namespace"]
                extern "C++" {
                    #[cxx_name = "B"]
                    #[namespace = "type_namespace"]
                    type A;

                    #[cxx_name = "D"]
                    type C;
                }
            },
            parse_quote! {
                extern "C++" {
                    type E;
                }
            },
        ];
        let mut type_names = TypeNames::default();
        assert!(type_names
            .populate_from_cxx_items(&items, "bridge_namespace", &format_ident!("ffi"))
            .is_ok());

        assert_eq!(type_names.num_types(), 3);

        assert_eq!(type_names.cxx_qualified("A"), "::type_namespace::B");
        assert_eq!(type_names.cxx_qualified("C"), "::extern_namespace::D");
        assert_eq!(type_names.cxx_qualified("E"), "::bridge_namespace::E");

        assert_eq!(type_names.namespace("A").unwrap(), "type_namespace");
        assert_eq!(type_names.namespace("C").unwrap(), "extern_namespace");
        assert_eq!(type_names.namespace("E").unwrap(), "bridge_namespace");

        assert_eq!(
            type_names.rust_qualified(&format_ident!("A")),
            parse_quote! { ffi::A }
        );
        assert_eq!(
            type_names.rust_qualified(&format_ident!("C")),
            parse_quote! { ffi::C }
        );
        assert_eq!(
            type_names.rust_qualified(&format_ident!("E")),
            parse_quote! { ffi::E }
        );
    }

    #[test]
    fn test_cxx_items_shared_enum() {
        let item: Item = parse_quote! {
            #[namespace = "enum_namespace"]
            #[cxx_name = "EnumB"]
            enum EnumA {
                A,
            }
        };

        let type_names = parse_cxx_item(item);

        assert_eq!(type_names.cxx_names.len(), 1);
        assert_eq!(type_names.cxx_names.get("EnumA").unwrap(), "EnumB");

        assert_eq!(type_names.namespaces.len(), 1);
        assert_eq!(
            type_names.namespaces.get("EnumA").unwrap(),
            "enum_namespace"
        );

        assert_eq!(type_names.qualified.len(), 1);
        assert_eq!(
            type_names.qualified.get(&format_ident!("EnumA")).unwrap(),
            &parse_quote! { ffi::EnumA }
        );
    }

    #[test]
    fn test_cxx_items_shared_struct() {
        let item: Item = parse_quote! {
            #[namespace = "struct_namespace"]
            #[cxx_name = "StructB"]
            struct StructA {
                field: i32,
            }
        };

        let type_names = parse_cxx_item(item);

        assert_eq!(type_names.cxx_names.len(), 1);
        assert_eq!(type_names.cxx_names.get("StructA").unwrap(), "StructB");

        assert_eq!(type_names.namespaces.len(), 1);
        assert_eq!(
            type_names.namespaces.get("StructA").unwrap(),
            "struct_namespace"
        );

        assert_eq!(type_names.qualified.len(), 1);
        assert_eq!(
            type_names.qualified.get(&format_ident!("StructA")).unwrap(),
            &parse_quote! { ffi::StructA }
        );
    }
}
