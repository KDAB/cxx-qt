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
        include!("cxx-qt-lib/core/qvector/qvector_u16.h");
        type QVector_u16 = crate::QVector<u16>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_u16);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_u16, _: &u16) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_u16"]
        fn construct(_: &QVector_u16) -> QVector_u16;
        #[rust_name = "qvector_default_u16"]
        fn construct() -> QVector_u16;
        #[rust_name = "qvector_drop_u16"]
        fn drop(_: &mut QVector_u16);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_u16"]
        fn qvectorReserve(_: &mut QVector_u16, size: isize);
        #[rust_name = "append_u16"]
        fn qvectorAppend(_: &mut QVector_u16, _: &u16);
        #[rust_name = "get_unchecked_u16"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_u16, pos: isize) -> &'a u16;
        #[rust_name = "index_of_u16"]
        fn qvectorIndexOf(_: &QVector_u16, _: &u16) -> isize;
        #[rust_name = "insert_u16"]
        fn qvectorInsert(_: &mut QVector_u16, _: isize, _: &u16);
        #[rust_name = "len_u16"]
        fn qvectorLen(_: &QVector_u16) -> isize;
        #[rust_name = "remove_u16"]
        fn qvectorRemove(_: &mut QVector_u16, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_u16, value: &u16) {
    ffi::append_u16(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_u16) -> ffi::QVector_u16 {
    ffi::qvector_clone_u16(v)
}

pub(crate) fn reserve(v: &mut ffi::QVector_u16, size: isize) {
    ffi::reserve_u16(v, size);
}

pub(crate) fn default() -> ffi::QVector_u16 {
    ffi::qvector_default_u16()
}

pub(crate) fn drop(v: &mut ffi::QVector_u16) {
    ffi::qvector_drop_u16(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_u16, pos: isize) -> &u16 {
    ffi::get_unchecked_u16(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_u16, value: &u16) -> isize {
    ffi::index_of_u16(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_u16, pos: isize, value: &u16) {
    ffi::insert_u16(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_u16) -> isize {
    ffi::len_u16(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_u16, pos: isize) {
    ffi::remove_u16(s, pos);
}
