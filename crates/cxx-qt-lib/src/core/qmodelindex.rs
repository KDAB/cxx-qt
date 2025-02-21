// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmodelindex.h");
        include!("cxx-qt-lib/qstring.h");

        type QModelIndex = super::QModelIndex;
        type QString = crate::QString;
        type quintptr = crate::quintptr;

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

        /// Returns a `quintptr` used by the model to associate the index with the internal data structure.
        #[rust_name = "internal_id"]
        fn internalId(self: &QModelIndex) -> quintptr;
        /// Returns a `*mut c_void` pointer used by the model to associate the index with the internal data structure.
        #[rust_name = "internal_pointer_mut"]
        fn internalPointer(self: &QModelIndex) -> *mut c_void;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
        type c_void = crate::c_void;

        #[doc(hidden)]
        #[rust_name = "qmodelindex_init_default"]
        fn construct() -> QModelIndex;
        #[doc(hidden)]
        #[rust_name = "qmodelindex_eq"]
        fn operatorEq(a: &QModelIndex, b: &QModelIndex) -> bool;
        #[doc(hidden)]
        #[rust_name = "qmodelindex_to_debug_qstring"]
        fn toDebugQString(value: &QModelIndex) -> QString;
    }
}

/// The QModelIndex class is used to locate data in a data model.
#[derive(Clone)]
#[repr(C)]
pub struct QModelIndex {
    _r: MaybeUninit<i32>,
    _c: MaybeUninit<i32>,
    _i: MaybeUninit<usize>,
    _m: MaybeUninit<usize>,
}

impl Default for QModelIndex {
    /// Creates a new empty model index. This type of model index is used to indicate that the position in the model is invalid.
    fn default() -> Self {
        ffi::qmodelindex_init_default()
    }
}

impl std::cmp::PartialEq for QModelIndex {
    fn eq(&self, other: &Self) -> bool {
        ffi::qmodelindex_eq(self, other)
    }
}

impl std::cmp::Eq for QModelIndex {}

impl fmt::Display for QModelIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl fmt::Debug for QModelIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qmodelindex_to_debug_qstring(self))
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QModelIndex {
    type Id = type_id!("QModelIndex");
    type Kind = cxx::kind::Trivial;
}
