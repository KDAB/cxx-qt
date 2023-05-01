// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::generator::naming::CombinedIdent;
use crate::parser::property::ParsedQProperty;
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::Ident;

/// Names for parts of a Q_PROPERTY
pub struct QPropertyName {
    pub name: CombinedIdent,
    pub getter: CombinedIdent,
    pub getter_mutable: CombinedIdent,
    pub setter: CombinedIdent,
    pub notify: CombinedIdent,
}

impl From<&Ident> for QPropertyName {
    fn from(ident: &Ident) -> Self {
        Self {
            name: CombinedIdent::from_property(ident.clone()),
            getter: CombinedIdent::getter_from_property(ident.clone()),
            getter_mutable: CombinedIdent::getter_mutable_from_property(ident),
            setter: CombinedIdent::setter_from_property(ident),
            notify: CombinedIdent::notify_from_property(ident),
        }
    }
}

impl From<&ParsedQProperty> for QPropertyName {
    fn from(property: &ParsedQProperty) -> Self {
        Self::from(&property.ident)
    }
}

impl CombinedIdent {
    /// For a given ident generate the Rust and C++ getter names
    fn getter_from_property(ident: Ident) -> Self {
        Self {
            cpp: format_ident!("get{}", ident.to_string().to_case(Case::Pascal)),
            rust: ident,
        }
    }

    /// For a given ident generate the Rust and C++ getter mutable names
    fn getter_mutable_from_property(ident: &Ident) -> Self {
        Self {
            cpp: format_ident!("get{}Mut", ident.to_string().to_case(Case::Pascal)),
            rust: format_ident!("{ident}_mut"),
        }
    }

    /// For a given ident generate the Rust and C++ names
    fn from_property(ident: Ident) -> Self {
        Self {
            cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
            rust: ident,
        }
    }

    /// For a given ident generate the Rust and C++ notify names
    fn notify_from_property(ident: &Ident) -> Self {
        let ident = format_ident!("{ident}_changed");
        Self {
            cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
            rust: ident,
        }
    }

    /// For a given ident generate the Rust and C++ setter names
    fn setter_from_property(ident: &Ident) -> Self {
        let ident = format_ident!("set_{ident}");
        Self {
            cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
            rust: ident,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use syn::parse_quote;

    use super::*;

    pub fn create_i32_qpropertyname() -> QPropertyName {
        let ty: syn::Type = parse_quote! { i32 };
        let property = ParsedQProperty {
            ident: format_ident!("my_property"),
            ty,
            vis: syn::Visibility::Inherited,
            cxx_type: None,
        };
        QPropertyName::from(&property)
    }

    #[test]
    fn test_parsed_property() {
        let names = create_i32_qpropertyname();
        assert_eq!(names.name.cpp, format_ident!("myProperty"));
        assert_eq!(names.name.rust, format_ident!("my_property"));
        assert_eq!(names.getter.cpp, format_ident!("getMyProperty"));
        assert_eq!(names.getter.rust, format_ident!("my_property"));
        assert_eq!(names.getter_mutable.cpp, format_ident!("getMyPropertyMut"));
        assert_eq!(names.getter_mutable.rust, format_ident!("my_property_mut"));
        assert_eq!(names.setter.cpp, format_ident!("setMyProperty"));
        assert_eq!(names.setter.rust, format_ident!("set_my_property"));
        assert_eq!(names.notify.cpp, format_ident!("myPropertyChanged"));
        assert_eq!(names.notify.rust, format_ident!("my_property_changed"));
    }
}
