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
    pub setter: CombinedIdent,
    pub notify: CombinedIdent,
}

impl From<&Ident> for QPropertyName {
    fn from(ident: &Ident) -> Self {
        Self {
            name: name_from_ident(ident),
            getter: getter_from_ident(ident),
            setter: setter_from_ident(ident),
            notify: notify_from_ident(ident),
        }
    }
}

impl From<&ParsedQProperty> for QPropertyName {
    fn from(property: &ParsedQProperty) -> Self {
        Self::from(&property.ident)
    }
}

/// For a given ident generate the Rust and C++ getter names
fn getter_from_ident(ident: &Ident) -> CombinedIdent {
    CombinedIdent {
        cpp: format_ident!("get{}", ident.to_string().to_case(Case::Pascal)),
        rust: ident.clone(),
    }
}

/// For a given ident generate the Rust and C++ names
fn name_from_ident(ident: &Ident) -> CombinedIdent {
    CombinedIdent {
        cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
        rust: ident.clone(),
    }
}

/// For a given ident generate the Rust and C++ notify names
fn notify_from_ident(ident: &Ident) -> CombinedIdent {
    let ident = format_ident!("{}_changed", ident);
    CombinedIdent {
        cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
        rust: ident,
    }
}

/// For a given ident generate the Rust and C++ setter names
fn setter_from_ident(ident: &Ident) -> CombinedIdent {
    let ident = format_ident!("set_{}", ident);
    CombinedIdent {
        cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
        rust: ident,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;

    #[test]
    fn test_parsed_property() {
        let ty: syn::Type = tokens_to_syn(quote! { i32 });
        let property = ParsedQProperty {
            ident: format_ident!("my_property"),
            ty,
            vis: syn::Visibility::Inherited,
        };
        let names = QPropertyName::from(&property);
        assert_eq!(names.name.cpp, format_ident!("myProperty"));
        assert_eq!(names.name.rust, format_ident!("my_property"));
        assert_eq!(names.getter.cpp, format_ident!("getMyProperty"));
        assert_eq!(names.getter.rust, format_ident!("my_property"));
        assert_eq!(names.setter.cpp, format_ident!("setMyProperty"));
        assert_eq!(names.setter.rust, format_ident!("set_my_property"));
        assert_eq!(names.notify.cpp, format_ident!("myPropertyChanged"));
        assert_eq!(names.notify.rust, format_ident!("my_property_changed"));
    }
}
