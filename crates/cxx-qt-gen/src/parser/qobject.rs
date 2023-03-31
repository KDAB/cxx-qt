// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::{
    inherit::ParsedInheritedMethod,
    invokable::ParsedQInvokable,
    property::{ParsedQProperty, ParsedRustField},
    signals::ParsedSignalsEnum,
};
use crate::syntax::{
    attribute::{attribute_find_path, attribute_tokens_to_map, AttributeDefault},
    fields::fields_to_named_fields_mut,
};
use syn::{
    spanned::Spanned, Error, Fields, Ident, ImplItem, ImplItemFn, Item, ItemStruct, LitStr, Result,
    Visibility,
};

/// Metadata for registering QML element
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QmlElementMetadata {
    pub uri: String,
    pub name: String,
    pub version_major: usize,
    pub version_minor: usize,
    pub uncreatable: bool,
    pub singleton: bool,
}

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
    /// List of inherited methods
    pub inherited_methods: Vec<ParsedInheritedMethod>,
    /// List of "impl" items that need to be implemented on the C++ object in Rust
    ///
    /// Note that they will only be visible on the Rust side
    pub passthrough_impl_items: Vec<ImplItem>,
    /// List of properties that need to be implemented on the C++ object
    ///
    /// These will be exposed as Q_PROPERTY on the C++ object
    pub properties: Vec<ParsedQProperty>,
    /// List of Rust fields on the struct that need getters and setters generated
    pub fields: Vec<ParsedRustField>,
    /// List of specifiers to register with in QML
    pub qml_metadata: Option<QmlElementMetadata>,
    /// Items that we don't need to generate anything for CXX or C++
    /// eg impls on the Rust object or Default implementations
    pub others: Vec<Item>,
}

impl ParsedQObject {
    /// Parse a [syn::ItemStruct] into a [ParsedQObject] with the index of the cxx_qt::qobject specified
    pub fn from_struct(qobject_struct: &ItemStruct, attr_index: usize) -> Result<Self> {
        let qml_metadata = Self::parse_qml_metadata(qobject_struct, attr_index)?;

        let attrs_map = attribute_tokens_to_map::<Ident, LitStr>(
            &qobject_struct.attrs[attr_index],
            AttributeDefault::Some(|span| LitStr::new("", span)),
        )?;

        // Find if there is any base class
        let base_class = attrs_map
            .get(&quote::format_ident!("base"))
            .map(|base| base.value());

        // Load the namespace, if it is empty then the ParsedCxxQtData will inject any global namespace
        let namespace = attrs_map
            .get(&quote::format_ident!("namespace"))
            .map_or_else(|| "".to_owned(), |base| base.value());

        // Remove the macro from the struct
        let mut qobject_struct = qobject_struct.clone();
        qobject_struct.attrs.remove(attr_index);

        // Parse any properties in the struct
        // and remove the #[qproperty] attribute
        let (properties, fields) = Self::parse_struct_fields(&mut qobject_struct.fields)?;

        // Ensure that the QObject is marked as pub otherwise the error is non obvious
        // https://github.com/KDAB/cxx-qt/issues/457
        if !matches!(qobject_struct.vis, Visibility::Public(..)) {
            return Err(Error::new(
                qobject_struct.span(),
                "qobject marked structs must be public",
            ));
        }

        Ok(Self {
            base_class,
            qobject_struct,
            namespace,
            signals: None,
            invokables: vec![],
            inherited_methods: vec![],
            passthrough_impl_items: vec![],
            properties,
            fields,
            qml_metadata,
            others: vec![],
        })
    }

    fn parse_qml_metadata(
        qobject_struct: &ItemStruct,
        attr_index: usize,
    ) -> Result<Option<QmlElementMetadata>> {
        let attrs_map = attribute_tokens_to_map::<Ident, LitStr>(
            &qobject_struct.attrs[attr_index],
            AttributeDefault::Some(|span| LitStr::new("", span)),
        )?;
        let qml_uri = attrs_map.get(&quote::format_ident!("qml_uri"));
        let qml_version = attrs_map.get(&quote::format_ident!("qml_version"));
        let qml_name = attrs_map.get(&quote::format_ident!("qml_name"));
        let qml_uncreatable = attrs_map.get(&quote::format_ident!("qml_uncreatable"));
        let qml_singleton = attrs_map.get(&quote::format_ident!("qml_singleton"));
        match (qml_uri, qml_version) {
            (Some(qml_uri), Some(qml_version)) => {
                let qml_version = qml_version.value();
                let version_parts: Vec<_> = qml_version.split('.').collect();
                let version_major = version_parts[0]
                    .parse()
                    .expect("Could not parse major version from qml_version");
                let version_minor = version_parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);

                let name = match qml_name {
                    Some(qml_name) => qml_name.value(),
                    None => qobject_struct.ident.to_string(),
                };

                Ok(Some(QmlElementMetadata {
                    uri: qml_uri.value(),
                    name,
                    version_major,
                    version_minor,
                    uncreatable: qml_uncreatable.is_some(),
                    singleton: qml_singleton.is_some(),
                }))
            }
            (Some(uri), None) => Err(Error::new(
                uri.span(),
                "qml_uri specified but no qml_version specified",
            )),
            (None, Some(version)) => Err(Error::new(
                version.span(),
                "qml_version specified but no qml_uri specified",
            )),
            (None, None) => {
                if let Some(qml_name) = qml_name {
                    return Err(Error::new(
                        qml_name.span(),
                        "qml_name specified but qml_uri and qml_version unspecified",
                    ));
                }
                if let Some(qml_uncreatable) = qml_uncreatable {
                    return Err(Error::new(
                        qml_uncreatable.span(),
                        "qml_uncreatable specified but qml_uri and qml_version unspecified",
                    ));
                }
                if let Some(qml_singleton) = qml_singleton {
                    return Err(Error::new(
                        qml_singleton.span(),
                        "qml_singleton specified but qml_uri and qml_version unspecified",
                    ));
                }
                Ok(None)
            }
        }
    }

    fn parse_impl_method(&mut self, method: &ImplItemFn) -> Result<()> {
        if let Some(invokable) = ParsedQInvokable::try_parse(method)? {
            self.invokables.push(invokable);
        } else {
            self.passthrough_impl_items
                .push(ImplItem::Fn(method.clone()));
        }
        Ok(())
    }

    /// Extract all methods (both invokable and non-invokable) from [syn::ImplItem]'s from each Impl block
    ///
    /// These will have come from a impl qobject::T block
    pub fn parse_impl_items(&mut self, items: &[ImplItem]) -> Result<()> {
        for item in items {
            // Check if this item is a method
            match item {
                ImplItem::Fn(method) => {
                    self.parse_impl_method(method)?;
                }
                _ => {
                    self.passthrough_impl_items.push(item.clone());
                }
            }
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

    use crate::parser::{invokable::ParsedQInvokableSpecifiers, tests::f64_type};
    use syn::{parse_quote, ItemImpl, Visibility};

    pub fn create_parsed_qobject() -> ParsedQObject {
        let qobject_struct: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            pub struct MyObject;
        };
        ParsedQObject::from_struct(&qobject_struct, 0).unwrap()
    }

    #[test]
    fn test_from_struct_no_base_class() {
        let qobject_struct: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            pub struct MyObject;
        };

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert!(qobject.base_class.is_none());
        assert!(qobject.qml_metadata.is_none());
    }

    #[test]
    fn test_from_struct_base_class() {
        let qobject_struct: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(base = "QStringListModel")]
            pub struct MyObject;
        };

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert_eq!(qobject.base_class.as_ref().unwrap(), "QStringListModel");
    }

    #[test]
    fn test_from_struct_properties_and_fields() {
        let qobject_struct: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            pub struct MyObject {
                #[qproperty]
                int_property: i32,

                #[qproperty]
                pub public_property: i32,

                field: i32,
            }
        };

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert_eq!(qobject.properties.len(), 2);
        assert_eq!(qobject.qobject_struct.fields.len(), 3);
    }

    #[test]
    fn test_from_struct_fields() {
        let qobject_struct: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            pub struct MyObject {
                field: i32,
            }
        };

        let qobject = ParsedQObject::from_struct(&qobject_struct, 0).unwrap();
        assert_eq!(qobject.properties.len(), 0);
        assert_eq!(qobject.qobject_struct.fields.len(), 1);
    }

    #[test]
    fn test_parse_impl_items_valid() {
        let mut qobject = create_parsed_qobject();
        let item: ItemImpl = parse_quote! {
            impl T {
                #[qinvokable]
                fn invokable(&self, a: f64, b: f64) {}

                #[qinvokable(return_cxx_type = "f32")]
                fn invokable_with_return_cxx_type(self: Pin<&mut Self>) -> f64 {}

                #[qinvokable(cxx_final, cxx_override, cxx_virtual)]
                fn invokable_with_specifiers(&self) -> f64 {}

                fn cpp_context(&self) {}
            }
        };
        assert!(qobject.parse_impl_items(&item.items).is_ok());
        assert_eq!(qobject.invokables.len(), 3);
        assert_eq!(qobject.passthrough_impl_items.len(), 1);
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
        let item: ItemImpl = parse_quote! {
            impl T {
                const VALUE: i32 = 1;

                macro_code!();

                type A = i32;

                #[qinvokable]
                fn invokable() {}

                fn cpp_context() {}
            }
        };
        assert!(qobject.parse_impl_items(&item.items).is_err());
    }

    #[test]
    fn test_parse_struct_fields_valid() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            pub struct T {
                #[qproperty]
                f64_property: f64,

                #[qproperty]
                pub public_property: f64,

                #[qproperty(cxx_type = "f32")]
                property_with_cxx_type: f64,

                field: f64,
            }
        };
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
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject]
            pub struct T(f64);
        };
        assert!(ParsedQObject::from_struct(&item, 0).is_err());
    }

    #[test]
    fn test_qml_metadata() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_uri = "foo.bar", qml_version = "1.0")]
            pub struct MyObject;
        };
        let qobject = ParsedQObject::from_struct(&item, 0).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                uri: "foo.bar".to_owned(),
                name: "MyObject".to_owned(),
                version_major: 1,
                version_minor: 0,
                uncreatable: false,
                singleton: false,
            })
        );
    }

    #[test]
    fn test_qml_metadata_named() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_uri = "foo.bar", qml_version = "1", qml_name = "MyQmlElement")]
            pub struct MyNamedObject;
        };
        let qobject = ParsedQObject::from_struct(&item, 0).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                uri: "foo.bar".to_owned(),
                name: "MyQmlElement".to_owned(),
                version_major: 1,
                version_minor: 0,
                uncreatable: false,
                singleton: false,
            })
        );
    }

    #[test]
    fn test_qml_metadata_singleton() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_uri = "foo.bar", qml_version = "1", qml_singleton)]
            pub struct MyObject;
        };
        let qobject = ParsedQObject::from_struct(&item, 0).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                uri: "foo.bar".to_owned(),
                name: "MyObject".to_owned(),
                version_major: 1,
                version_minor: 0,
                uncreatable: false,
                singleton: true,
            })
        );
    }

    #[test]
    fn test_qml_metadata_uncreatable() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_uri = "foo.bar", qml_version = "1", qml_uncreatable)]
            pub struct MyObject;
        };
        let qobject = ParsedQObject::from_struct(&item, 0).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                uri: "foo.bar".to_owned(),
                name: "MyObject".to_owned(),
                version_major: 1,
                version_minor: 0,
                uncreatable: true,
                singleton: false,
            })
        );
    }

    #[test]
    fn test_qml_metadata_no_version() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_uri = "foo.bar")]
            pub struct MyObject;
        };
        assert!(ParsedQObject::from_struct(&item, 0).is_err());
    }

    #[test]
    fn test_qml_metadata_no_uri() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_version = "1.0")]
            pub struct MyObject;
        };
        assert!(ParsedQObject::from_struct(&item, 0).is_err());
    }

    #[test]
    fn test_qml_metadata_only_name_no_version_no_uri() {
        let item: ItemStruct = parse_quote! {
            #[cxx_qt::qobject(qml_name = "MyQmlElement")]
            pub struct MyObject;
        };
        assert!(ParsedQObject::from_struct(&item, 0).is_err());
    }
}
