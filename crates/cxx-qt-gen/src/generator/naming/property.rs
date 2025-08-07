// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{
    naming::Name,
    parser::property::{FlagState, ParsedQProperty},
};
use quote::format_ident;
use syn::Result;

use crate::generator::structuring::StructuredQObject;
use core::ops::Deref;

#[derive(Debug)]
pub enum NameState {
    Auto(Name),
    Custom(Name),
}

impl Deref for NameState {
    type Target = Name;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Custom(name) | Self::Auto(name) => name,
        }
    }
}

impl NameState {
    pub fn from_flag_with_auto_fn(
        state: &FlagState,
        auto_fn: impl Fn() -> Name,
        structured_qobject: &StructuredQObject,
        signal: bool,
    ) -> Result<Self> {
        let lookup_fn = if signal {
            StructuredQObject::signal_lookup
        } else {
            StructuredQObject::method_lookup
        };
        Ok(match state {
            FlagState::Auto => Self::Auto(auto_fn()),
            FlagState::Custom(ident) => Self::Custom(lookup_fn(structured_qobject, ident)?),
        })
    }
}

/// Names for parts of a Q_PROPERTY
pub struct QPropertyNames {
    pub name: Name,
    pub getter: NameState,
    pub setter: Option<NameState>,
    pub notify: Option<NameState>,
    pub reset: Option<Name>,
}

impl QPropertyNames {
    pub(crate) fn try_from_property(
        property: &ParsedQProperty,
        structured_qobject: &StructuredQObject,
    ) -> Result<Self> {
        let property_name = &property.name;

        // Cache flags as they are accessed multiple times
        let flags = &property.flags;

        let getter = NameState::from_flag_with_auto_fn(
            &flags.read,
            || getter_name_from_property(property_name),
            structured_qobject,
            false,
        )?;

        let setter = flags
            .write
            .clone()
            .map(|setter| {
                NameState::from_flag_with_auto_fn(
                    &setter,
                    || setter_name_from_property(property_name),
                    structured_qobject,
                    false,
                )
            })
            .transpose()?;

        let notify = flags
            .notify
            .clone()
            .map(|notify| {
                NameState::from_flag_with_auto_fn(
                    &notify,
                    || notify_name_from_property(property_name),
                    structured_qobject,
                    true,
                )
            })
            .transpose()?;

        let reset = flags
            .reset
            .as_ref()
            .map(|ident| structured_qobject.method_lookup(ident))
            .transpose()?;

        Ok(Self {
            getter,
            setter,
            notify,
            reset,
            name: property_name.clone(),
        })
    }
}

fn capitalise_first(s: &mut str) -> &mut str {
    if !s.is_empty() {
        s[0..1].make_ascii_uppercase();
    }
    s
}

/// For a given property name generate the getter name
pub fn getter_name_from_property(name: &Name) -> Name {
    name.clone().with_cxx_name(format!(
        "get{}",
        capitalise_first(&mut name.cxx_unqualified())
    ))
}

/// For a given property name generate the setter name
pub fn setter_name_from_property(name: &Name) -> Name {
    name.clone()
        .with_rust_name(format_ident!("set_{}", name.rust_unqualified()))
        .with_cxx_name(format!(
            "set{}",
            capitalise_first(&mut name.cxx_unqualified())
        ))
}

/// For a given property name generate the notify signal name
pub fn notify_name_from_property(name: &Name) -> Name {
    name.clone()
        .with_rust_name(format_ident!("{}_changed", name.rust_unqualified()))
        .with_cxx_name(format!("{}Changed", name.cxx_unqualified()))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::parser::property::QPropertyFlags;
    use crate::parser::qobject::ParsedQObject;
    use crate::parser::CaseConversion;
    use syn::{parse_quote, ItemStruct};

    pub fn create_i32_qpropertyname() -> QPropertyNames {
        let property = ParsedQProperty {
            name: Name::mock_name_with_cxx("my_property", "myProperty"),
            ty: parse_quote! { i32 },
            flags: QPropertyFlags::default(),
        };

        let obj = ParsedQObject::mock();

        let structured_qobject = StructuredQObject::mock(&obj);
        QPropertyNames::try_from_property(&property, &structured_qobject)
            .expect("Failed to create QPropertyNames")
    }

    #[test]
    fn test_invalid_custom_signal() {
        let input: ItemStruct = parse_quote! {
            #[qproperty(T, reused_prop, READ, WRITE, NOTIFY = unknown_signal)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(&input.attrs[0], CaseConversion::none()).unwrap();

        let obj = ParsedQObject::mock();

        let structured_qobject = StructuredQObject::mock(&obj);
        let qproperty_names = QPropertyNames::try_from_property(&property, &structured_qobject);
        assert!(qproperty_names.is_err());
    }

    #[test]
    fn test_parsed_property() {
        let names = create_i32_qpropertyname();
        assert_eq!(names.name.cxx_unqualified(), "myProperty");
        assert_eq!(names.name.rust_unqualified(), "my_property");
        assert_eq!(names.getter.cxx_unqualified(), "getMyProperty");
        assert_eq!(names.getter.rust_unqualified(), "my_property");
        assert_eq!(
            names.setter.as_ref().unwrap().cxx_unqualified(),
            "setMyProperty"
        );
        assert_eq!(
            names.setter.as_ref().unwrap().rust_unqualified(),
            "set_my_property"
        );
        assert_eq!(
            names.notify.as_ref().unwrap().cxx_unqualified(),
            "myPropertyChanged"
        );
        assert_eq!(
            names.notify.as_ref().unwrap().rust_unqualified(),
            "my_property_changed"
        );
    }

    #[test]
    fn test_capitalise_first() {
        assert_eq!(capitalise_first(&mut "abc".to_owned()), "Abc");
        assert_eq!(capitalise_first(&mut String::new()), "");
        assert_eq!(capitalise_first(&mut "a".to_owned()), "A");
    }
}
