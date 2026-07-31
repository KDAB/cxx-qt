// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::QJsonValue;
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qjsonarray.h");
        type QJsonArray = super::QJsonArray;

        include!("cxx-qt-lib/qjsonvalue.h");
        type QJsonValue = crate::QJsonValue;

        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Inserts `value` at the end of the array.
        fn append(self: &mut QJsonArray, value: &QJsonValue);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qjsonarray_drop"]
        fn drop(qjsonarray: &mut QJsonArray);

        #[doc(hidden)]
        #[rust_name = "qjsonarray_init_default"]
        fn construct() -> QJsonArray;

        #[doc(hidden)]
        #[rust_name = "qjsonarray_init_from_qjsonarray"]
        fn construct(qjsonarray: &QJsonArray) -> QJsonArray;

        #[doc(hidden)]
        #[rust_name = "qjsonarray_eq"]
        fn operatorEq(a: &QJsonArray, b: &QJsonArray) -> bool;

        #[doc(hidden)]
        #[rust_name = "qjsonarray_to_debug_qstring"]
        fn toDebugQString(array: &QJsonArray) -> QString;

        #[doc(hidden)]
        #[rust_name = "qjsonarray_len"]
        fn qjsonarrayLen(array: &QJsonArray) -> isize;

        #[doc(hidden)]
        #[rust_name = "qjsonarray_at"]
        fn qjsonarrayAt(array: &QJsonArray, i: isize) -> QJsonValue;
    }
}

/// The `QJsonArray` class encapsulates a JSON array.
///
/// Qt Documentation: [QJsonArray](https://doc.qt.io/qt/qjsonarray.html#details)
#[repr(C)]
pub struct QJsonArray {
    #[cfg(cxxqt_qt_version_major = "5")]
    _d: MaybeUninit<usize>,
    _a: MaybeUninit<usize>,
}

impl Drop for QJsonArray {
    fn drop(&mut self) {
        ffi::qjsonarray_drop(self);
    }
}

impl Default for QJsonArray {
    fn default() -> Self {
        ffi::qjsonarray_init_default()
    }
}

impl Clone for QJsonArray {
    fn clone(&self) -> Self {
        ffi::qjsonarray_init_from_qjsonarray(self)
    }
}

impl PartialEq for QJsonArray {
    fn eq(&self, other: &Self) -> bool {
        ffi::qjsonarray_eq(self, other)
    }
}

impl fmt::Debug for QJsonArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qjsonarray_to_debug_qstring(self).fmt(f)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QJsonArray is trivial.
unsafe impl cxx::ExternType for QJsonArray {
    type Id = cxx::type_id!("QJsonArray");
    type Kind = cxx::kind::Trivial;
}

impl QJsonArray {
    /// Returns the number of elements in the array.
    pub fn len(&self) -> isize {
        ffi::qjsonarray_len(self)
    }

    /// Returns `true` if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the element at position `i`.
    /// The returned QJsonValue is Undefined, if `i` is out of bounds.
    pub fn at(&self, i: isize) -> QJsonValue {
        ffi::qjsonarray_at(self, i)
    }

    /// Returns an iterator over the elements of the array.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            array: self,
            index: 0,
        }
    }
}

/// An iterator over the elements of a [`QJsonArray`].
///
/// This struct is created by [`QJsonArray::iter`].
pub struct Iter<'a> {
    array: &'a QJsonArray,
    index: isize,
}

impl Iterator for Iter<'_> {
    type Item = QJsonValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.len() {
            return None;
        }
        let next = self.array.at(self.index);
        self.index += 1;
        Some(next)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        (self.array.len() - self.index) as usize
    }
}

impl<'a> IntoIterator for &'a QJsonArray {
    type Item = QJsonValue;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
