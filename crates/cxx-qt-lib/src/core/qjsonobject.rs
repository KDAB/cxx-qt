// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{QJsonValue, QString};
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qjsonobject.h");
        type QJsonObject = super::QJsonObject;

        include!("cxx-qt-lib/qjsonvalue.h");
        type QJsonValue = crate::QJsonValue;

        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;

        /// Returns `true` if the object is empty.
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QJsonObject) -> bool;

        /// Returns `true` if the object contains `key`.
        fn contains(self: &QJsonObject, key: &QString) -> bool;

        /// Returns the value for `key`, or an undefined [`QJsonValue`] if the key is absent.
        fn value(self: &QJsonObject, key: &QString) -> QJsonValue;

        /// Removes `key` from the object.
        fn remove(self: &mut QJsonObject, key: &QString);

        /// Returns all keys in the object.
        fn keys(self: &QJsonObject) -> QStringList;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qjsonobject_drop"]
        fn drop(object: &mut QJsonObject);

        #[doc(hidden)]
        #[rust_name = "qjsonobject_init_default"]
        fn construct() -> QJsonObject;

        #[doc(hidden)]
        #[rust_name = "qjsonobject_init_from_qjsonobject"]
        fn construct(object: &QJsonObject) -> QJsonObject;

        #[doc(hidden)]
        #[rust_name = "qjsonobject_eq"]
        fn operatorEq(a: &QJsonObject, b: &QJsonObject) -> bool;

        #[doc(hidden)]
        #[rust_name = "qjsonobject_to_debug_qstring"]
        fn toDebugQString(object: &QJsonObject) -> QString;

        #[doc(hidden)]
        #[rust_name = "qjsonobject_len"]
        fn qjsonobjectLen(object: &QJsonObject) -> isize;

        #[doc(hidden)]
        #[rust_name = "qjsonobject_insert"]
        fn qjsonobjectInsert(object: &mut QJsonObject, key: &QString, value: &QJsonValue);
    }
}

/// The `QJsonObject` class encapsulates a JSON object.
///
/// Qt Documentation: [QJsonObject](https://doc.qt.io/qt/qjsonobject.html#details)
#[repr(C)]
pub struct QJsonObject {
    #[cfg(cxxqt_qt_version_major = "5")]
    _d: MaybeUninit<usize>,
    _o: MaybeUninit<usize>,
}

impl Default for QJsonObject {
    fn default() -> Self {
        ffi::qjsonobject_init_default()
    }
}

impl Drop for QJsonObject {
    fn drop(&mut self) {
        ffi::qjsonobject_drop(self);
    }
}

impl Clone for QJsonObject {
    fn clone(&self) -> Self {
        ffi::qjsonobject_init_from_qjsonobject(self)
    }
}

impl PartialEq for QJsonObject {
    fn eq(&self, other: &Self) -> bool {
        ffi::qjsonobject_eq(self, other)
    }
}

impl fmt::Debug for QJsonObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qjsonobject_to_debug_qstring(self).fmt(f)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QJsonObject is trivial.
unsafe impl cxx::ExternType for QJsonObject {
    type Id = cxx::type_id!("QJsonObject");
    type Kind = cxx::kind::Trivial;
}

impl QJsonObject {
    /// Returns the number of key-value pairs in the object.
    pub fn len(&self) -> isize {
        ffi::qjsonobject_len(self)
    }

    /// Inserts or replaces the value for `key`.
    pub fn insert(&mut self, key: &QString, value: &QJsonValue) {
        ffi::qjsonobject_insert(self, key, value);
    }
}
