// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::naming::property::QPropertyNames;

/// Generate the metaobject line for a given property
pub fn generate(idents: &QPropertyNames, cxx_ty: &str) -> String {
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

    format!(
        "Q_PROPERTY({ty} {ident} {meta_parts})",
        ty = cxx_ty,
        ident = idents.name.cxx_unqualified(),
        meta_parts = parts.join(" ")
    )
}
