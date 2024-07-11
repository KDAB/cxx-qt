// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::naming::property::QPropertyNames;

/// Generate the metaobject line for a given property
pub fn generate(idents: &QPropertyNames, cxx_ty: &str) -> String {
    let mut output: String = format!(
        "Q_PROPERTY({ty} {ident} READ {ident_getter} ",
        ty = cxx_ty,
        ident = idents.name.cxx_unqualified(),
        ident_getter = idents.getter.cxx_unqualified()
    );
    // Write
    if let Some(name) = &idents.setter {
        output += format!("WRITE {} ", name.cxx_unqualified()).as_str();
    }
    // Notify
    if let Some(name) = &idents.notify {
        output += format!("NOTIFY {}", name.cxx_unqualified()).as_str();
    }

    output += ")";

    output
}
