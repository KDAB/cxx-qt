// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector_i32.h");
        type QVector_i32 = crate::QVector<i32>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_i32);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_i32, _: &i32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_i32"]
        fn construct(_: &QVector_i32) -> QVector_i32;
        #[rust_name = "qvector_default_i32"]
        fn construct() -> QVector_i32;
        #[rust_name = "qvector_drop_i32"]
        fn drop(_: &mut QVector_i32);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_i32"]
        fn qvectorReserve(_: &mut QVector_i32, size: isize);
        #[rust_name = "append_i32"]
        fn qvectorAppend(_: &mut QVector_i32, _: &i32);
        #[rust_name = "get_unchecked_i32"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_i32, pos: isize) -> &'a i32;
        #[rust_name = "index_of_i32"]
        fn qvectorIndexOf(_: &QVector_i32, _: &i32) -> isize;
        #[rust_name = "insert_i32"]
        fn qvectorInsert(_: &mut QVector_i32, _: isize, _: &i32);
        #[rust_name = "len_i32"]
        fn qvectorLen(_: &QVector_i32) -> isize;
        #[rust_name = "remove_i32"]
        fn qvectorRemove(_: &mut QVector_i32, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_i32, value: &i32) {
    ffi::append_i32(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_i32) -> ffi::QVector_i32 {
    ffi::qvector_clone_i32(v)
}

pub(crate) fn reserve(v: &mut ffi::QVector_i32, size: isize) {
    ffi::reserve_i32(v, size);
}

pub(crate) fn default() -> ffi::QVector_i32 {
    ffi::qvector_default_i32()
}

pub(crate) fn drop(v: &mut ffi::QVector_i32) {
    ffi::qvector_drop_i32(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_i32, pos: isize) -> &i32 {
    ffi::get_unchecked_i32(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_i32, value: &i32) -> isize {
    ffi::index_of_i32(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_i32, pos: isize, value: &i32) {
    ffi::insert_i32(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_i32) -> isize {
    ffi::len_i32(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_i32, pos: isize) {
    ffi::remove_i32(s, pos);
}
