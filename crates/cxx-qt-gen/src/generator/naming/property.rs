// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{naming::Name, parser::property::{ParsedQProperty, QPropertyFlag}};
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
    pub notify: Name,
}

impl From<&ParsedQProperty> for QPropertyNames {
    fn from(property: &ParsedQProperty) -> Self {
        let property_name = property_name_from_rust_name(property.ident.clone());

        let mut getter: Name = Name::new(format_ident!("test_ident"));
        let mut setter: Option<Name> = None;
        let mut notify: Name = Name::new(format_ident!("test_ident"));

        for flag in &property.flags {
            match flag {
                QPropertyFlag::Write(ref signature) => { // TODO: remove if let blocks (passing custom func should not change name of getter, only its contents)
                    if let Some(ident) = signature {
                        setter = Some(Name::new(ident.clone()))
                    }
                    else {
                        setter = Some(setter_name_from_property(&property_name))
                    }
                },
                QPropertyFlag::Read(ref signature) => {
                    if let Some(ident) = signature {
                        getter = Name::new(ident.clone())
                    }
                    else {
                        getter = getter_name_from_property(&property_name)
                    }
                },
                QPropertyFlag::Notify(ref signature) => {
                    if let Some(ident) = signature {
                        notify = Name::new(ident.clone())
                    }
                    else {
                        notify = notify_name_from_property(&property_name)
                    }
                },
            }
        }
        let setter_wrapper: Option<Name>;
        if let Some(name) = &setter {
            setter_wrapper = Some(wrapper_name_from_function_name(&name));
        }
        else {
            setter_wrapper = None;
        }

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

    pub fn create_i32_qpropertyname() -> QPropertyNames {
        let ty: syn::Type = parse_quote! { i32 };
        let property = ParsedQProperty {
            ident: format_ident!("my_property"),
            ty,
            flags: Default::default(),
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
        assert_eq!(names.setter.clone().expect("Setter was empty").cxx_unqualified(), "setMyProperty");
        assert_eq!(
            names.setter.clone().expect("Setter was empty").rust_unqualified(),
            &format_ident!("set_my_property")
        );
        assert_eq!(names.notify.cxx_unqualified(), "myPropertyChanged");
        assert_eq!(
            names.notify.rust_unqualified(),
            &format_ident!("my_property_changed")
        );
    }
}
