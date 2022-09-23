// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::{
    invokable::{ParsedQInvokable, ParsedQInvokableSpecifiers},
    parameter::ParsedFunctionParameter,
    property::{ParsedQProperty, ParsedRustField},
    signals::ParsedSignalsEnum,
};
use crate::syntax::{
    attribute::{attribute_find_path, attribute_tokens_to_map, AttributeDefault},
    fields::fields_to_named_fields_mut,
    implitemmethod::is_method_mutable,
};
use std::collections::HashSet;
use syn::{
    spanned::Spanned, Error, Fields, Ident, ImplItem, ImplItemMethod, Item, ItemStruct, LitStr,
    Result,
};
use syn::{FnArg, Pat, PatIdent, PatType};

/// A representation of a QObject within a CXX-Qt [syn::ItemMod]
///
/// This has initial splitting of [syn::Item]'s into relevant blocks, other phases will
/// then mutate these [syn::Item]'s for generation purposes.
pub struct ParsedQObject {
    /// The base class of the struct
    pub base_class: Option<String>,
    /// QObject struct that stores the invokables for the QObject
    pub qobject_struct: ItemStruct,
    /// The namespace of the QObject. If one isn't specified for the QObject,
    /// this will be the same as the module
    pub namespace: String,
    /// Representation of the Signals enum that defines the Q_SIGNALS for the QObject
    pub signals: Option<ParsedSignalsEnum>,
    /// List of invokables that need to be implemented on the C++ object in Rust
    ///
    /// These will also be exposed as Q_INVOKABLE on the C++ object
    pub invokables: Vec<ParsedQInvokable>,
    /// List of methods that need to be implemented on the C++ object in Rust
    ///
    /// Note that they will only be visible on the Rust side
    pub methods: Vec<ImplItemMethod>,
    /// List of properties that need to be implemented on the C++ object
    ///
    /// These will be exposed as Q_PROPERTY on the C++ object
    pub properties: Vec<ParsedQProperty>,
    /// List of Rust fields on the struct that need getters and setters generated
    pub fields: Vec<ParsedRustField>,
    /// Items that we don't need to generate anything for CXX or C++
    /// eg impls on the Rust object or Default implementations
    pub others: Vec<Item>,
}

impl ParsedQObject {
    /// Parse a [syn::ItemStruct] into a [ParsedQObject] with the index of the cxx_qt::qobject specified
    pub fn from_struct(qobject_struct: &ItemStruct, attr_index: usize) -> Result<Self> {
        // Find if there is any base class
        let base_class = attribute_tokens_to_map::<Ident, LitStr>(
            &qobject_struct.attrs[attr_index],
            AttributeDefault::None,
        )?
        .get(&quote::format_ident!("base"))
        .map(|base| base.value());

        // Remove the macro from the struct
        let mut qobject_struct = qobject_struct.clone();
        qobject_struct.attrs.remove(attr_index);

        // Parse any properties in the struct
        // and remove the #[qproperty] attribute
        let (properties, fields) = Self::parse_struct_fields(&mut qobject_struct.fields)?;

        Ok(Self {
            base_class,
            qobject_struct,
            // TODO: read from the qobject macro later
            namespace: "".to_owned(),
            signals: None,
            invokables: vec![],
            methods: vec![],
            properties,
            fields,
            others: vec![],
        })
    }

    /// Extract all methods (both invokable and non-invokable) from [syn::ImplItem]'s from each Impl block
    ///
    /// These will have come from a impl cxx_qt::QObject<T> block
    pub fn parse_impl_items(&mut self, items: &[ImplItem]) -> Result<()> {
        for item in items {
            // Check if this item is a method
            if let ImplItem::Method(method) = item {
                // Determine if this method is an invokable
                if let Some(index) = attribute_find_path(&method.attrs, &["qinvokable"]) {
                    // Parse any return_cxx_type in the qproperty macro
                    let attrs_map = attribute_tokens_to_map::<Ident, LitStr>(
                        &method.attrs[index],
                        AttributeDefault::Some(|span| LitStr::new("", span)),
                    )?;
                    let return_cxx_type = attrs_map
                        .get(&quote::format_ident!("return_cxx_type"))
                        .map(|lit_str| lit_str.value());

                    // Parse any C++ specifiers
                    let mut specifiers = HashSet::new();
                    if attrs_map.contains_key(&quote::format_ident!("cxx_final")) {
                        specifiers.insert(ParsedQInvokableSpecifiers::Final);
                    }
                    if attrs_map.contains_key(&quote::format_ident!("cxx_override")) {
                        specifiers.insert(ParsedQInvokableSpecifiers::Override);
                    }
                    if attrs_map.contains_key(&quote::format_ident!("cxx_virtual")) {
                        specifiers.insert(ParsedQInvokableSpecifiers::Virtual);
                    }

                    // Determine if the invokable is mutable
                    let mutable = is_method_mutable(method);

                    // Read the signal inputs into parameter blocks
                    let parameters = method
                        .sig
                        .inputs
                        .iter()
                        .map(|input| {
                            match input {
                                FnArg::Typed(PatType { pat, ty, .. }) => {
                                    let ident = if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                                        ident.clone()
                                    } else {
                                        return Err(Error::new(
                                            input.span(),
                                            "Invalid argument ident format.",
                                        ));
                                    };

                                    // Ignore self as a parameter
                                    if ident == "self" {
                                        return Ok(None);
                                    }

                                    Ok(Some(ParsedFunctionParameter {
                                        ident,
                                        ty: (**ty).clone(),
                                        // TODO: later we might support cxx_type for parameters in invokables
                                        cxx_type: None,
                                    }))
                                }
                                // Ignore self as a parameter
                                FnArg::Receiver(_) => Ok(None),
                            }
                        })
                        .filter_map(|result| result.map_or_else(|e| Some(Err(e)), |v| v.map(Ok)))
                        .collect::<Result<Vec<ParsedFunctionParameter>>>()?;

                    // Remove the invokable attribute
                    let mut method = method.clone();
                    method.attrs.remove(index);
                    self.invokables.push(ParsedQInvokable {
                        method,
                        mutable,
                        parameters,
                        return_cxx_type,
                        specifiers,
                    });
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
    fn parse_struct_fields(
        fields: &mut Fields,
    ) -> Result<(Vec<ParsedQProperty>, Vec<ParsedRustField>)> {
        let mut properties = vec![];
        let mut rust_fields = vec![];
        for field in fields_to_named_fields_mut(fields)? {
            // Try to find any properties defined within the struct
            if let Some(index) = attribute_find_path(&field.attrs, &["qproperty"]) {
                // Parse any cxx_type in the qproperty macro
                let cxx_type = attribute_tokens_to_map::<Ident, LitStr>(
                    &field.attrs[index],
                    AttributeDefault::None,
                )?
                .get(&quote::format_ident!("cxx_type"))
                .map(|lit_str| lit_str.value());

                // Remove the #[qproperty] attribute
                field.attrs.remove(index);

                properties.push(ParsedQProperty {
                    ident: field.ident.clone().unwrap(),
                    ty: field.ty.clone(),
                    vis: field.vis.clone(),
                    cxx_type,
                });
            } else {
                rust_fields.push(ParsedRustField {
                    ident: field.ident.clone().unwrap(),
                    ty: field.ty.clone(),
                    vis: field.vis.clone(),
                })
            }
        }

        Ok((properties, rust_fields))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::parser::tests::f64_type;
    use crate::tests::tokens_to_syn;
    use quote::quote;
    use syn::{ItemImpl, Visibility};

    pub fn create_parsed_qobject() -> ParsedQObject {
        let qobject_struct: ItemStruct = tokens_to_syn(quote! {
            #[cxx_qt::qobject]
            struct MyObject;
        });
        ParsedQObject::from_struct(&qobject_struct, 0).unwrap()
    }

    #[test]
    fn test_from_struct_no_base_class() {
        let qobject_struct: ItemStruct = tokens_to_syn(quote! {
            #[cxx_qt::qobject]
            struct MyObject;
        });

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert!(qobject.base_class.is_none());
    }

    #[test]
    fn test_from_struct_base_class() {
        let qobject_struct: ItemStruct = tokens_to_syn(quote! {
            #[cxx_qt::qobject(base = "QStringListModel")]
            struct MyObject;
        });

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert_eq!(qobject.base_class.as_ref().unwrap(), "QStringListModel");
    }

    #[test]
    fn test_from_struct_properties_and_fields() {
        let qobject_struct: ItemStruct = tokens_to_syn(quote! {
            #[cxx_qt::qobject]
            struct MyObject {
                #[qproperty]
                int_property: i32,

                #[qproperty]
                pub public_property: i32,

                field: i32,
            }
        });

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert_eq!(qobject.properties.len(), 2);
        assert_eq!(qobject.qobject_struct.fields.len(), 3);
    }

    #[test]
    fn test_from_struct_fields() {
        let qobject_struct: ItemStruct = tokens_to_syn(quote! {
            #[cxx_qt::qobject]
            struct MyObject {
                field: i32,
            }
        });

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert_eq!(qobject.properties.len(), 0);
        assert_eq!(qobject.qobject_struct.fields.len(), 1);
    }

    #[test]
    fn test_parse_impl_items_valid() {
        let mut qobject = create_parsed_qobject();
        let item: ItemImpl = tokens_to_syn(quote! {
            impl T {
                #[qinvokable]
                fn invokable(&self, a: f64, b: f64) {}

                #[qinvokable(return_cxx_type = "f32")]
                fn invokable_with_return_cxx_type(self: Pin<&mut Self>) -> f64 {}

                #[qinvokable(cxx_final, cxx_override, cxx_virtual)]
                fn invokable_with_specifiers() -> f64 {}

                fn cpp_context(&self) {}
            }
        });
        assert!(qobject.parse_impl_items(&item.items).is_ok());
        assert_eq!(qobject.invokables.len(), 3);
        assert_eq!(qobject.methods.len(), 1);
        assert!(qobject.invokables[0].return_cxx_type.is_none());
        assert!(!qobject.invokables[0].mutable);
        assert_eq!(qobject.invokables[0].parameters.len(), 2);
        assert_eq!(qobject.invokables[0].parameters[0].ident, "a");
        assert_eq!(qobject.invokables[0].parameters[0].ty, f64_type());
        assert_eq!(qobject.invokables[0].parameters[1].ident, "b");
        assert_eq!(qobject.invokables[0].parameters[1].ty, f64_type());
        assert_eq!(
            qobject.invokables[1].return_cxx_type.as_ref().unwrap(),
            "f32"
        );
        assert!(qobject.invokables[1].mutable);
        assert!(qobject.invokables[2]
            .specifiers
            .contains(&ParsedQInvokableSpecifiers::Final));
        assert!(qobject.invokables[2]
            .specifiers
            .contains(&ParsedQInvokableSpecifiers::Override));
        assert!(qobject.invokables[2]
            .specifiers
            .contains(&ParsedQInvokableSpecifiers::Virtual));
    }

    #[test]
    fn test_parse_impl_items_invalid() {
        let mut qobject = create_parsed_qobject();
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
        let item: ItemStruct = tokens_to_syn(quote! {
            #[cxx_qt::qobject]
            struct T {
                #[qproperty]
                f64_property: f64,

                #[qproperty]
                pub public_property: f64,

                #[qproperty(cxx_type = "f32")]
                property_with_cxx_type: f64,

                field: f64,
            }
        });
        let properties = ParsedQObject::from_struct(&item, 0).unwrap().properties;
        assert_eq!(properties.len(), 3);
        assert_eq!(properties[0].ident, "f64_property");
        assert_eq!(properties[0].ty, f64_type());
        assert!(matches!(properties[0].vis, Visibility::Inherited));
        assert!(properties[0].cxx_type.is_none());

        assert_eq!(properties[1].ident, "public_property");
        assert_eq!(properties[1].ty, f64_type());
        assert!(matches!(properties[1].vis, Visibility::Public(_)));
        assert!(properties[1].cxx_type.is_none());

        assert_eq!(properties[2].ident, "property_with_cxx_type");
        assert_eq!(properties[2].ty, f64_type());
        assert!(matches!(properties[2].vis, Visibility::Inherited));
        assert_eq!(properties[2].cxx_type.as_ref().unwrap(), "f32");
    }

    #[test]
    fn test_parse_struct_fields_invalid() {
        let item: ItemStruct = tokens_to_syn(quote! {
            #[cxx_qt::qobject]
            struct T(f64);
        });
        assert!(ParsedQObject::from_struct(&item, 0).is_err());
    }
}
