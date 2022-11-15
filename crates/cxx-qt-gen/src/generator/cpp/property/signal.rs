// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::naming::property::QPropertyName;
use crate::CppFragment;

pub fn generate(idents: &QPropertyName) -> CppFragment {
    CppFragment::Header(format!(
        "Q_SIGNAL void {ident_notify}();",
        ident_notify = idents.notify.cpp
    ))
}
