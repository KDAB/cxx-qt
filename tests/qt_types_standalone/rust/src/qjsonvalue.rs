// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QJsonArray, QJsonObject, QJsonValue, QString};

#[cxx::bridge]
mod qjsonvalue_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qjsonarray.h");
        type QJsonArray = cxx_qt_lib::QJsonArray;

        include!("cxx-qt-lib/qjsonobject.h");
        type QJsonObject = cxx_qt_lib::QJsonObject;

        include!("cxx-qt-lib/qjsonvalue.h");
        type QJsonValue = cxx_qt_lib::QJsonValue;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "Rust" {
        fn construct_qjsonvalue_default() -> QJsonValue;
        fn construct_qjsonvalue_bool(value: bool) -> QJsonValue;
        fn construct_qjsonvalue_double(value: f64) -> QJsonValue;
        fn construct_qjsonvalue_int(value: i32) -> QJsonValue;
        fn construct_qjsonvalue_string(value: &QString) -> QJsonValue;
        fn construct_qjsonvalue_array(value: &QJsonArray) -> QJsonValue;
        fn construct_qjsonvalue_object(value: &QJsonObject) -> QJsonValue;

        fn read_qjsonvalue_bool(value: &QJsonValue, expected: bool) -> bool;
        fn read_qjsonvalue_double(value: &QJsonValue, expected: f64) -> bool;
        fn read_qjsonvalue_int(value: &QJsonValue, expected: i32) -> bool;
        fn read_qjsonvalue_string(value: &QJsonValue, expected: &QString) -> bool;
        fn read_qjsonvalue_array(value: &QJsonValue, expected: &QJsonArray) -> bool;
        fn read_qjsonvalue_object(value: &QJsonValue, expected: &QJsonObject) -> bool;
        fn read_qjsonvalue_null(value: &QJsonValue) -> bool;
        fn read_qjsonvalue_undefined(value: &QJsonValue) -> bool;

        fn clone_qjsonvalue(value: &QJsonValue) -> QJsonValue;
        fn can_handle_qjsonvalue_change() -> bool;
    }
}

fn construct_qjsonvalue_default() -> QJsonValue {
    QJsonValue::default()
}

fn construct_qjsonvalue_bool(value: bool) -> QJsonValue {
    QJsonValue::from(value)
}

fn construct_qjsonvalue_double(value: f64) -> QJsonValue {
    QJsonValue::from(value)
}

fn construct_qjsonvalue_int(value: i32) -> QJsonValue {
    QJsonValue::from(value as i64)
}

fn construct_qjsonvalue_string(value: &QString) -> QJsonValue {
    QJsonValue::from(value)
}

fn construct_qjsonvalue_array(value: &QJsonArray) -> QJsonValue {
    QJsonValue::from(value)
}

fn construct_qjsonvalue_object(value: &QJsonObject) -> QJsonValue {
    QJsonValue::from(value)
}

fn read_qjsonvalue_bool(value: &QJsonValue, expected: bool) -> bool {
    value.is_bool() && value.to_bool() == expected
}

fn read_qjsonvalue_double(value: &QJsonValue, expected: f64) -> bool {
    value.is_double() && (value.to_double() - expected).abs() < f64::EPSILON
}

fn read_qjsonvalue_int(value: &QJsonValue, expected: i32) -> bool {
    value.is_double() && value.to_int() == expected
}

fn read_qjsonvalue_string(value: &QJsonValue, expected: &QString) -> bool {
    value.is_string() && &value.to_string() == expected
}

fn read_qjsonvalue_array(value: &QJsonValue, expected: &QJsonArray) -> bool {
    value.is_array() && &value.to_array() == expected
}

fn read_qjsonvalue_object(value: &QJsonValue, expected: &QJsonObject) -> bool {
    value.is_object() && &value.to_object() == expected
}

fn read_qjsonvalue_null(value: &QJsonValue) -> bool {
    value.is_null() && !value.is_undefined()
}

fn read_qjsonvalue_undefined(value: &QJsonValue) -> bool {
    value.is_undefined() && !value.is_null()
}

fn clone_qjsonvalue(value: &QJsonValue) -> QJsonValue {
    value.clone()
}

fn can_handle_qjsonvalue_change() -> bool {
    let mut array = QJsonArray::default();
    for i in 0..64 {
        array.append(&QJsonValue::from(&QString::from(&format!(
            "Very very long string number {i} that is hopefully long enough to allocate"
        ))));
    }
    let long_value = QJsonValue::from(&array);

    let mut value = QJsonValue::from(&QString::from("Short string"));
    assert!(value.is_string());

    value = long_value;

    value.is_array() && value.to_array().len() == 64
}
