// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    naming::Name,
    parser::{constructor::Constructor, inherit::ParsedInheritedMethod, property::ParsedQProperty},
    syntax::{
        attribute::attribute_take_path, expr::expr_to_string, foreignmod::ForeignTypeIdentAlias,
        path::path_compare_str,
    },
};
#[cfg(test)]
use quote::format_ident;

use syn::{Attribute, Error, Ident, ItemImpl, Meta, Result};

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
    /// The name of the QObject
    pub name: Name,
    /// The ident of the inner type of the QObject
    pub rust_type: Ident,
    /// List of inherited methods
    pub inherited_methods: Vec<ParsedInheritedMethod>,
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
    /// Whether this type has a #[qobject] / Q_OBJECT macro
    pub has_qobject_macro: bool,

    /// The original declaration entered by the user, i.e. a type alias with a list of attributes
    pub declaration: ForeignTypeIdentAlias,
}

impl ParsedQObject {
    #[cfg(test)]
    pub fn mock_parsed_qobject() -> Self {
        ParsedQObject {
            base_class: None,
            name: Name::new(format_ident!("my_property")),
            rust_type: format_ident!("i32"),
            inherited_methods: vec![],
            constructors: vec![],
            properties: vec![],
            qml_metadata: None,
            locking: false,
            threading: false,
            has_qobject_macro: false,
            declaration: ForeignTypeIdentAlias {
                attrs: vec![],
                ident_left: format_ident!("MyObject"),
                ident_right: format_ident!("MyObjectRust"),
            },
        }
    }

    /// Parse a ForeignTypeIdentAlias into a [ParsedQObject] with the index of the #[qobject] specified
    pub fn parse(
        mut declaration: ForeignTypeIdentAlias,
        namespace: Option<&str>,
        module: &Ident,
    ) -> Result<Self> {
        // Find any QML metadata
        let qml_metadata =
            Self::parse_qml_metadata(&declaration.ident_left, &mut declaration.attrs)?;

        // Find if there is any base class
        let base_class = attribute_take_path(&mut declaration.attrs, &["base"])
            .map(|attr| expr_to_string(&attr.meta.require_name_value()?.value))
            .transpose()?;

        let name = Name::from_ident_and_attrs(
            &declaration.ident_left,
            &declaration.attrs,
            namespace,
            Some(module),
        )?;

        // Parse any properties in the type
        // and remove the #[qproperty] attribute
        let properties = Self::parse_property_attributes(&mut declaration.attrs)?;
        let inner = declaration.ident_right.clone();

        Ok(Self {
            base_class,
            declaration,
            name,
            rust_type: inner,
            inherited_methods: vec![],
            constructors: vec![],
            properties,
            qml_metadata,
            locking: true,
            threading: false,
            has_qobject_macro: false,
        })
    }

    fn parse_qml_metadata(
        qobject_ident: &Ident,
        attrs: &mut Vec<Attribute>,
    ) -> Result<Option<QmlElementMetadata>> {
        // Find if there is a qml_element attribute
        if let Some(attr) = attribute_take_path(attrs, &["qml_element"]) {
            // Extract the name of the qml_element
            let name = match attr.meta {
                Meta::NameValue(name_value) => expr_to_string(&name_value.value)?,
                _ => qobject_ident.to_string(),
            };

            // Determine if this element is uncreatable
            let uncreatable = attribute_take_path(attrs, &["qml_uncreatable"]).is_some();

            // Determine if this element is a singleton
            let singleton = attribute_take_path(attrs, &["qml_singleton"]).is_some();

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
        while let Some(attr) = attribute_take_path(attrs, &["qproperty"]) {
            properties.push(ParsedQProperty::parse(attr)?);
        }

        Ok(properties)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::parser::tests::f64_type;
    use quote::format_ident;
    use syn::{parse_quote, ItemImpl};

    pub fn create_parsed_qobject() -> ParsedQObject {
        let qobject_struct: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            type MyObject = super::MyObjectRust;
        };
        ParsedQObject::parse(qobject_struct, None, &format_ident!("qobject")).unwrap()
    }

    #[test]
    fn test_from_struct_no_base_class() {
        let qobject_struct: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            type MyObject = super::MyObjectRust;
        };

        let qobject =
            ParsedQObject::parse(qobject_struct, None, &format_ident!("qobject")).unwrap();
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

        let qobject =
            ParsedQObject::parse(qobject_struct, None, &format_ident!("qobject")).unwrap();
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

        let qobject =
            ParsedQObject::parse(qobject_struct, None, &format_ident!("qobject")).unwrap();
        assert_eq!(qobject.properties.len(), 2);
    }

    #[test]
    fn test_from_struct_fields() {
        let qobject_struct: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            type MyObject = super::MyObjectRust;
        };

        let qobject =
            ParsedQObject::parse(qobject_struct, None, &format_ident!("qobject")).unwrap();
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
        let properties = ParsedQObject::parse(item, None, &format_ident!("qobject"))
            .unwrap()
            .properties;
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
        let qobject = ParsedQObject::parse(item, None, &format_ident!("qobject")).unwrap();
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
        let qobject = ParsedQObject::parse(item, None, &format_ident!("qobject")).unwrap();
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
        let qobject = ParsedQObject::parse(item, None, &format_ident!("qobject")).unwrap();
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
        let qobject = ParsedQObject::parse(item, None, &format_ident!("qobject")).unwrap();
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
