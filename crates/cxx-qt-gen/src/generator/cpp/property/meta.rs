// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::naming::property::QPropertyName;

/// Generate the metaobject line for a given property
pub fn generate(idents: &QPropertyName, cxx_ty: &str) -> String {
    format!(
        "Q_PROPERTY({ty} {ident} READ {ident_getter} WRITE {ident_setter} NOTIFY {ident_notify})",
        ty = cxx_ty,
        ident = idents.name.cpp,
        ident_getter = idents.getter.cpp,
        ident_setter = idents.setter.cpp,
        ident_notify = idents.notify.cpp,
    )
}
