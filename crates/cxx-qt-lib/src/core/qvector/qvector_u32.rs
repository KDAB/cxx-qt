// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector_u32.h");
        type QVector_u32 = crate::QVector<u32>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_u32);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_u32, _: &u32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_u32"]
        fn construct(_: &QVector_u32) -> QVector_u32;
        #[rust_name = "qvector_default_u32"]
        fn construct() -> QVector_u32;
        #[rust_name = "qvector_drop_u32"]
        fn drop(_: &mut QVector_u32);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_u32"]
        fn qvectorReserve(_: &mut QVector_u32, size: isize);
        #[rust_name = "append_u32"]
        fn qvectorAppend(_: &mut QVector_u32, _: &u32);
        #[rust_name = "get_unchecked_u32"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_u32, pos: isize) -> &'a u32;
        #[rust_name = "index_of_u32"]
        fn qvectorIndexOf(_: &QVector_u32, _: &u32) -> isize;
        #[rust_name = "insert_u32"]
        fn qvectorInsert(_: &mut QVector_u32, _: isize, _: &u32);
        #[rust_name = "len_u32"]
        fn qvectorLen(_: &QVector_u32) -> isize;
        #[rust_name = "remove_u32"]
        fn qvectorRemove(_: &mut QVector_u32, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_u32, value: &u32) {
    ffi::append_u32(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_u32) -> ffi::QVector_u32 {
    ffi::qvector_clone_u32(v)
}

pub(crate) fn reserve(v: &mut ffi::QVector_u32, size: isize) {
    ffi::reserve_u32(v, size);
}

pub(crate) fn default() -> ffi::QVector_u32 {
    ffi::qvector_default_u32()
}

pub(crate) fn drop(v: &mut ffi::QVector_u32) {
    ffi::qvector_drop_u32(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_u32, pos: isize) -> &u32 {
    ffi::get_unchecked_u32(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_u32, value: &u32) -> isize {
    ffi::index_of_u32(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_u32, pos: isize, value: &u32) {
    ffi::insert_u32(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_u32) -> isize {
    ffi::len_u32(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_u32, pos: isize) {
    ffi::remove_u32(s, pos);
}
