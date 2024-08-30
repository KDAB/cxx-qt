// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    naming::Name,
    parser::property::ParsedQProperty,
    syntax::{
        attribute::attribute_take_path, expr::expr_to_string, foreignmod::ForeignTypeIdentAlias,
    },
};
#[cfg(test)]
use quote::format_ident;

use syn::{Attribute, Error, Ident, Meta, Result};

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
    /// List of properties that need to be implemented on the C++ object
    ///
    /// These will be exposed as Q_PROPERTY on the C++ object
    pub properties: Vec<ParsedQProperty>,
    /// List of specifiers to register with in QML
    pub qml_metadata: Option<QmlElementMetadata>,
    pub rust_type: Ident,
    /// Whether this type has a #[qobject] / Q_OBJECT macro
    pub has_qobject_macro: bool,

    /// The original declaration entered by the user, i.e. a type alias with a list of attributes
    pub declaration: ForeignTypeIdentAlias,
}

impl ParsedQObject {
    #[cfg(test)]
    pub fn mock() -> Self {
        ParsedQObject {
            base_class: None,
            name: Name::new(format_ident!("MyObject")),
            rust_type: format_ident!("MyObjectRust"),
            properties: vec![],
            qml_metadata: None,
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
        let has_qobject_macro = attribute_take_path(&mut declaration.attrs, &["qobject"]).is_some();

        // Find if there is any base class
        let base_class = attribute_take_path(&mut declaration.attrs, &["base"])
            .map(|attr| {
                let string = expr_to_string(&attr.meta.require_name_value()?.value)?;
                if string.is_empty() {
                    // Ensure that the base class attribute is not empty, as this is not valid in both cases
                    // - when there is a qobject macro it is not valid
                    // - when there is not a qobject macro it is not valid
                    return Err(Error::new_spanned(
                        attr,
                        "The #[base] attribute cannot be empty",
                    ));
                }
                Ok(string)
            })
            .transpose()?;

        // Ensure that if there is no qobject macro that a base class is specificed
        //
        // Note this assumes the check above
        if !has_qobject_macro && base_class.is_none() {
            return Err(Error::new_spanned(
                declaration.ident_left,
                "A type without a #[qobject] attribute must specify a #[base] attribute",
            ));
        }

        // Find any QML metadata
        let qml_metadata =
            Self::parse_qml_metadata(&declaration.ident_left, &mut declaration.attrs)?;

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
            properties,
            qml_metadata,
            has_qobject_macro,
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
    use syn::parse_quote;

    pub fn create_parsed_qobject() -> ParsedQObject {
        let qobject_struct: ForeignTypeIdentAlias = parse_quote! {
            #[qobject]
            type MyObject = super::MyObjectRust;
        };
        ParsedQObject::parse(qobject_struct, None, &format_ident!("qobject")).unwrap()
    }

    macro_rules! parse_qobject {
        { $($input:tt)* } => {
            {
                let input = parse_quote! {
                    $($input)*
                };
                ParsedQObject::parse(input, None, &format_ident!("qobject")).unwrap()
            }
       }
    }

    #[test]
    fn test_has_qobject_name() {
        let qobject = parse_qobject! {
            #[qobject]
            type MyObject = super::MyObjectRust;
        };
        assert!(qobject.has_qobject_macro);
        let qobject = parse_qobject! {
            #[base="Thing"]
            type MyObject = super::MyObjectRust;
        };
        assert!(!qobject.has_qobject_macro);
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

    macro_rules! assert_parse_errors {
        { $($input:tt)* } => {
            $(assert!(ParsedQObject::parse(syn::parse_quote! $input, None, &format_ident!("qobject")).is_err());)*
        }
    }

    #[test]
    fn test_parse_errors() {
        assert_parse_errors! {
            {
                #[qobject]
                #[base = ""]
                type MyObject = super::T;
            }
            {
                type MyObject = super::T;
            }
        }
    }
}
