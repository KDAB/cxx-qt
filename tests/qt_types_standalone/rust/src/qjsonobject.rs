// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QJsonObject, QJsonValue, QString, QStringList};

#[cxx::bridge]
mod qjsonobject_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qjsonobject.h");
        type QJsonObject = cxx_qt_lib::QJsonObject;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;
    }

    extern "Rust" {
        fn construct_qjsonobject_default() -> QJsonObject;
        fn construct_qjsonobject(key: &QString, value: &QString) -> QJsonObject;
        fn read_qjsonobject(object: &QJsonObject, key: &QString, value: &QString) -> bool;
        fn clone_qjsonobject(object: &QJsonObject) -> QJsonObject;
        fn keys_qjsonobject(object: &QJsonObject) -> QStringList;
        fn can_handle_qjsonobject_change() -> bool;
    }
}

fn construct_qjsonobject_default() -> QJsonObject {
    QJsonObject::default()
}

fn construct_qjsonobject(key: &QString, value: &QString) -> QJsonObject {
    let mut object = QJsonObject::default();
    object.insert(key, &QJsonValue::from(value));
    object
}

fn read_qjsonobject(object: &QJsonObject, key: &QString, value: &QString) -> bool {
    object.len() == 1
        && !object.is_empty()
        && object.contains(key)
        && &object.value(key).to_string() == value
        // A missing key gives an undefined value
        && object.value(&QString::from("missing key")).is_undefined()
}

fn clone_qjsonobject(object: &QJsonObject) -> QJsonObject {
    object.clone()
}

fn keys_qjsonobject(object: &QJsonObject) -> QStringList {
    object.keys()
}

fn can_handle_qjsonobject_change() -> bool {
    let mut long_object = QJsonObject::default();
    for i in 0..64 {
        long_object.insert(
            &QString::from(&format!("key {i}")),
            &QJsonValue::from(&QString::from(&format!(
                "Very very long string number {i} that is hopefully long enough to allocate"
            ))),
        );
    }

    let mut object = QJsonObject::default();
    assert!(object.is_empty());
    object.insert(
        &QString::from("key"),
        &QJsonValue::from(&QString::from("Short string")),
    );
    assert!(object.len() == 1);

    object = long_object;
    object.remove(&QString::from("key 0"));

    object.len() == 63 && !object.contains(&QString::from("key 0"))
}
