// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{
    naming::Name,
    parser::property::{FlagState, ParsedQProperty},
};
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::{Ident, Result};

use crate::generator::structuring::StructuredQObject;
use std::ops::Deref;

#[derive(Debug)]
pub enum NameState {
    Auto(Name),
    Custom(Name),
}

impl Deref for NameState {
    type Target = Name;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Auto(name) => name,
            Self::Custom(name) => name,
        }
    }
}

impl NameState {
    pub fn from_flag_with_auto_fn(
        state: &FlagState,
        auto_fn: impl Fn() -> Name,
        structured_qobject: &StructuredQObject,
    ) -> Result<Self> {
        match state {
            FlagState::Auto => Ok(Self::Auto(auto_fn())),
            FlagState::Custom(ident) => Ok(Self::Custom(structured_qobject.method_lookup(ident)?)),
        }
    }
}

/// Names for parts of a Q_PROPERTY
pub struct QPropertyNames {
    pub name: Name,
    pub getter: NameState,
    pub getter_wrapper: Option<Name>,
    pub setter: Option<NameState>,
    pub setter_wrapper: Option<Name>,
    pub notify: Option<NameState>,
    pub reset: Option<Name>,
}

impl QPropertyNames {
    pub(crate) fn try_from_property(
        property: &ParsedQProperty,
        structured_qobject: &StructuredQObject,
    ) -> Result<Self> {
        let property_name = property_name_from_rust_name(property.ident.clone());

        // Cache flags as they are accessed multiple times
        let flags = &property.flags;

        let getter = NameState::from_flag_with_auto_fn(
            &flags.read,
            || getter_name_from_property(&property_name),
            structured_qobject,
        )?;

        let setter = if let Some(setter) = &flags.write {
            Some(NameState::from_flag_with_auto_fn(
                setter,
                || setter_name_from_property(&property_name),
                structured_qobject,
            )?)
        } else {
            None
        };

        let notify = if let Some(notify) = &flags.notify {
            Some(NameState::from_flag_with_auto_fn(
                notify,
                || notify_name_from_property(&property_name),
                structured_qobject,
            )?)
        } else {
            None
        };

        let setter_wrapper = if let Some(NameState::Auto(ref setter)) = setter {
            Some(wrapper_name_from_function_name(setter))
        } else {
            None
        };

        let getter_wrapper = if let NameState::Auto(ref getter) = getter {
            Some(wrapper_name_from_function_name(getter))
        } else {
            None
        };

        let reset = flags.reset.as_ref().map(|ident| Name::new(ident.clone()));

        Ok(Self {
            getter_wrapper,
            getter,
            setter_wrapper,
            setter,
            notify,
            reset,
            name: property_name,
        })
    }
}

fn property_name_from_rust_name(ident: Ident) -> Name {
    // TODO: ParsedQProperty should probably take care of this already and allow the user to set
    // their own name for C++ if they want to.
    let cxx_name = ident.to_string().to_case(Case::Camel);
    Name::new(ident).with_cxx_name(cxx_name)
}

/// For a given property name generate the getter name
fn getter_name_from_property(name: &Name) -> Name {
    name.clone().with_cxx_name(format!(
        "get{}",
        name.cxx_unqualified().to_case(Case::Pascal)
    ))
}

/// For a given property name generate the setter name
fn setter_name_from_property(name: &Name) -> Name {
    name.clone()
        .with_rust_name(format_ident!("set_{}", name.rust_unqualified()))
        .with_cxx_name(format!(
            "set{}",
            name.cxx_unqualified().to_case(Case::Pascal)
        ))
}

/// For a given function name generate the Rust and C++ wrapper names
fn wrapper_name_from_function_name(name: &Name) -> Name {
    name.clone()
        .with_rust_name(format_ident!("{}_wrapper", name.rust_unqualified().clone()))
        .with_cxx_name(format!("{}Wrapper", name.cxx_unqualified()))
}

/// For a given property name generate the notify signal name
fn notify_name_from_property(name: &Name) -> Name {
    name.clone()
        .with_rust_name(format_ident!("{}_changed", name.rust_unqualified()))
        .with_cxx_name(format!("{}Changed", name.cxx_unqualified()))
}

#[cfg(test)]
pub mod tests {
    use syn::parse_quote;

    use super::*;
    use crate::parser::property::QPropertyFlags;
    use crate::parser::qobject::ParsedQObject;
    use crate::syntax::foreignmod::ForeignTypeIdentAlias;

    pub fn create_i32_qpropertyname() -> QPropertyNames {
        let ty: syn::Type = parse_quote! { i32 };
        let property = ParsedQProperty {
            ident: format_ident!("my_property"),
            ty,
            flags: QPropertyFlags::default(),
        };
        // PROTO, only for allowing code to compile
        let obj = ParsedQObject {
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
        };

        let structured_qobject = StructuredQObject::from_qobject(&obj);
        QPropertyNames::try_from_property(&property, &structured_qobject)
            .expect("Failed to create QPropertyNames")
    }

    #[test]
    fn test_parsed_property() {
        let names = create_i32_qpropertyname();
        assert_eq!(names.name.cxx_unqualified(), "myProperty");
        assert_eq!(names.name.rust_unqualified(), &format_ident!("my_property"));
        assert_eq!(names.getter.cxx_unqualified(), "getMyProperty");
        assert_eq!(
            names.getter.rust_unqualified(),
            &format_ident!("my_property")
        );
        assert_eq!(
            names.setter.as_ref().unwrap().cxx_unqualified(),
            "setMyProperty"
        );
        assert_eq!(
            names.setter.as_ref().unwrap().rust_unqualified(),
            &format_ident!("set_my_property")
        );
        assert_eq!(
            names.notify.as_ref().unwrap().cxx_unqualified(),
            "myPropertyChanged"
        );
        assert_eq!(
            names.notify.as_ref().unwrap().rust_unqualified(),
            &format_ident!("my_property_changed")
        );
    }
}
