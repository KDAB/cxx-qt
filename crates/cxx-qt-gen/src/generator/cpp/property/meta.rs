// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::naming::property::QPropertyNames;

/// Generate the metaobject line for a given property
pub fn generate(idents: &QPropertyNames, cxx_ty: &str) -> String {
    format!(
        "Q_PROPERTY({ty} {ident} READ {ident_getter} WRITE {ident_setter} NOTIFY {ident_notify})",
        ty = cxx_ty,
        ident = idents.name.cxx_unqualified(),
        ident_getter = idents.getter.cxx_unqualified(),
        ident_setter = idents.setter.cxx_unqualified(),
        ident_notify = idents.notify.cxx_unqualified()
    )
}
