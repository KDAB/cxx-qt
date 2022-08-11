// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::{property::ParsedQProperty, signals::ParsedSignalsEnum};
use crate::syntax::{attribute::attribute_find_path, fields::fields_to_named_fields_mut};
use syn::{
    spanned::Spanned, Error, Fields, ImplItem, ImplItemMethod, Item, ItemImpl, ItemStruct, Result,
};

/// A representation of a QObject within a CXX-Qt [syn::ItemMod]
///
/// This has initial splitting of [syn::Item]'s into relevant blocks, other phases will
/// then mutate these [syn::Item]'s for generation purposes.
#[derive(Default)]
pub struct ParsedQObject {
    /// Data struct that currently stores the properties for the QObject
    ///
    /// In the future this will be removed
    pub data_struct: Option<ItemStruct>,
    /// QObject struct that stores the invokables for the QObject
    pub qobject_struct: Option<ItemStruct>,
    /// Representation of the Signals enum that defines the Q_SIGNALS for the QObject
    pub signals: Option<ParsedSignalsEnum>,
    /// List of invokables that need to be implemented on the C++ object in Rust
    ///
    /// These will also be exposed as Q_INVOKABLE on the C++ object
    pub invokables: Vec<ImplItemMethod>,
    /// List of methods that need to be implemented on the C++ object in Rust
    ///
    /// Note that they will only be visible on the Rust side
    pub methods: Vec<ImplItemMethod>,
    /// List of properties that need to be implemented on the C++ object
    ///
    /// These will be exposed as Q_PROPERTY on the C++ object
    pub properties: Vec<ParsedQProperty>,
    /// Update request handler for the QObject
    ///
    /// In the future this may be removed
    pub update_requester_handler: Option<ItemImpl>,
    /// Items that we don't need to generate anything for CXX or C++
    /// eg impls on the Rust object or Default implementations
    pub others: Vec<Item>,
}

impl ParsedQObject {
    /// Extract all methods (both invokable and non-invokable) from [syn::ImplItem]'s from each Impl block
    ///
    /// These will have come from a impl cxx_qt::QObject<T> block
    pub fn parse_impl_items(&mut self, items: &[ImplItem]) -> Result<()> {
        for item in items {
            // Check if this item is a method
            if let ImplItem::Method(method) = item {
                // Determine if this method is an invokable
                if let Some(index) = attribute_find_path(&method.attrs, &["qinvokable"]) {
                    // Remove the invokable attribute
                    let mut invokable = method.clone();
                    invokable.attrs.remove(index);
                    self.invokables.push(invokable);
                } else {
                    self.methods.push(method.clone());
                }
            } else {
                return Err(Error::new(item.span(), "Only methods are supported."));
            };
        }

        Ok(())
    }

    /// Extract all the properties from [syn::Fields] from a [syn::ItemStruct]
    pub fn parse_struct_fields(&mut self, fields: &mut Fields) -> Result<()> {
        for field in fields_to_named_fields_mut(fields)? {
            // Try to find any properties defined within the struct
            if let Some(index) = attribute_find_path(&field.attrs, &["qproperty"]) {
                // Remove the #[qproperty] attribute
                field.attrs.remove(index);

                self.properties.push(ParsedQProperty {
                    ident: field.ident.clone().unwrap(),
                    ty: field.ty.clone(),
                    vis: field.vis.clone(),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::tests::f64_type;
    use crate::tests::tokens_to_syn;
    use quote::quote;
    use syn::{ItemImpl, Visibility};

    #[test]
    fn test_parse_impl_items_valid() {
        let mut qobject = ParsedQObject::default();
        let item: ItemImpl = tokens_to_syn(quote! {
            impl T {
                #[qinvokable]
                fn invokable() {}

                fn cpp_context() {}
            }
        });
        assert!(qobject.parse_impl_items(&item.items).is_ok());
        assert_eq!(qobject.invokables.len(), 1);
        assert_eq!(qobject.methods.len(), 1);
    }

    #[test]
    fn test_parse_impl_items_invalid() {
        let mut qobject = ParsedQObject::default();
        let item: ItemImpl = tokens_to_syn(quote! {
            impl T {
                const VALUE: i32 = 1;

                macro_code!();

                type A = i32;

                #[qinvokable]
                fn invokable() {}

                fn cpp_context() {}
            }
        });
        assert!(qobject.parse_impl_items(&item.items).is_err());
    }

    #[test]
    fn test_parse_struct_fields_valid() {
        let mut qobject = ParsedQObject::default();
        let mut item: ItemStruct = tokens_to_syn(quote! {
            struct T {
                #[qproperty]
                f64_property: f64,

                #[qproperty]
                pub public_property: f64,

                field: f64,
            }
        });
        assert!(qobject.parse_struct_fields(&mut item.fields).is_ok());
        assert_eq!(qobject.properties.len(), 2);
        assert_eq!(qobject.properties[0].ident, "f64_property");
        assert_eq!(qobject.properties[0].ty, f64_type());
        assert!(matches!(qobject.properties[0].vis, Visibility::Inherited));
        assert_eq!(qobject.properties[1].ident, "public_property");
        assert_eq!(qobject.properties[1].ty, f64_type());
        assert!(matches!(qobject.properties[1].vis, Visibility::Public(_)));
    }

    #[test]
    fn test_parse_struct_fields_invalid() {
        let mut qobject = ParsedQObject::default();
        let mut item: ItemStruct = tokens_to_syn(quote! {
            struct T(f64);
        });
        assert!(qobject.parse_struct_fields(&mut item.fields).is_err());
    }
}
