// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QByteArray;

#[cxx::bridge]
mod qbytearray_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");

        type QByteArray = cxx_qt_lib::QByteArray;
    }

    extern "Rust" {
        fn construct_qbytearray(slice: bool) -> QByteArray;
        fn read_qbytearray(s: &QByteArray) -> bool;
        fn modify_qbytearray(s: Pin<&mut QByteArray>);
        fn can_handle_qbytearray_change() -> bool;
        fn clone_qbytearray(s: &QByteArray) -> QByteArray;
        fn can_use_as_slice() -> bool;
    }
}

fn construct_qbytearray(slice: bool) -> QByteArray {
    if slice {
        QByteArray::from("String constructed by Rust")
    } else {
        let rs_string = "String constructed by Rust".to_owned();
        QByteArray::from(&rs_string)
    }
}

fn read_qbytearray(s: &cxx_qt_lib::QByteArray) -> bool {
    s.as_slice() == b"String constructed by C++"
}

fn modify_qbytearray(mut s: core::pin::Pin<&mut cxx_qt_lib::QByteArray>) {
    *s = QByteArray::from("Updated string value");
}

fn can_handle_qbytearray_change() -> bool {
    let long_s = b"Very very long string that is hopefully long enough to allocate and get Valgrind's attention :)";
    let long_s_ptr = QByteArray::from(long_s);

    let short_s = b"Short string";
    let mut short_s_ptr = QByteArray::from(short_s);
    assert!(short_s_ptr.as_slice() == short_s);

    short_s_ptr = long_s_ptr;

    short_s_ptr.as_slice() == long_s
}

fn clone_qbytearray(s: &QByteArray) -> QByteArray {
    s.clone()
}

fn can_use_as_slice() -> bool {
    let string = "String slice";
    let slice = unsafe { QByteArray::from_raw_data(string.as_bytes()) };

    slice.as_slice() == string.as_bytes()
}
