// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QSet;

#[cxx::bridge]
mod qset_cxx {
    // ANCHOR: book_qset
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_i32 = cxx_qt_lib::QSet<i32>;
    }
    // ANCHOR_END: book_qset

    extern "Rust" {
        fn construct_qset_i32() -> QSet_i32;
        fn read_qset_i32(s: &QSet_i32) -> bool;
        fn clone_qset_i32(s: &QSet_i32) -> QSet_i32;
    }
}

fn construct_qset_i32() -> QSet<i32> {
    let mut s = QSet::<i32>::default();
    s.insert(1);
    s.insert(1);
    s.insert(3);
    s.insert(3);
    s
}

fn read_qset_i32(s: &QSet<i32>) -> bool {
    // Ensure that the iterator works by building a vector from it
    let mut vec = s.iter().cloned().collect::<Vec<i32>>();

    // Sort the vec as a set iterator does not have a guaranteed order
    vec.sort();
    vec == vec![1, 3]
}

fn clone_qset_i32(s: &QSet<i32>) -> QSet<i32> {
    s.clone()
}
