// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::Qt5List;

#[cxx::bridge]
mod qt5list_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt5list.h");
        type Qt5List_i32 = cxx_qt_lib::Qt5List<i32>;
    }

    extern "Rust" {
        fn construct_qt5list_i32() -> Qt5List_i32;
        fn read_qt5list_i32(v: &Qt5List_i32) -> bool;
        fn clone_qt5list_i32(v: &Qt5List_i32) -> Qt5List_i32;
    }
}

fn construct_qt5list_i32() -> Qt5List<i32> {
    let mut v = Qt5List::<i32>::default();
    v.append(1);
    v.append(1);
    v.append(3);
    v.append(3);
    v
}

fn read_qt5list_i32(v: &Qt5List<i32>) -> bool {
    // Ensure that the iterator works by building a vector from it
    let vec = v.iter().cloned().collect::<Vec<i32>>();

    vec == vec![1, 1, 3, 3]
}

fn clone_qt5list_i32(v: &Qt5List<i32>) -> Qt5List<i32> {
    v.clone()
}
