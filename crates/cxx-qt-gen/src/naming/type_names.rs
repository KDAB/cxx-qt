// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

use quote::format_ident;
use syn::{
    parse_quote, token::Brace, Attribute, Error, Ident, Item, ItemEnum, ItemForeignMod, ItemStruct,
    Path, Result,
};

use crate::syntax::{
    attribute::attribute_find_path, expr::expr_to_string,
    foreignmod::foreign_mod_to_foreign_item_types,
};

use super::Name;
use crate::parser::{cxxqtdata::ParsedCxxQtData, qenum::ParsedQEnum, qobject::ParsedQObject};

/// The purpose of this struct is to store all nameable types.
///
/// This is used by the generator phase to find types by their identifier, such that they can be
/// fully qualified in Rust and C++.
#[derive(Debug)]
pub struct TypeNames {
    names: BTreeMap<Ident, Name>,
    // Currently, there is only a single case for which duplicate type names are allowed.
    // That is if a type is declared as a shared type, and declared as external.
    // So store the names of the types that are declared as shared and external, so we can
    // check for duplicates in all other cases.
    extern_types: BTreeSet<Ident>,
    shared_types: BTreeSet<Ident>,
}

impl Default for TypeNames {
    fn default() -> Self {
        let trivial = |rust_name, namespace: &str, cxx_name: &str| Name {
            rust: format_ident!("{}", rust_name),
            cxx: Some(cxx_name.to_owned()),
            module: None,
            namespace: Some(namespace.to_owned()),
        };

        // By default, CXX supports a set of types that don't need to be declared:
        // See: https://github.com/dtolnay/cxx/blob/master/syntax/atom.rs
        // and: https://github.com/dtolnay/cxx/blob/a6e1cd1e8d9d6df20e88e7443963dc4c5c8c4875/gen/src/write.rs#L1311
        let cxx_types = [
            Name {
                rust: format_ident!("bool"),
                module: None,
                cxx: None,
                namespace: None,
            },
            Name {
                rust: format_ident!("c_char"),
                module: Some(parse_quote! { ::std::ffi }),
                cxx: Some("char".to_owned()),
                namespace: None,
            },
            trivial("u8", "::std", "uint8_t"),
            trivial("u16", "::std", "uint16_t"),
            trivial("u32", "::std", "uint32_t"),
            trivial("u64", "::std", "uint64_t"),
            trivial("usize", "::std", "size_t"),
            trivial("i8", "::std", "int8_t"),
            trivial("i16", "::std", "int16_t"),
            trivial("i32", "::std", "int32_t"),
            trivial("i64", "::std", "int64_t"),
            trivial("isize", "::rust", "isize"),
            Name {
                rust: format_ident!("f32"),
                module: None,
                cxx: Some("float".to_owned()),
                namespace: None,
            },
            Name {
                rust: format_ident!("f64"),
                module: None,
                cxx: Some("double".to_owned()),
                namespace: None,
            },
            Name {
                rust: format_ident!("CxxString"),
                module: Some(parse_quote! { ::cxx }),
                cxx: Some("string".to_owned()),
                namespace: Some("::std".to_owned()),
            },
            Name {
                rust: format_ident!("String"),
                module: None,
                cxx: None,
                namespace: Some("::rust".to_owned()),
            },
        ];

        let mut this = Self {
            names: BTreeMap::default(),
            extern_types: BTreeSet::default(),
            shared_types: BTreeSet::default(),
        };
        for name in cxx_types {
            this.names.insert(name.rust.clone(), name);
        }
        this
    }
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
            self.insert_qobject(qobject)?;
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

    fn insert_qobject(&mut self, qobject: &ParsedQObject) -> Result<()> {
        self.names
            .insert(qobject.name.rust.clone(), qobject.name.clone());
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
            self.populate_or_else(
                &foreign_type.ident,
                &foreign_type.attrs,
                block_namespace.as_deref(),
                module_ident,
                |this, name| {
                    // Shared types can be declared as external, in which case duplicates
                    // are allowed.
                    // TODO: Check that we're in `extern "C++"` and not `extern "Rust"`
                    if !this.shared_types.contains(&name.rust)
                        || this.extern_types.contains(&name.rust)
                    {
                        return Err(this.duplicate_type(&name.rust));
                    }
                    this.check_duplicate_compatability(&name)
                },
            )?;

            self.extern_types.insert(foreign_type.ident.clone());
        }

        Ok(())
    }

    fn check_duplicate_compatability(&self, duplicate: &Name) -> Result<()> {
        if Some(duplicate) != self.names.get(&duplicate.rust) {
            Err(Error::new_spanned(
                &duplicate.rust,
                format!(
                    "The type `{}` is defined multiple times with different mappings",
                    duplicate.rust
                ),
            ))
        } else {
            Ok(())
        }
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
                self.populate_or_else(ident, attrs, namespace, module_ident, |this, name| {
                    // Shared types may appear twice in the bridge, but only together
                    // with a declaration of the same external type.
                    if !this.extern_types.contains(&name.rust)
                        || this.shared_types.contains(&name.rust)
                    {
                        return Err(this.duplicate_type(&name.rust));
                    }
                    this.check_duplicate_compatability(&name)
                })?;
                self.shared_types.insert(ident.clone());
            }
            _others => {}
        }

        // If there is a foreign mod then process it
        if let Item::ForeignMod(foreign_mod) = &item {
            self.populate_from_foreign_mod_item(foreign_mod, namespace, module_ident)?;
        }

        Ok(())
    }

    fn unknown_type(&self, ident: &Ident) -> Error {
        Error::new_spanned(ident, format!("Undeclared type: `{ident}`!"))
    }

    /// For a given rust ident return the CXX name with its namespace
    ///
    /// Ideally we'd want this type name to always be **fully** qualified, starting with `::`.
    /// Unfortunately, this isn't always possible, as the Qt5 meta object system doesn't register
    /// types with the fully qualified path :(
    /// E.g. it will recognize `QString`, but not `::QString` from QML.
    ///
    /// This needs to be considered in many places (properties, signals, invokables, etc.)
    /// Therefore, for now we'll use the qualified, but not fully qualified version of `namespace::type`.
    /// This should work in most cases, but it's not perfect.
    pub fn cxx_qualified(&self, ident: &Ident) -> Result<String> {
        // Check if there is a cxx_name or namespace to handle
        let name = self
            .names
            .get(ident)
            .ok_or_else(|| self.unknown_type(ident))?;

        let cxx_name = name.cxx_unqualified();

        if let Some(namespace) = &name.namespace {
            Ok(format!("{namespace}::{cxx_name}"))
        } else {
            Ok(cxx_name)
        }
    }

    /// For a given rust ident return the CXX name **without** its namespace
    pub fn cxx_unqualified(&self, ident: &Ident) -> Result<String> {
        self.names
            .get(ident)
            .ok_or_else(|| self.unknown_type(ident))
            .map(Name::cxx_unqualified)
    }

    /// For a given rust ident return the namespace if it's not empty
    pub fn namespace(&self, ident: &Ident) -> Result<Option<String>> {
        self.names
            .get(ident)
            .ok_or_else(|| self.unknown_type(ident))
            .map(|name| name.namespace.clone())
    }

    /// Return a qualified version of the ident that can by used to refer to the type T outside of a CXX bridge
    ///
    /// Eg MyObject -> ffi::MyObject
    ///
    /// Note that this only handles types that are declared inside this bridge.
    /// E.g. UniquePtr -> cxx::UniquePtr isn't handled here.
    pub fn rust_qualified(&self, ident: &Ident) -> Result<Path> {
        self.names
            .get(ident)
            .ok_or_else(|| self.unknown_type(ident))
            .map(|name| {
                if let Some(module) = &name.module {
                    let mut qualified_ident = module.clone();
                    qualified_ident.segments.push(name.rust.clone().into());
                    qualified_ident
                } else {
                    Path::from(name.rust.clone())
                }
            })
    }

    fn duplicate_type(&self, ident: &Ident) -> Error {
        Error::new_spanned(
            ident,
            format!("The type name `{ident}` is defined multiple times"),
        )
    }

    /// Helper which builds mappings from namespace, cxx_name, and rust_name attributes
    fn populate(
        &mut self,
        ident: &Ident,
        attrs: &[Attribute],
        parent_namespace: Option<&str>,
        module_ident: &Ident,
    ) -> Result<()> {
        self.populate_or_else(
            ident,
            attrs,
            parent_namespace,
            module_ident,
            |this, name| Err(this.duplicate_type(&name.rust)),
        )
    }

    fn populate_or_else(
        &mut self,
        ident: &Ident,
        attrs: &[Attribute],
        parent_namespace: Option<&str>,
        module_ident: &Ident,
        fallback: impl FnOnce(&mut Self, Name) -> Result<()>,
    ) -> Result<()> {
        let name = Name::from_ident_and_attrs(ident, attrs, parent_namespace, module_ident)?;

        let entry = self.names.entry(name.rust.clone());

        match entry {
            Entry::Occupied(_) => fallback(self, name),
            Entry::Vacant(entry) => {
                entry.insert(name);
                Ok(())
            }
        }
    }

    #[cfg(test)]
    pub fn num_types(&self) -> usize {
        self.names.len()
    }

    #[cfg(test)]
    // Only for testing, return a TypeNames struct that contains a qobject::MyObject
    pub fn mock() -> Self {
        let mut this = Self::default();
        this.insert("MyObject", Some(format_ident!("qobject")), None, None);
        this
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
        let module = module.map(Path::from);
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
    fn test_unknown_type() {
        let types = TypeNames::default();
        // By default, 16 types are known to CXX
        assert_eq!(types.num_types(), 16);

        assert!(types.cxx_unqualified(&format_ident!("A")).is_err());
        assert!(types.cxx_qualified(&format_ident!("A")).is_err());
        assert!(types.namespace(&format_ident!("A")).is_err());
    }

    #[test]
    fn test_attribute_none() {
        let mut types = TypeNames::default();
        let ident = format_ident!("A");
        assert!(types
            .populate(&ident, &[], None, &format_ident!("ffi"))
            .is_ok());

        assert_eq!(types.num_types(), 17);
        assert_eq!(
            types.rust_qualified(&ident).unwrap(),
            parse_quote! { ffi::A }
        );
        assert_eq!(types.cxx_qualified(&ident).unwrap(), "A");
        assert!(types.namespace(&ident).unwrap().is_none());
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

        assert_eq!(types.num_types(), 17);
        assert_eq!(types.cxx_qualified(&ident).unwrap(), "B");
        assert!(types.namespace(&ident).unwrap().is_none());
        assert_eq!(
            types.rust_qualified(&ident).unwrap(),
            parse_quote! { ffi::A }
        );
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

        assert_eq!(types.num_types(), 17);
        assert_eq!(
            types.namespace(&ident).unwrap(),
            Some("type_namespace".to_owned())
        );
        assert_eq!(
            types.rust_qualified(&ident).unwrap(),
            parse_quote! { ffi::A }
        );
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

        assert_eq!(types.num_types(), 17);
        // The rust_name must be used as the key to the TypeNames struct, otherwise most methods
        // return an error.
        assert!(types.cxx_unqualified(&ident).is_err());
        assert!(types.namespace(&ident).is_err());

        let rust_ident = &format_ident!("B");
        assert_eq!(
            types.rust_qualified(rust_ident).unwrap(),
            parse_quote! { ffi::B }
        );
        assert_eq!(types.cxx_unqualified(rust_ident).unwrap(), "A");
        assert!(types.namespace(rust_ident).unwrap().is_none());
    }

    #[test]
    fn test_parent_namespace() {
        let mut types = TypeNames::default();
        let ident = format_ident!("A");
        assert!(types
            .populate(&ident, &[], Some("bridge_namespace"), &format_ident!("ffi"))
            .is_ok());

        assert_eq!(types.cxx_qualified(&ident).unwrap(), "bridge_namespace::A");
        assert_eq!(
            types.namespace(&ident).unwrap().unwrap(),
            "bridge_namespace"
        );
        assert_eq!(types.num_types(), 17);
        assert_eq!(
            types.rust_qualified(&format_ident!("A")).unwrap(),
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

        assert!(types.namespace(&ident).unwrap().is_none());
        assert_eq!(types.num_types(), 17);
        assert_eq!(
            types.rust_qualified(&ident).unwrap(),
            parse_quote! { my_module::A }
        );
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
        assert_eq!(type_names.num_types(), 17);
        assert_eq!(type_names.cxx_qualified(&ident).unwrap(), "B");

        assert_eq!(
            type_names.rust_qualified(&ident).unwrap(),
            parse_quote! { ffi::A }
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
        let mut types = TypeNames::default();
        assert!(types
            .populate_from_cxx_items(&items, Some("bridge_namespace"), &format_ident!("ffi"))
            .is_ok());

        assert_eq!(types.num_types(), 19);

        assert_eq!(
            types.cxx_qualified(&format_ident!("A")).unwrap(),
            "type_namespace::B"
        );
        assert_eq!(
            types.cxx_qualified(&format_ident!("C")).unwrap(),
            "extern_namespace::D"
        );
        assert_eq!(
            types.cxx_qualified(&format_ident!("E")).unwrap(),
            "bridge_namespace::E"
        );

        assert_eq!(
            types.namespace(&format_ident!("A")).unwrap().unwrap(),
            "type_namespace"
        );
        assert_eq!(
            types.namespace(&format_ident!("C")).unwrap().unwrap(),
            "extern_namespace"
        );
        assert_eq!(
            types.namespace(&format_ident!("E")).unwrap().unwrap(),
            "bridge_namespace"
        );

        assert_eq!(
            types.rust_qualified(&format_ident!("A")).unwrap(),
            parse_quote! { ffi::A }
        );
        assert_eq!(
            types.rust_qualified(&format_ident!("C")).unwrap(),
            parse_quote! { ffi::C }
        );
        assert_eq!(
            types.rust_qualified(&format_ident!("E")).unwrap(),
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

        assert_eq!(type_names.num_types(), 17);
        assert_eq!(type_names.cxx_unqualified(&ident).unwrap(), "EnumB");
        assert_eq!(
            type_names.namespace(&ident).unwrap().unwrap(),
            "enum_namespace"
        );
        assert_eq!(
            type_names.cxx_qualified(&ident).unwrap(),
            "enum_namespace::EnumB"
        );
        assert_eq!(
            type_names.rust_qualified(&ident).unwrap(),
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

        assert_eq!(types.num_types(), 17);
        assert_eq!(types.cxx_unqualified(&ident).unwrap(), "StructB");
        assert_eq!(
            types.cxx_qualified(&ident).unwrap(),
            "struct_namespace::StructB"
        );
        assert_eq!(
            types.namespace(&ident).unwrap().unwrap(),
            "struct_namespace"
        );
        assert_eq!(
            types.rust_qualified(&ident).unwrap(),
            parse_quote! { ffi::StructA }
        );
    }

    #[test]
    fn test_duplicate_types() {
        let items = [
            parse_quote! {
                extern "C++" {
                    #[rust_name="B"]
                    type A;
                }
            },
            parse_quote! {
                extern "Rust" {
                    type B;
                }
            },
        ];

        let mut types = TypeNames::default();
        assert!(types
            .populate_from_cxx_items(&items, None, &format_ident!("ffi"))
            .is_err());
    }

    #[test]
    fn test_extern_shared_type() {
        let items = [
            parse_quote! {
                enum A { }
            },
            parse_quote! {
                extern "C++" {
                    type A;
                }
            },
            parse_quote! {
                struct B { }
            },
            parse_quote! {
                extern "C++" {
                    type B;
                }
            },
        ];
        let mut types = TypeNames::default();
        assert!(types
            .populate_from_cxx_items(&items, None, &format_ident!("ffi"))
            .is_ok());
    }

    #[test]
    fn test_extern_shared_type_incompatible() {
        let items = [
            parse_quote! {
                #[namespace="X"]
                enum A { }
            },
            parse_quote! {
                extern "C++" {
                    type A;
                }
            },
        ];

        let mut types = TypeNames::default();
        assert!(types
            .populate_from_cxx_items(&items, None, &format_ident!("ffi"))
            .is_err());
    }
}
