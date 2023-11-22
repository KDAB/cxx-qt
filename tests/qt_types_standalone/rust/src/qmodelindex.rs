// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QModelIndex;

#[cxx::bridge]
mod qmodelindex_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
        type c_void = cxx_qt_lib::c_void;
    }

    extern "Rust" {
        fn construct_qmodelindex() -> QModelIndex;
        fn read_qmodelindex(i: &QModelIndex) -> bool;
        fn clone_qmodelindex(i: &QModelIndex) -> QModelIndex;
        fn internal_pointer_qmodelindex(i: &QModelIndex) -> *mut c_void;
        fn internal_id_qmodelindex(i: &QModelIndex) -> usize;
    }
}

fn construct_qmodelindex() -> QModelIndex {
    QModelIndex::default()
}

fn read_qmodelindex(i: &QModelIndex) -> bool {
    i.is_valid() && i.row() == 0
}

fn clone_qmodelindex(i: &QModelIndex) -> QModelIndex {
    i.clone()
}

fn internal_pointer_qmodelindex(i: &QModelIndex) -> *mut qmodelindex_cxx::c_void {
    i.internal_pointer_mut()
}

fn internal_id_qmodelindex(i: &QModelIndex) -> usize {
    i.internal_id()
}
