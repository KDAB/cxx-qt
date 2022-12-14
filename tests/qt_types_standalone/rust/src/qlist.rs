// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QList;

#[cxx::bridge]
mod qlist_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlist.h");
        type QList_i32 = cxx_qt_lib::QList<i32>;
    }

    extern "Rust" {
        fn construct_qlist_i32() -> QList_i32;
        fn read_qlist_i32(v: &QList_i32) -> bool;
        fn clone_qlist_i32(v: &QList_i32) -> QList_i32;
    }
}

fn construct_qlist_i32() -> QList<i32> {
    let mut v = QList::<i32>::default();
    v.append(1);
    v.append(1);
    v.append(3);
    v.append(3);
    v
}

fn read_qlist_i32(v: &QList<i32>) -> bool {
    // Ensure that the iterator works by building a vector from it
    let vec = v.iter().cloned().collect::<Vec<i32>>();

    vec == vec![1, 1, 3, 3]
}

fn clone_qlist_i32(v: &QList<i32>) -> QList<i32> {
    v.clone()
}
