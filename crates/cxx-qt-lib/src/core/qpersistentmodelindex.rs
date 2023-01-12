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
        include!("cxx-qt-lib/qpersistentmodelindex.h");
        include!("cxx-qt-lib/qstring.h");

        type QPersistentModelIndex = super::QPersistentModelIndex;
        type QString = crate::QString;

        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = crate::QModelIndex;

        /// Returns the column this persistent model index refers to.
        fn column(self: &QPersistentModelIndex) -> i32;
        /// Returns true if this persistent model index is valid; otherwise returns false.
        ///
        /// A valid index belongs to a model, and has non-negative row and column numbers.
        #[rust_name = "is_valid"]
        fn isValid(self: &QPersistentModelIndex) -> bool;
        /// Returns the parent QModelIndex for this persistent index, or an invalid QModelIndex if it has no parent.
        fn parent(self: &QPersistentModelIndex) -> QModelIndex;
        /// Returns the row this persistent model index refers to.
        fn row(self: &QPersistentModelIndex) -> i32;
        /// Returns the sibling at row and column or an invalid QModelIndex if there is no sibling at this position.
        fn sibling(self: &QPersistentModelIndex, row: i32, column: i32) -> QModelIndex;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpersistentmodelindex_drop"]
        fn drop(string: &mut QPersistentModelIndex);

        #[doc(hidden)]
        #[rust_name = "qpersistentmodelindex_from_qmodelindex"]
        fn construct(index: &QModelIndex) -> QPersistentModelIndex;
        #[doc(hidden)]
        #[rust_name = "qpersistentmodelindex_clone"]
        fn construct(other: &QPersistentModelIndex) -> QPersistentModelIndex;
        #[doc(hidden)]
        #[rust_name = "qpersistentmodelindex_eq"]
        fn operatorEq(a: &QPersistentModelIndex, b: &QPersistentModelIndex) -> bool;
        #[doc(hidden)]
        #[rust_name = "qpersistentmodelindex_to_qstring"]
        fn toQString(value: &QPersistentModelIndex) -> QString;
    }
}

/// The QPersistentModelIndex class is used to locate data in a data model.
#[repr(C)]
pub struct QPersistentModelIndex {
    _space: MaybeUninit<usize>,
}

impl Clone for QPersistentModelIndex {
    /// Creates a new QPersistentModelIndex that is a copy of the other persistent model index.
    fn clone(&self) -> Self {
        ffi::qpersistentmodelindex_clone(self)
    }
}

impl Drop for QPersistentModelIndex {
    /// Destroys the persistent model index.
    fn drop(&mut self) {
        ffi::qpersistentmodelindex_drop(self)
    }
}

impl From<&crate::QModelIndex> for QPersistentModelIndex {
    /// Creates a new QPersistentModelIndex that is a copy of the model index.
    fn from(index: &crate::QModelIndex) -> Self {
        ffi::qpersistentmodelindex_from_qmodelindex(index)
    }
}

impl std::cmp::PartialEq for QPersistentModelIndex {
    fn eq(&self, other: &Self) -> bool {
        ffi::qpersistentmodelindex_eq(self, other)
    }
}

impl fmt::Display for QPersistentModelIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qpersistentmodelindex_to_qstring(self))
    }
}

impl fmt::Debug for QPersistentModelIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QPersistentModelIndex {
    type Id = type_id!("QPersistentModelIndex");
    type Kind = cxx::kind::Trivial;
}
