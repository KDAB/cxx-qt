// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{QJsonArray, QJsonObject, QString};
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qjsonvalue.h");
        type QJsonValue = super::QJsonValue;

        include!("cxx-qt-lib/qjsonarray.h");
        type QJsonArray = crate::QJsonArray;

        include!("cxx-qt-lib/qjsonobject.h");
        type QJsonObject = crate::QJsonObject;

        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns `true` if the value is null.
        #[rust_name = "is_null"]
        fn isNull(self: &QJsonValue) -> bool;

        /// Returns `true` if the value is undefined. This can occur for example if you
        /// query a non-existent key in a `QJsonObject`.
        #[rust_name = "is_undefined"]
        fn isUndefined(self: &QJsonValue) -> bool;

        /// Returns `true` if the value contains a boolean.
        #[rust_name = "is_bool"]
        fn isBool(self: &QJsonValue) -> bool;

        /// Returns `true` if the value contains a number (double).
        #[rust_name = "is_double"]
        fn isDouble(self: &QJsonValue) -> bool;

        /// Returns `true` if the value contains a string.
        #[rust_name = "is_string"]
        fn isString(self: &QJsonValue) -> bool;

        /// Returns `true` if the value contains an array.
        #[rust_name = "is_array"]
        fn isArray(self: &QJsonValue) -> bool;

        /// Returns `true` if the value contains an object.
        #[rust_name = "is_object"]
        fn isObject(self: &QJsonValue) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qjsonvalue_drop"]
        fn drop(value: &mut QJsonValue);

        #[doc(hidden)]
        #[rust_name = "qjsonvalue_init_default"]
        fn construct() -> QJsonValue;

        #[doc(hidden)]
        #[rust_name = "qjsonvalue_init_from_qjsonvalue"]
        fn construct(value: &QJsonValue) -> QJsonValue;

        #[doc(hidden)]
        #[rust_name = "qjsonvalue_init_from_bool"]
        fn construct(value: bool) -> QJsonValue;

        #[doc(hidden)]
        #[rust_name = "qjsonvalue_init_from_i64"]
        fn qjsonvalueFromI64(value: i64) -> QJsonValue;

        #[doc(hidden)]
        #[rust_name = "qjsonvalue_init_from_f64"]
        fn construct(value: f64) -> QJsonValue;

        #[doc(hidden)]
        #[rust_name = "qjsonvalue_init_from_qstring"]
        fn construct(value: &QString) -> QJsonValue;

        #[doc(hidden)]
        #[rust_name = "qjsonvalue_init_from_qjsonarray"]
        fn construct(value: &QJsonArray) -> QJsonValue;

        #[doc(hidden)]
        #[rust_name = "qjsonvalue_init_from_qjsonobject"]
        fn construct(value: &QJsonObject) -> QJsonValue;

        #[doc(hidden)]
        #[rust_name = "qjsonvalue_eq"]
        fn operatorEq(a: &QJsonValue, b: &QJsonValue) -> bool;

        #[doc(hidden)]
        #[rust_name = "qjsonvalue_to_debug_qstring"]
        fn toDebugQString(value: &QJsonValue) -> QString;

        #[doc(hidden)]
        #[rust_name = "to_bool"]
        fn qjsonvalueToBool(value: &QJsonValue) -> bool;

        #[doc(hidden)]
        #[rust_name = "to_double"]
        fn qjsonvalueToDouble(value: &QJsonValue) -> f64;

        #[doc(hidden)]
        #[rust_name = "to_int"]
        fn qjsonvalueToInt(value: &QJsonValue) -> i32;

        #[doc(hidden)]
        #[rust_name = "to_string"]
        fn qjsonvalueToQString(value: &QJsonValue) -> QString;

        #[doc(hidden)]
        #[rust_name = "to_array"]
        fn qjsonvalueToArray(value: &QJsonValue) -> QJsonArray;

        #[doc(hidden)]
        #[rust_name = "to_object"]
        fn qjsonvalueToObject(value: &QJsonValue) -> QJsonObject;
    }
}

/// The `QJsonValue` class encapsulates a value in JSON.
///
/// The default-constructed value is `QJsonValue::Null`.
///
/// Qt Documentation: [QJsonValue](https://doc.qt.io/qt/qjsonvalue.html#details)
#[repr(C)]
pub struct QJsonValue {
    _n: MaybeUninit<usize>,
    _container: MaybeUninit<usize>,
    _t: MaybeUninit<usize>,
}

impl Default for QJsonValue {
    /// Constructs a null value.
    fn default() -> Self {
        ffi::qjsonvalue_init_default()
    }
}

impl Drop for QJsonValue {
    fn drop(&mut self) {
        ffi::qjsonvalue_drop(self);
    }
}

impl Clone for QJsonValue {
    fn clone(&self) -> Self {
        ffi::qjsonvalue_init_from_qjsonvalue(self)
    }
}

impl PartialEq for QJsonValue {
    fn eq(&self, other: &Self) -> bool {
        ffi::qjsonvalue_eq(self, other)
    }
}

impl fmt::Debug for QJsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qjsonvalue_to_debug_qstring(self).fmt(f)
    }
}

impl From<bool> for QJsonValue {
    fn from(value: bool) -> Self {
        ffi::qjsonvalue_init_from_bool(value)
    }
}

impl From<i64> for QJsonValue {
    fn from(value: i64) -> Self {
        ffi::qjsonvalue_init_from_i64(value)
    }
}

impl From<f64> for QJsonValue {
    fn from(value: f64) -> Self {
        ffi::qjsonvalue_init_from_f64(value)
    }
}

impl From<&QString> for QJsonValue {
    fn from(value: &QString) -> Self {
        ffi::qjsonvalue_init_from_qstring(value)
    }
}

impl From<&QJsonArray> for QJsonValue {
    fn from(value: &QJsonArray) -> Self {
        ffi::qjsonvalue_init_from_qjsonarray(value)
    }
}

impl From<&QJsonObject> for QJsonValue {
    fn from(value: &QJsonObject) -> Self {
        ffi::qjsonvalue_init_from_qjsonobject(value)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QJsonValue is trivial.
unsafe impl cxx::ExternType for QJsonValue {
    type Id = cxx::type_id!("QJsonValue");
    type Kind = cxx::kind::Trivial;
}

impl QJsonValue {
    /// Converts the value to a `bool`. Returns `false` if the value is not a boolean.
    pub fn to_bool(&self) -> bool {
        ffi::to_bool(self)
    }

    /// Converts the value to a `f64`. Returns `0.0` if the value is not a double.
    pub fn to_double(&self) -> f64 {
        ffi::to_double(self)
    }

    /// Converts the value to a `i32`. Returns `0` if the value is not an integer.
    pub fn to_int(&self) -> i32 {
        ffi::to_int(self)
    }

    /// Converts the value to a [`QString`]. Returns an empty string if the value is
    /// not a string.
    pub fn to_string(&self) -> QString {
        ffi::to_string(self)
    }

    /// Converts the value to a [`QJsonArray`]. Returns an empty array if the value
    /// is not an array.
    pub fn to_array(&self) -> QJsonArray {
        ffi::to_array(self)
    }

    /// Converts the value to a [`QJsonObject`]. Returns an empty object if the value
    /// is not an object.
    pub fn to_object(&self) -> QJsonObject {
        ffi::to_object(self)
    }
}
