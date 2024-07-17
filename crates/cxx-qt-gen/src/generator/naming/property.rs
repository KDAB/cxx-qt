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
use syn::Ident;

/// Names for parts of a Q_PROPERTY
pub struct QPropertyNames {
    pub name: Name,
    pub getter: Name,
    pub getter_wrapper: Name,
    pub setter: Option<Name>,
    pub setter_wrapper: Option<Name>,
    pub notify: Option<Name>,
}

impl From<&ParsedQProperty> for QPropertyNames {
    fn from(property: &ParsedQProperty) -> Self {
        let property_name = property_name_from_rust_name(property.ident.clone());

        let flags = &property.flags;

        let getter = match &flags.read {
            FlagState::Auto => getter_name_from_property(&property_name),
            FlagState::Custom(ident) => Name::new(ident.clone()),
        };

        let setter = match &flags.write {
            Some(state) => match state {
                FlagState::Auto => Some(setter_name_from_property(&property_name)),
                FlagState::Custom(ident) => Some(Name::new(ident.clone())),
            },
            None => None,
        };

        let notify = match &flags.notify {
            Some(state) => match state {
                FlagState::Auto => Some(notify_name_from_property(&property_name)),
                FlagState::Custom(ident) => Some(Name::new(ident.clone())),
            },
            None => None,
        };

        let setter_wrapper = setter.as_ref().map(wrapper_name_from_function_name);

        Self {
            getter_wrapper: wrapper_name_from_function_name(&getter),
            getter,
            setter_wrapper,
            setter,
            notify,
            name: property_name,
        }
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

    pub fn create_i32_qpropertyname() -> QPropertyNames {
        let ty: syn::Type = parse_quote! { i32 };
        let property = ParsedQProperty {
            ident: format_ident!("my_property"),
            ty,
            flags: QPropertyFlags::default(),
        };
        QPropertyNames::from(&property)
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
