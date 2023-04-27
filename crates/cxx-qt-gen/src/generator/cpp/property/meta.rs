// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{cpp::types::CppType, naming::property::QPropertyName};
use crate::parser::property::ParsedQProperty;

/// Generate the metaobject line for a given property
pub fn generate(idents: &QPropertyName, property: &ParsedQProperty, cxx_ty: &CppType) -> String {
    let getter_setter_not_explicit = property.get.is_none() && property.set.is_none();

    let getter = if getter_setter_not_explicit || property.get.is_some() {
        format!("READ {ident_getter}", ident_getter = idents.getter.cpp)
    } else {
        String::new()
    };

    let setter = if getter_setter_not_explicit || property.set.is_some() {
        format!("WRITE {ident_setter}", ident_setter = idents.setter.cpp)
    } else {
        String::new()
    };

    format!(
        "Q_PROPERTY({ty} {ident} {getter} {setter} NOTIFY {ident_notify})",
        ty = cxx_ty.as_cxx_ty(),
        ident = idents.name.cpp,
        getter = getter,
        setter = setter,
        ident_notify = idents.notify.cpp,
    )
}
