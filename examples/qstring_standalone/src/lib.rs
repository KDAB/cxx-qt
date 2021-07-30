// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::pin::Pin;
use cxx_qt_lib::{let_qstring, QString};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("bridge.h");

        type QString = cxx_qt_lib::QString;

        fn test_constructed_qstring(s: &QString) -> bool;
        fn assign_to_qstring(s: Pin<&mut QString>, v: &QString);
    }

    extern "Rust" {
        fn can_construct_qstring(slice: bool) -> bool;
        fn can_read_qstring(s: &QString) -> bool;
        fn modify_qstring(s: Pin<&mut QString>);
        fn can_handle_qstring_change() -> bool;
    }
}

fn can_construct_qstring(slice: bool) -> bool {
    if slice {
        let_qstring!(s = "String constructed by Rust");
        ffi::test_constructed_qstring(&s)
    } else {
        let rs_string = "String constructed by Rust".to_owned();
        let_qstring!(s = rs_string);
        ffi::test_constructed_qstring(&s)
    }
}

fn can_read_qstring(s: &QString) -> bool {
    let rs = s.to_rust();
    rs == "String constructed by C++"
}

fn modify_qstring(s: Pin<&mut QString>) {
    let_qstring!(v = "Updated string value");
    ffi::assign_to_qstring(s, &v);
}

fn can_handle_qstring_change() -> bool {
    let long_s = "Very very long string that is hopefully long enough to allocate and get Valgrind's attention :)";

    let_qstring!(s = "Short string");
    let_qstring!(v = long_s);
    ffi::assign_to_qstring(s.as_mut(), &v);

    let rs = s.to_rust();
    rs == long_s
}
