// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QJsonArray, QJsonValue, QString};

#[cxx::bridge]
mod qjsonarray_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qjsonarray.h");
        type QJsonArray = cxx_qt_lib::QJsonArray;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "Rust" {
        fn construct_qjsonarray_default() -> QJsonArray;
        fn construct_qjsonarray(first: &QString, second: &QString) -> QJsonArray;
        fn read_qjsonarray(array: &QJsonArray, first: &QString, second: &QString) -> bool;
        fn clone_qjsonarray(array: &QJsonArray) -> QJsonArray;
        fn copy_qjsonarray_by_iterating(array: &QJsonArray) -> QJsonArray;
        fn can_handle_qjsonarray_change() -> bool;
    }
}

fn construct_qjsonarray_default() -> QJsonArray {
    QJsonArray::default()
}

fn construct_qjsonarray(first: &QString, second: &QString) -> QJsonArray {
    let mut array = QJsonArray::default();
    array.append(&QJsonValue::from(first));
    array.append(&QJsonValue::from(second));
    array
}

fn read_qjsonarray(array: &QJsonArray, first: &QString, second: &QString) -> bool {
    array.len() == 2
        && !array.is_empty()
        && array.iter().len() == 2
        && array.at(0).to_string() == *first
        && array.at(1).to_string() == *second
        // Out of bounds access returns an undefined value
        && array.at(2).is_undefined()
}

fn clone_qjsonarray(array: &QJsonArray) -> QJsonArray {
    array.clone()
}

fn copy_qjsonarray_by_iterating(array: &QJsonArray) -> QJsonArray {
    let mut copy = QJsonArray::default();
    for value in array {
        copy.append(&value);
    }
    copy
}

fn can_handle_qjsonarray_change() -> bool {
    let mut long_array = QJsonArray::default();
    for i in 0..64 {
        long_array.append(&QJsonValue::from(&QString::from(&format!(
            "Very very long string number {i} that is hopefully long enough to allocate"
        ))));
    }

    let mut array = QJsonArray::default();
    assert!(array.is_empty());
    array.append(&QJsonValue::from(&QString::from("Short string")));
    assert!(array.len() == 1);

    array = long_array;

    array.len() == 64
}
