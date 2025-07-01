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
        include!("cxx-qt-lib/core/qvector/qvector_i16.h");
        type QVector_i16 = crate::QVector<i16>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_i16);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_i16, _: &i16) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_i16"]
        fn construct(_: &QVector_i16) -> QVector_i16;
        #[rust_name = "qvector_default_i16"]
        fn construct() -> QVector_i16;
        #[rust_name = "qvector_drop_i16"]
        fn drop(_: &mut QVector_i16);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_i16"]
        fn qvectorReserve(_: &mut QVector_i16, size: isize);
        #[rust_name = "append_i16"]
        fn qvectorAppend(_: &mut QVector_i16, _: &i16);
        #[rust_name = "get_unchecked_i16"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_i16, pos: isize) -> &'a i16;
        #[rust_name = "index_of_i16"]
        fn qvectorIndexOf(_: &QVector_i16, _: &i16) -> isize;
        #[rust_name = "insert_i16"]
        fn qvectorInsert(_: &mut QVector_i16, _: isize, _: &i16);
        #[rust_name = "len_i16"]
        fn qvectorLen(_: &QVector_i16) -> isize;
        #[rust_name = "remove_i16"]
        fn qvectorRemove(_: &mut QVector_i16, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_i16, value: &i16) {
    ffi::append_i16(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_i16) -> ffi::QVector_i16 {
    ffi::qvector_clone_i16(v)
}

pub(crate) fn reserve(v: &mut ffi::QVector_i16, size: isize) {
    ffi::reserve_i16(v, size);
}

pub(crate) fn default() -> ffi::QVector_i16 {
    ffi::qvector_default_i16()
}

pub(crate) fn drop(v: &mut ffi::QVector_i16) {
    ffi::qvector_drop_i16(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_i16, pos: isize) -> &i16 {
    ffi::get_unchecked_i16(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_i16, value: &i16) -> isize {
    ffi::index_of_i16(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_i16, pos: isize, value: &i16) {
    ffi::insert_i16(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_i16) -> isize {
    ffi::len_i16(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_i16, pos: isize) {
    ffi::remove_i16(s, pos);
}
