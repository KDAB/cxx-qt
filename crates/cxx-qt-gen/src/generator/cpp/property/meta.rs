// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::naming::property::QPropertyNames;
use crate::parser::property::QPropertyFlags;

/// Generate the metaobject line for a given property
pub fn generate(idents: &QPropertyNames, flags: &QPropertyFlags, cxx_ty: &str) -> String {
    let mut parts = vec![format!(
        "READ {ident_getter}",
        ident_getter = idents.getter.cxx_unqualified()
    )];

    if let Some(setter) = &idents.setter {
        parts.push(format!("WRITE {}", setter.cxx_unqualified()));
    }

    if let Some(notify) = &idents.notify {
        parts.push(format!("NOTIFY {}", notify.cxx_unqualified()));
    }

    if let Some(reset) = &idents.reset {
        parts.push(format!("RESET {}", reset.cxx_unqualified()));
    }

    if flags.constant {
        parts.push(String::from("CONSTANT"));
    }

    if flags.required {
        parts.push(String::from("REQUIRED"));
    }

    if flags.is_final {
        parts.push(String::from("FINAL"));
    }

    format!(
        "Q_PROPERTY({ty} {ident} {meta_parts})",
        ty = cxx_ty,
        ident = idents.name.cxx_unqualified(),
        meta_parts = parts.join(" ")
    )
}
