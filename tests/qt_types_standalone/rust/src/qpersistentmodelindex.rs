// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QModelIndex, QPersistentModelIndex};

#[cxx::bridge]
mod qpersistentmodelindex_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpersistentmodelindex.h");
        type QPersistentModelIndex = cxx_qt_lib::QPersistentModelIndex;
    }

    extern "Rust" {
        fn construct_qpersistentmodelindex() -> QPersistentModelIndex;
        fn read_qpersistentmodelindex(i: &QPersistentModelIndex, expected: i32) -> bool;
        fn clone_qpersistentmodelindex(i: &QPersistentModelIndex) -> QPersistentModelIndex;
    }
}

fn construct_qpersistentmodelindex() -> QPersistentModelIndex {
    QPersistentModelIndex::from(&QModelIndex::default())
}

fn read_qpersistentmodelindex(i: &QPersistentModelIndex, expected: i32) -> bool {
    i.is_valid() && i.row() == expected
}

fn clone_qpersistentmodelindex(i: &QPersistentModelIndex) -> QPersistentModelIndex {
    i.clone()
}
