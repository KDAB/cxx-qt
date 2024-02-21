// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module provides utils for working with syn, CXX, and C++.
//!
//! Such as converting a [syn::Type] into a C++ string, or determining
//! if a type is unsafe in a CXX bridge.
//!
//! The idea of this module is that it should be independent as
//! these methods could be split out into a cxx-utils crate later on.

use std::collections::BTreeMap;

pub(crate) mod cpp;
pub(crate) mod rust;

use quote::format_ident;
use syn::{
    spanned::Spanned, token::Brace, Attribute, Ident, Item, ItemEnum, ItemForeignMod, ItemStruct,
    Path, Result,
};

use crate::syntax::{
    attribute::attribute_find_path, expr::expr_to_string,
    foreignmod::foreign_mod_to_foreign_item_types, path::path_from_idents,
};

use crate::parser::{cxxqtdata::ParsedCxxQtData, qenum::ParsedQEnum, qobject::ParsedQObject};

/// This struct contains all names a certain syntax element may have
///
/// This includes the rust_name, cxx_name, as well as qualifications like
/// the rust module and C++ namespace.
#[derive(Debug)]
pub struct Name {
    rust: Ident,
    cxx: Option<String>,
    module: Ident,
    namespace: Option<String>,
}

impl Name {
    fn from_ident_and_attrs(
        ident: &Ident,
        attrs: &[Attribute],
        parent_namespace: Option<&str>,
        module: &Ident,
    ) -> Result<Self> {
        // Find if there is a namespace (for C++ generation)
        let mut namespace = if let Some(index) = attribute_find_path(attrs, &["namespace"]) {
            Some(expr_to_string(
                &attrs[index].meta.require_name_value()?.value,
            )?)
        } else {
            parent_namespace.map(|namespace| namespace.to_owned())
        };

        // This is an important check as it allows for the namespace to be cleared by assigning an
        // empty namespace (i.e. #[namespace = ""])
        if let Some(namespace_name) = &namespace {
            if namespace_name.is_empty() {
                namespace = None;
            }
        }

        // Find if there is a cxx_name mapping (for C++ generation)
        let cxx_name = attribute_find_path(attrs, &["cxx_name"])
            .map(|index| -> Result<_> {
                expr_to_string(&attrs[index].meta.require_name_value()?.value)
            })
            .transpose()?;

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

        Ok(Self {
            rust: rust_ident,
            cxx: cxx_name,
            namespace,
            module: module.clone(),
        })
    }
}

/// The purpose of this struct is to store all nameable types.
///
/// This is used by the generator phase to find types by their identifier, such that they can be
/// fully qualified in Rust and C++.
#[derive(Default, Debug)]
pub struct TypeNames {
    names: BTreeMap<Ident, Name>,
}

impl TypeNames {
    /// The "Naming" phase.
    /// Extract all nameable types from the CXX-Qt data and the CXX items.
    ///
    /// This allows the generator to fully-qualify all types in the generated code.
    pub fn from_parsed_data(
        cxx_qt_data: &ParsedCxxQtData,
        cxx_items: &[Item],
        bridge_namespace: Option<&str>,
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
        bridge_namespace: Option<&str>,
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
        bridge_namespace: Option<&str>,
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
            self.insert_qobject(qobject, bridge_namespace, module_ident)?;
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
        _namespace: Option<&str>,
        module_ident: &Ident,
    ) -> Result<()> {
        let name = Name::from_ident_and_attrs(
            &qobject.qobject_ty.ident_left,
            &qobject.qobject_ty.attrs,
            Some(&qobject.namespace),
            module_ident,
        )?;
        self.names.insert(name.rust.clone(), name);
        Ok(())
    }

    fn populate_from_foreign_mod_item(
        &mut self,
        foreign_mod: &ItemForeignMod,
        bridge_namespace: Option<&str>,
        module_ident: &Ident,
    ) -> Result<()> {
        // Retrieve a namespace from the mod or the bridge
        let block_namespace =
            if let Some(index) = attribute_find_path(&foreign_mod.attrs, &["namespace"]) {
                Some(expr_to_string(
                    &foreign_mod.attrs[index].meta.require_name_value()?.value,
                )?)
            } else {
                bridge_namespace.map(str::to_owned)
            };

        // Read each of the types in the mod (type A;)
        for foreign_type in foreign_mod_to_foreign_item_types(foreign_mod)? {
            self.populate(
                &foreign_type.ident,
                &foreign_type.attrs,
                block_namespace.as_deref(),
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
        namespace: Option<&str>,
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
    pub fn cxx_qualified(&self, ident: &Ident) -> String {
        // Check if there is a cxx_name or namespace to handle
        let name = self.names.get(ident);

        if name.is_none() {
            return ident.to_string();
        }
        let name = name.unwrap();

        let cxx_name = name.cxx.clone().unwrap_or_else(|| name.rust.to_string());

        if let Some(namespace) = &name.namespace {
            format!("::{namespace}::{cxx_name}")
        } else {
            cxx_name.clone()
        }
    }

    /// For a given rust ident return the CXX name **without** its namespace
    pub fn cxx_unqualified(&self, ident: &Ident) -> Result<String> {
        if let Some(name) = self.names.get(ident) {
            Ok(name.cxx.clone().unwrap_or_else(|| ident.to_string()))
        } else {
            Err(syn::Error::new_spanned(ident, "Unknown type!"))
        }
    }

    /// For a given rust ident return the namespace if it's not empty
    pub fn namespace(&self, ident: &Ident) -> Option<String> {
        self.names
            .get(ident)
            .and_then(|name| name.namespace.clone())
    }

    /// Return a qualified version of the ident that can by used to refer to the type T outside of a CXX bridge
    ///
    /// Eg MyObject -> ffi::MyObject
    ///
    /// Note that this only handles types that are declared inside this bridge.
    /// E.g. UniquePtr -> cxx::UniquePtr isn't handled here.
    pub fn rust_qualified(&self, ident: &Ident) -> syn::Path {
        if let Some(name) = self.names.get(ident) {
            path_from_idents(&name.module, &name.rust)
        } else {
            Path::from(ident.clone())
        }
    }

    /// Helper which builds mappings from namespace, cxx_name, and rust_name attributes
    fn populate(
        &mut self,
        ident: &Ident,
        attrs: &[Attribute],
        parent_namespace: Option<&str>,
        module_ident: &Ident,
    ) -> Result<()> {
        let name = Name::from_ident_and_attrs(ident, attrs, parent_namespace, module_ident)?;
        self.names.insert(name.rust.clone(), name);
        Ok(())
    }

    #[cfg(test)]
    pub fn num_types(&self) -> usize {
        self.names.len()
    }

    #[cfg(test)]
    // This function only exists for testing, to allow mocking of the type names
    pub fn insert(
        &mut self,
        ident: &str,
        module: Option<Ident>,
        cxx_name: Option<&str>,
        namespace: Option<&str>,
    ) {
        let module = module.unwrap_or(format_ident!("A"));
        let name = Name {
            rust: format_ident!("{ident}"),
            cxx: cxx_name.map(str::to_owned),
            namespace: namespace.map(str::to_owned),
            module,
        };

        self.names.insert(name.rust.clone(), name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quote::format_ident;
    use syn::parse_quote;

    #[test]
    fn test_attribute_none() {
        let mut types = TypeNames::default();
        let ident = format_ident!("A");
        assert!(types
            .populate(&ident, &[], None, &format_ident!("ffi"))
            .is_ok());

        assert_eq!(types.num_types(), 1);
        assert_eq!(types.rust_qualified(&ident), parse_quote! { ffi::A });
        assert_eq!(types.cxx_qualified(&ident), "A"); // TODO Should this be "::A"?
        assert!(types.namespace(&ident).is_none());
    }

    #[test]
    fn test_attribute_cxx_name() {
        let mut types = TypeNames::default();
        let ident = format_ident!("A");
        assert!(types
            .populate(
                &ident,
                &[parse_quote! { #[cxx_name = "B"] }],
                None,
                &format_ident!("ffi")
            )
            .is_ok());

        assert_eq!(types.num_types(), 1);
        assert_eq!(types.cxx_qualified(&ident), "B");
        assert!(types.namespace(&ident).is_none());
        assert_eq!(types.rust_qualified(&ident), parse_quote! { ffi::A });
    }

    #[test]
    fn test_attribute_namespace() {
        let mut types = TypeNames::default();
        let ident = format_ident!("A");
        assert!(types
            .populate(
                &ident,
                &[parse_quote! { #[namespace = "type_namespace"] }],
                Some("bridge_namespace"),
                &format_ident!("ffi")
            )
            .is_ok());

        assert_eq!(types.num_types(), 1);
        assert_eq!(types.namespace(&ident), Some("type_namespace".to_owned()));
        assert_eq!(types.rust_qualified(&ident), parse_quote! { ffi::A });
    }

    #[test]
    fn test_attribute_rust_name() {
        let mut types = TypeNames::default();
        let ident = format_ident!("A");
        assert!(types
            .populate(
                &ident,
                &[parse_quote! { #[rust_name = "B"] }],
                None,
                &format_ident!("ffi")
            )
            .is_ok());

        assert_eq!(types.num_types(), 1);
        assert!(types.namespace(&ident).is_none());
        assert_eq!(
            types.rust_qualified(&format_ident!("B")),
            parse_quote! { ffi::B }
        );
    }

    #[test]
    fn test_parent_namespace() {
        let mut types = TypeNames::default();
        let ident = format_ident!("A");
        assert!(types
            .populate(&ident, &[], Some("bridge_namespace"), &format_ident!("ffi"))
            .is_ok());

        assert_eq!(types.cxx_qualified(&ident), "::bridge_namespace::A");
        assert_eq!(types.namespace(&ident).unwrap(), "bridge_namespace");
        assert_eq!(types.num_types(), 1);
        assert_eq!(
            types.rust_qualified(&format_ident!("A")),
            parse_quote! { ffi::A }
        );
    }

    #[test]
    fn test_qualified() {
        let mut types = TypeNames::default();
        let ident = format_ident!("A");
        assert!(types
            .populate(&ident, &[], None, &format_ident!("my_module"))
            .is_ok());

        assert!(types.namespace(&ident).is_none());
        assert_eq!(types.num_types(), 1);
        assert_eq!(types.rust_qualified(&ident), parse_quote! { my_module::A });
    }

    fn parse_cxx_item(item: Item) -> TypeNames {
        let mut type_names = TypeNames::default();
        assert!(type_names
            .populate_from_cxx_items(&[item], None, &format_ident!("ffi"))
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
        let ident = format_ident!("A");
        assert_eq!(type_names.num_types(), 1);
        assert_eq!(type_names.cxx_qualified(&ident), "B");

        assert_eq!(type_names.rust_qualified(&ident), parse_quote! { ffi::A });
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
        let mut types = TypeNames::default();
        assert!(types
            .populate_from_cxx_items(&items, Some("bridge_namespace"), &format_ident!("ffi"))
            .is_ok());

        assert_eq!(types.num_types(), 3);

        assert_eq!(
            &types.cxx_qualified(&format_ident!("A")),
            "::type_namespace::B"
        );
        assert_eq!(
            &types.cxx_qualified(&format_ident!("C")),
            "::extern_namespace::D"
        );
        assert_eq!(
            &types.cxx_qualified(&format_ident!("E")),
            "::bridge_namespace::E"
        );

        assert_eq!(
            types.namespace(&format_ident!("A")).unwrap(),
            "type_namespace"
        );
        assert_eq!(
            types.namespace(&format_ident!("C")).unwrap(),
            "extern_namespace"
        );
        assert_eq!(
            types.namespace(&format_ident!("E")).unwrap(),
            "bridge_namespace"
        );

        assert_eq!(
            types.rust_qualified(&format_ident!("A")),
            parse_quote! { ffi::A }
        );
        assert_eq!(
            types.rust_qualified(&format_ident!("C")),
            parse_quote! { ffi::C }
        );
        assert_eq!(
            types.rust_qualified(&format_ident!("E")),
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

        let ident = format_ident!("EnumA");
        let type_names = parse_cxx_item(item);

        assert_eq!(type_names.num_types(), 1);
        assert_eq!(type_names.cxx_unqualified(&ident).unwrap(), "EnumB");
        assert_eq!(type_names.namespace(&ident).unwrap(), "enum_namespace");
        assert_eq!(type_names.cxx_qualified(&ident), "::enum_namespace::EnumB");
        assert_eq!(
            type_names.rust_qualified(&ident),
            parse_quote! { ffi::EnumA }
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

        let ident = format_ident!("StructA");
        let types = parse_cxx_item(item);

        assert_eq!(types.num_types(), 1);
        assert_eq!(types.cxx_unqualified(&ident).unwrap(), "StructB");
        assert_eq!(types.cxx_qualified(&ident), "::struct_namespace::StructB");
        assert_eq!(types.namespace(&ident).unwrap(), "struct_namespace");
        assert_eq!(types.rust_qualified(&ident), parse_quote! { ffi::StructA });
    }
}
