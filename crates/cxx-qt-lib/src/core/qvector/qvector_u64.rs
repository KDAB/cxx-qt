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
        include!("cxx-qt-lib/qvector_u64.h");
        type QVector_u64 = crate::QVector<u64>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_u64);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_u64, _: &u64) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_u64"]
        fn construct(_: &QVector_u64) -> QVector_u64;
        #[rust_name = "qvector_default_u64"]
        fn construct() -> QVector_u64;
        #[rust_name = "qvector_drop_u64"]
        fn drop(_: &mut QVector_u64);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_u64"]
        fn qvectorReserve(_: &mut QVector_u64, size: isize);
        #[rust_name = "append_u64"]
        fn qvectorAppend(_: &mut QVector_u64, _: &u64);
        #[rust_name = "get_unchecked_u64"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_u64, pos: isize) -> &'a u64;
        #[rust_name = "index_of_u64"]
        fn qvectorIndexOf(_: &QVector_u64, _: &u64) -> isize;
        #[rust_name = "insert_u64"]
        fn qvectorInsert(_: &mut QVector_u64, _: isize, _: &u64);
        #[rust_name = "len_u64"]
        fn qvectorLen(_: &QVector_u64) -> isize;
        #[rust_name = "remove_u64"]
        fn qvectorRemove(_: &mut QVector_u64, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_u64, value: &u64) {
    ffi::append_u64(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_u64) -> ffi::QVector_u64 {
    ffi::qvector_clone_u64(v)
}

pub(crate) fn reserve(v: &mut ffi::QVector_u64, size: isize) {
    ffi::reserve_u64(v, size);
}

pub(crate) fn default() -> ffi::QVector_u64 {
    ffi::qvector_default_u64()
}

pub(crate) fn drop(v: &mut ffi::QVector_u64) {
    ffi::qvector_drop_u64(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_u64, pos: isize) -> &u64 {
    ffi::get_unchecked_u64(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_u64, value: &u64) -> isize {
    ffi::index_of_u64(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_u64, pos: isize, value: &u64) {
    ffi::insert_u64(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_u64) -> isize {
    ffi::len_u64(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_u64, pos: isize) {
    ffi::remove_u64(s, pos);
}
