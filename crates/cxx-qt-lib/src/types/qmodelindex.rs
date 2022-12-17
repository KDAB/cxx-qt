// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmodelindex.h");

        type QModelIndex = super::QModelIndex;

        /// Returns the column this model index refers to.
        fn column(self: &QModelIndex) -> i32;
        /// Returns true if this model index is valid; otherwise returns false.
        ///
        /// A valid index belongs to a model, and has non-negative row and column numbers.
        #[rust_name = "is_valid"]
        fn isValid(self: &QModelIndex) -> bool;
        /// Returns the parent of the model index, or QModelIndex() if it has no parent.
        fn parent(self: &QModelIndex) -> QModelIndex;
        /// Returns the row this model index refers to.
        fn row(self: &QModelIndex) -> i32;
        /// Returns the sibling at row and column. If there is no sibling at this position, an invalid QModelIndex is returned.
        fn sibling(self: &QModelIndex, row: i32, column: i32) -> QModelIndex;
        /// Returns the sibling at column for the current row. If there is no sibling at this position, an invalid QModelIndex is returned.
        #[rust_name = "sibling_at_column"]
        fn siblingAtColumn(self: &QModelIndex, column: i32) -> QModelIndex;
        /// Returns the sibling at row for the current column. If there is no sibling at this position, an invalid QModelIndex is returned.
        #[rust_name = "sibling_at_row"]
        fn siblingAtRow(self: &QModelIndex, row: i32) -> QModelIndex;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qmodelindex_init_default"]
        fn construct() -> QModelIndex;
    }
}

/// The QModelIndex class is used to locate data in a data model.
#[derive(Clone)]
#[repr(C)]
pub struct QModelIndex {
    _space: MaybeUninit<[usize; 3]>,
}

impl Default for QModelIndex {
    /// Creates a new empty model index. This type of model index is used to indicate that the position in the model is invalid.
    fn default() -> Self {
        ffi::qmodelindex_init_default()
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QModelIndex {
    type Id = type_id!("QModelIndex");
    type Kind = cxx::kind::Trivial;
}
