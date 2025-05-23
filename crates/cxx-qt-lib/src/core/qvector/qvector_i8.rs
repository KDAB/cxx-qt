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
        include!("cxx-qt-lib/core/qvector/qvector_i8.h");
        type QVector_i8 = crate::QVector<i8>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_i8);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_i8, _: &i8) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_i8"]
        fn construct(_: &QVector_i8) -> QVector_i8;
        #[rust_name = "qvector_default_i8"]
        fn construct() -> QVector_i8;
        #[rust_name = "qvector_drop_i8"]
        fn drop(_: &mut QVector_i8);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_i8"]
        fn qvectorReserve(_: &mut QVector_i8, size: isize);
        #[rust_name = "append_i8"]
        fn qvectorAppend(_: &mut QVector_i8, _: &i8);
        #[rust_name = "get_unchecked_i8"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_i8, pos: isize) -> &'a i8;
        #[rust_name = "index_of_i8"]
        fn qvectorIndexOf(_: &QVector_i8, _: &i8) -> isize;
        #[rust_name = "insert_i8"]
        fn qvectorInsert(_: &mut QVector_i8, _: isize, _: &i8);
        #[rust_name = "len_i8"]
        fn qvectorLen(_: &QVector_i8) -> isize;
        #[rust_name = "remove_i8"]
        fn qvectorRemove(_: &mut QVector_i8, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_i8, value: &i8) {
    ffi::append_i8(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_i8) -> ffi::QVector_i8 {
    ffi::qvector_clone_i8(v)
}

pub(crate) fn reserve(v: &mut ffi::QVector_i8, size: isize) {
    ffi::reserve_i8(v, size);
}

pub(crate) fn default() -> ffi::QVector_i8 {
    ffi::qvector_default_i8()
}

pub(crate) fn drop(v: &mut ffi::QVector_i8) {
    ffi::qvector_drop_i8(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_i8, pos: isize) -> &i8 {
    ffi::get_unchecked_i8(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_i8, value: &i8) -> isize {
    ffi::index_of_i8(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_i8, pos: isize, value: &i8) {
    ffi::insert_i8(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_i8) -> isize {
    ffi::len_i8(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_i8, pos: isize) {
    ffi::remove_i8(s, pos);
}
