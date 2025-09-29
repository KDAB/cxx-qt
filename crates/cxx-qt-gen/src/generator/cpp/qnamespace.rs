// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeSet;

use indoc::formatdoc;

use crate::{parser::qnamespace::ParsedQNamespace, writer::cpp::namespaced};

/// Generate the declaration of the namespace, including the Q_NAMESPACE macro.
pub fn generate(qnamespace: &ParsedQNamespace, includes: &mut BTreeSet<String>) -> String {
    includes.insert("#include <QtCore/QObject>".to_owned());
    let mut result = "Q_NAMESPACE".to_owned();
    if qnamespace.qml_element {
        includes.insert("#include <QtQml/QQmlEngine>".to_owned());
        result = formatdoc! { r"
            {result}
            QML_ELEMENT"};
    }
    namespaced(&qnamespace.namespace, &result)
}
