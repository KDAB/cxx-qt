// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;

        include!("cxx-qt-lib/qhash.h");
        type QHash_i32_QByteArray = crate::QHash<super::QHashPair_i32_QByteArray>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QHash_i32_QByteArray);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QHash_i32_QByteArray, key: &i32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhash_clone_i32_QByteArray"]
        fn construct(_: &QHash_i32_QByteArray) -> QHash_i32_QByteArray;
        #[rust_name = "qhash_default_i32_QByteArray"]
        fn construct() -> QHash_i32_QByteArray;
        #[rust_name = "qhash_drop_i32_QByteArray"]
        fn drop(_: &mut QHash_i32_QByteArray);
    }

    #[namespace = "rust::cxxqtlib1::qhash"]
    unsafe extern "C++" {
        #[rust_name = "get_or_default_i32_QByteArray"]
        fn qhashGetOrDefault(_: &QHash_i32_QByteArray, key: &i32) -> QByteArray;
        #[rust_name = "get_unchecked_key_i32_QByteArray"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qhashGetUncheckedKey<'a>(_: &'a QHash_i32_QByteArray, pos: isize) -> &'a i32;
        #[rust_name = "get_unchecked_value_i32_QByteArray"]
        unsafe fn qhashGetUncheckedValue(_: &QHash_i32_QByteArray, pos: isize) -> &QByteArray;
        #[rust_name = "insert_i32_QByteArray"]
        fn qhashInsert(_: &mut QHash_i32_QByteArray, key: &i32, value: &QByteArray);
        #[rust_name = "len_i32_QByteArray"]
        fn qhashLen(_: &QHash_i32_QByteArray) -> isize;
        #[rust_name = "remove_i32_QByteArray"]
        fn qhashRemove(_: &mut QHash_i32_QByteArray, key: &i32) -> bool;
        #[rust_name = "reserve_i32_QByteArray"]
        fn qhashReserve(_: &mut QHash_i32_QByteArray, size: isize);
    }
}

pub(crate) fn clone(hash: &ffi::QHash_i32_QByteArray) -> ffi::QHash_i32_QByteArray {
    ffi::qhash_clone_i32_QByteArray(hash)
}

pub(crate) fn default() -> ffi::QHash_i32_QByteArray {
    ffi::qhash_default_i32_QByteArray()
}

pub(crate) fn drop(hash: &mut ffi::QHash_i32_QByteArray) {
    ffi::qhash_drop_i32_QByteArray(hash);
}

pub(crate) fn get_or_default(hash: &ffi::QHash_i32_QByteArray, key: &i32) -> ffi::QByteArray {
    ffi::get_or_default_i32_QByteArray(hash, key)
}

pub(crate) unsafe fn get_unchecked_key(hash: &ffi::QHash_i32_QByteArray, pos: isize) -> &i32 {
    ffi::get_unchecked_key_i32_QByteArray(hash, pos)
}

pub(crate) unsafe fn get_unchecked_value(
    hash: &ffi::QHash_i32_QByteArray,
    pos: isize,
) -> &ffi::QByteArray {
    ffi::get_unchecked_value_i32_QByteArray(hash, pos)
}

pub(crate) fn insert(hash: &mut ffi::QHash_i32_QByteArray, key: &i32, value: &ffi::QByteArray) {
    ffi::insert_i32_QByteArray(hash, key, value);
}

pub(crate) fn len(hash: &ffi::QHash_i32_QByteArray) -> isize {
    ffi::len_i32_QByteArray(hash)
}

pub(crate) fn remove(hash: &mut ffi::QHash_i32_QByteArray, key: &i32) -> bool {
    ffi::remove_i32_QByteArray(hash, key)
}

pub(crate) fn reserve(hash: &mut ffi::QHash_i32_QByteArray, size: isize) {
    ffi::reserve_i32_QByteArray(hash, size)
}

#[allow(non_camel_case_types)]
pub struct QHashPair_i32_QByteArray;

unsafe impl ExternType for QHashPair_i32_QByteArray {
    type Id = type_id!("QHashPair_i32_QByteArray");
    type Kind = cxx::kind::Trivial;
}
