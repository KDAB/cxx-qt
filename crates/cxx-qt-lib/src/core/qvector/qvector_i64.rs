// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//! This is an auto-generated file. Do not edit.
//! Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector_i64.h");
        type QVector_i64 = crate::QVector<i64>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_i64);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_i64, _: &i64) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_i64"]
        fn construct(_: &QVector_i64) -> QVector_i64;
        #[rust_name = "qvector_default_i64"]
        fn construct() -> QVector_i64;
        #[rust_name = "qvector_drop_i64"]
        fn drop(_: &mut QVector_i64);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_i64"]
        fn qvectorReserve(_: &mut QVector_i64, size: isize);
        #[rust_name = "append_i64"]
        fn qvectorAppend(_: &mut QVector_i64, _: &i64);
        #[rust_name = "get_unchecked_i64"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_i64, pos: isize) -> &'a i64;
        #[rust_name = "index_of_i64"]
        fn qvectorIndexOf(_: &QVector_i64, _: &i64) -> isize;
        #[rust_name = "insert_i64"]
        fn qvectorInsert(_: &mut QVector_i64, _: isize, _: &i64);
        #[rust_name = "len_i64"]
        fn qvectorLen(_: &QVector_i64) -> isize;
        #[rust_name = "remove_i64"]
        fn qvectorRemove(_: &mut QVector_i64, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_i64, value: &i64) {
    ffi::append_i64(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_i64) -> ffi::QVector_i64 {
    ffi::qvector_clone_i64(v)
}

pub(crate) fn reserve(v: &mut ffi::QVector_i64, size: isize) {
    ffi::reserve_i64(v, size);
}

pub(crate) fn default() -> ffi::QVector_i64 {
    ffi::qvector_default_i64()
}

pub(crate) fn drop(v: &mut ffi::QVector_i64) {
    ffi::qvector_drop_i64(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_i64, pos: isize) -> &i64 {
    ffi::get_unchecked_i64(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_i64, value: &i64) -> isize {
    ffi::index_of_i64(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_i64, pos: isize, value: &i64) {
    ffi::insert_i64(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_i64) -> isize {
    ffi::len_i64(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_i64, pos: isize) {
    ffi::remove_i64(s, pos);
}
