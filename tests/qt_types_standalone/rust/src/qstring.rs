// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QString;

#[cxx::bridge]
mod qstring_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");

        type QString = cxx_qt_lib::QString;
    }

    extern "Rust" {
        fn construct_qstring(slice: bool) -> QString;
        fn read_qstring(s: &QString) -> bool;
        fn modify_qstring(s: Pin<&mut QString>);
        fn can_handle_qstring_change() -> bool;
        fn clone_qstring(s: &QString) -> QString;
    }
}

fn construct_qstring(slice: bool) -> QString {
    if slice {
        QString::from("String constructed by Rust")
    } else {
        let rs_string = "String constructed by Rust".to_owned();
        QString::from(&rs_string)
    }
}

fn read_qstring(s: &cxx_qt_lib::QString) -> bool {
    let rs = s.to_string();
    rs == "String constructed by C++"
}

fn modify_qstring(mut s: core::pin::Pin<&mut cxx_qt_lib::QString>) {
    *s = QString::from("Updated string value");
}

fn can_handle_qstring_change() -> bool {
    let long_s = "Very very long string that is hopefully long enough to allocate and get Valgrind's attention :)";
    let long_s_ptr = QString::from(long_s);

    let short_s = "Short string";
    let mut short_s_ptr = QString::from(short_s);
    assert!(short_s_ptr.to_string() == short_s);

    short_s_ptr = long_s_ptr;

    short_s_ptr.to_string() == long_s
}

fn clone_qstring(s: &QString) -> QString {
    s.clone()
}
