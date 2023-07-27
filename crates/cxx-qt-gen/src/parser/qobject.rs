// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    parser::{
        constructor::Constructor, inherit::ParsedInheritedMethod, method::ParsedMethod,
        property::ParsedQProperty, signals::ParsedSignal,
    },
    syntax::{
        attribute::attribute_find_path, expr::expr_to_string, foreignmod::ForeignTypeIdentAlias,
        path::path_compare_str,
    },
};
use syn::{Attribute, Error, Ident, ImplItem, Item, ItemImpl, Meta, Result};

/// Metadata for registering QML element
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QmlElementMetadata {
    pub name: String,
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
    /// QObject type that stores the invokables for the QObject
    pub qobject_ty: ForeignTypeIdentAlias,
    /// The namespace of the QObject. If one isn't specified for the QObject,
    /// this will be the same as the module
    pub namespace: String,
    /// Representation of the Q_SIGNALS for the QObject
    pub signals: Vec<ParsedSignal>,
    /// List of methods that need to be implemented on the C++ object in Rust
    ///
    /// These could also be exposed as Q_INVOKABLE on the C++ object
    pub methods: Vec<ParsedMethod>,
    /// List of inherited methods
    pub inherited_methods: Vec<ParsedInheritedMethod>,
    /// List of "impl" items that need to be implemented on the C++ object in Rust
    ///
    /// Note that they will only be visible on the Rust side
    pub passthrough_impl_items: Vec<ImplItem>,
    /// Any user-defined constructors
    pub constructors: Vec<Constructor>,
    /// List of properties that need to be implemented on the C++ object
    ///
    /// These will be exposed as Q_PROPERTY on the C++ object
    pub properties: Vec<ParsedQProperty>,
    /// List of specifiers to register with in QML
    pub qml_metadata: Option<QmlElementMetadata>,
    /// Whether locking is enabled for this QObject
    pub locking: bool,
    /// Whether threading has been enabled for this QObject
    pub threading: bool,
    /// Items that we don't need to generate anything for CXX or C++
    /// eg impls on the Rust object or Default implementations
    pub others: Vec<Item>,
}

impl TryFrom<&ForeignTypeIdentAlias> for ParsedQObject {
    type Error = syn::Error;

    /// Parse a ForeignTypeIdentAlias into a [ParsedQObject] with the index of the #[qobject] specified
    fn try_from(qobject_ty: &ForeignTypeIdentAlias) -> Result<Self> {
        let mut qobject_ty = qobject_ty.clone();

        // Find any QML metadata
        let qml_metadata = Self::parse_qml_metadata(&qobject_ty.ident_left, &mut qobject_ty.attrs)?;

        // Find if there is any base class
        let base_class = attribute_find_path(&qobject_ty.attrs, &["base"])
            .map(|index| {
                let attr = qobject_ty.attrs.remove(index);
                expr_to_string(&attr.meta.require_name_value()?.value)
            })
            .transpose()?;

        // Load the namespace, if it is empty then the ParsedCxxQtData will inject any global namespace
        let namespace = attribute_find_path(&qobject_ty.attrs, &["namespace"])
            .map(|index| {
                let attr = qobject_ty.attrs.remove(index);
                expr_to_string(&attr.meta.require_name_value()?.value)
            })
            .transpose()?
            .unwrap_or_else(|| "".to_owned());

        // Parse any properties in the type
        // and remove the #[qproperty] attribute
        let properties = Self::parse_property_attributes(&mut qobject_ty.attrs)?;

        Ok(Self {
            base_class,
            qobject_ty,
            namespace,
            signals: vec![],
            methods: vec![],
            inherited_methods: vec![],
            passthrough_impl_items: vec![],
            constructors: vec![],
            properties,
            qml_metadata,
            locking: true,
            threading: false,
            others: vec![],
        })
    }
}

impl ParsedQObject {
    fn parse_qml_metadata(
        qobject_ident: &Ident,
        attrs: &mut Vec<Attribute>,
    ) -> Result<Option<QmlElementMetadata>> {
        // Find if there is a qml_element attribute
        if let Some(index) = attribute_find_path(attrs, &["qml_element"]) {
            // Extract the name of the qml_element
            let name = match attrs.remove(index).meta {
                Meta::NameValue(name_value) => expr_to_string(&name_value.value)?,
                _ => qobject_ident.to_string(),
            };

            // Determine if this element is uncreatable
            let uncreatable = attribute_find_path(attrs, &["qml_uncreatable"])
                .map(|index| attrs.remove(index))
                .is_some();

            // Determine if this element is a singleton
            let singleton = attribute_find_path(attrs, &["qml_singleton"])
                .map(|index| attrs.remove(index))
                .is_some();

            return Ok(Some(QmlElementMetadata {
                name,
                uncreatable,
                singleton,
            }));
        }

        Ok(None)
    }

    pub fn parse_trait_impl(&mut self, imp: ItemImpl) -> Result<()> {
        let (not, trait_path, _) = &imp
            .trait_
            .as_ref()
            .ok_or_else(|| Error::new_spanned(imp.clone(), "Expected trait impl!"))?;

        if let Some(attr) = imp.attrs.first() {
            return Err(Error::new_spanned(
                attr,
                "Attributes are not allowed on trait impls in cxx_qt::bridge",
            ));
        }

        if path_compare_str(trait_path, &["cxx_qt", "Locking"]) {
            if imp.unsafety.is_none() {
                return Err(Error::new_spanned(
                    trait_path,
                    "cxx_qt::Locking must be an unsafe impl",
                ));
            }

            if not.is_none() {
                return Err(Error::new_spanned(
                    trait_path,
                    "cxx_qt::Locking is enabled by default, it can only be negated.",
                ));
            }

            // Check that cxx_qt::Threading is not enabled
            if self.threading {
                return Err(Error::new_spanned(
                    trait_path,
                    "cxx_qt::Locking must be enabled if cxx_qt::Threading is enabled",
                ));
            }

            self.locking = false;
            Ok(())
        } else if path_compare_str(trait_path, &["cxx_qt", "Threading"]) {
            if not.is_some() {
                return Err(Error::new_spanned(
                    trait_path,
                    "Negative impls for cxx_qt::Threading are not allowed",
                ));
            }

            // Check that cxx_qt::Locking is not disabled
            if !self.locking {
                return Err(Error::new_spanned(
                    trait_path,
                    "cxx_qt::Locking must be enabled if cxx_qt::Threading is enabled",
                ));
            }

            self.threading = true;
            Ok(())
        } else if path_compare_str(trait_path, &["cxx_qt", "Constructor"]) {
            self.constructors.push(Constructor::parse(imp)?);
            Ok(())
        } else {
            // TODO: Give suggestions on which trait might have been meant
            Err(Error::new_spanned(
                trait_path,
                "Unsupported trait!\nCXX-Qt currently only supports:\n- cxx_qt::Threading\n- cxx_qt::Constructor\n- cxx_qt::Locking\nNote that the trait must always be fully-qualified."
            ))
        }
    }

    fn parse_property_attributes(attrs: &mut Vec<Attribute>) -> Result<Vec<ParsedQProperty>> {
        let mut properties = vec![];

        // Note that once extract_if is stable, this would allow for comparing all the
        // elements once using path_compare_str and then building ParsedQProperty
        // from the extracted elements.
        // https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html#method.extract_if
        while let Some(index) = attribute_find_path(attrs, &["qproperty"]) {
            properties.push(ParsedQProperty::parse(attrs.remove(index))?);
        }

        Ok(properties)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::parser::tests::f64_type;
    use syn::{parse_quote, ItemImpl};

    pub fn create_parsed_qobject() -> ParsedQObject {
        let qobject_struct: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            type MyObject = super::MyObjectRust;
        };
        ParsedQObject::try_from(&qobject_struct).unwrap()
    }

    #[test]
    fn test_from_struct_no_base_class() {
        let qobject_struct: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            type MyObject = super::MyObjectRust;
        };

        let qobject = ParsedQObject::try_from(&qobject_struct).unwrap();
        assert!(qobject.base_class.is_none());
        assert!(qobject.qml_metadata.is_none());
    }

    #[test]
    fn test_from_struct_base_class() {
        let qobject_struct: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            #[base = "QStringListModel"]
            type MyObject = super::MyObjectRust;
        };

        let qobject = ParsedQObject::try_from(&qobject_struct).unwrap();
        assert_eq!(qobject.base_class.as_ref().unwrap(), "QStringListModel");
    }

    #[test]
    fn test_from_struct_properties_and_fields() {
        let qobject_struct: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            #[qproperty(i32, int_property)]
            #[qproperty(i32, public_property)]
            type MyObject = super::MyObjectRust;
        };

        let qobject = ParsedQObject::try_from(&qobject_struct).unwrap();
        assert_eq!(qobject.properties.len(), 2);
    }

    #[test]
    fn test_from_struct_fields() {
        let qobject_struct: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            type MyObject = super::MyObjectRust;
        };

        let qobject = ParsedQObject::try_from(&qobject_struct).unwrap();
        assert_eq!(qobject.properties.len(), 0);
    }

    #[test]
    fn test_parse_trait_impl_valid() {
        let mut qobject = create_parsed_qobject();
        let item: ItemImpl = parse_quote! {
            impl cxx_qt::Threading for MyObject {}
        };
        assert!(!qobject.threading);
        assert!(qobject.parse_trait_impl(item).is_ok());
        assert!(qobject.threading);
    }

    #[test]
    fn test_parse_trait_impl_invalid() {
        let mut qobject = create_parsed_qobject();

        // must be a trait
        let item: ItemImpl = parse_quote! {
            impl T {}
        };
        assert!(qobject.parse_trait_impl(item).is_err());

        // no attribute allowed
        let item: ItemImpl = parse_quote! {
            #[attr]
            impl cxx_qt::Threading for T {}
        };
        assert!(qobject.parse_trait_impl(item).is_err());

        // Threading cannot be negative
        let item: ItemImpl = parse_quote! {
            impl !cxx_qt::Threading for T {}
        };
        assert!(qobject.parse_trait_impl(item).is_err());

        // must be a known trait
        let item: ItemImpl = parse_quote! {
            #[attr]
            impl cxx_qt::ABC for T {}
        };
        assert!(qobject.parse_trait_impl(item).is_err());
    }

    #[test]
    fn test_parse_struct_fields_valid() {
        let item: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            #[qproperty(f64, f64_property)]
            #[qproperty(f64, public_property)]
            type T = super::TRust;
        };
        let properties = ParsedQObject::try_from(&item).unwrap().properties;
        assert_eq!(properties.len(), 2);

        assert_eq!(properties[0].ident, "f64_property");
        assert_eq!(properties[0].ty, f64_type());

        assert_eq!(properties[1].ident, "public_property");
        assert_eq!(properties[1].ty, f64_type());
    }

    #[test]
    fn test_qml_metadata() {
        let item: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            #[qml_element]
            type MyObject = super::MyObjectRust;
        };
        let qobject = ParsedQObject::try_from(&item).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                name: "MyObject".to_string(),
                uncreatable: false,
                singleton: false,
            })
        );
    }

    #[test]
    fn test_qml_metadata_named() {
        let item: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            #[qml_element = "OtherName"]
            type MyObject = super::MyObjectRust;
        };
        let qobject = ParsedQObject::try_from(&item).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                name: "OtherName".to_string(),
                uncreatable: false,
                singleton: false,
            })
        );
    }

    #[test]
    fn test_qml_metadata_singleton() {
        let item: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            #[qml_element]
            #[qml_singleton]
            type MyObject = super::MyObjectRust;
        };
        let qobject = ParsedQObject::try_from(&item).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                name: "MyObject".to_string(),
                uncreatable: false,
                singleton: true,
            })
        );
    }

    #[test]
    fn test_qml_metadata_uncreatable() {
        let item: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            #[qml_element]
            #[qml_uncreatable]
            type MyObject = super::MyObjectRust;
        };
        let qobject = ParsedQObject::try_from(&item).unwrap();
        assert_eq!(
            qobject.qml_metadata,
            Some(QmlElementMetadata {
                name: "MyObject".to_string(),
                uncreatable: true,
                singleton: false,
            })
        );
    }
}
