// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{cpp::fragment::CppFragmentPair, naming::property::QPropertyName};
use indoc::formatdoc;

pub fn generate(idents: &QPropertyName, qobject_ident: &str) -> CppFragmentPair {
    CppFragmentPair {
        header: format!("void {ident_emit}();", ident_emit = idents.emit.cpp),
        source: formatdoc! {
            r#"
            void
            {qobject_ident}::{ident_emit}()
            {{
                const auto signalSuccess = QMetaObject::invokeMethod(this, "{ident_notify}", Qt::QueuedConnection);
                Q_ASSERT(signalSuccess);
            }}
            "#,
            ident_emit = idents.emit.cpp,
            ident_notify = idents.notify.cpp,
            qobject_ident = qobject_ident,
        },
    }
}
