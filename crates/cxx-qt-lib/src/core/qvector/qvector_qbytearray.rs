// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;

        include!("cxx-qt-lib/qvector_QByteArray.h");
        type QVector_QByteArray = crate::QVector<QByteArray>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QByteArray);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QByteArray, _: &QByteArray) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QByteArray"]
        fn construct(_: &QVector_QByteArray) -> QVector_QByteArray;
        #[rust_name = "qvector_default_QByteArray"]
        fn construct() -> QVector_QByteArray;
        #[rust_name = "qvector_drop_QByteArray"]
        fn drop(_: &mut QVector_QByteArray);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QByteArray"]
        fn qvectorReserve(_: &mut QVector_QByteArray, size: isize);
        #[rust_name = "append_QByteArray"]
        fn qvectorAppend(_: &mut QVector_QByteArray, _: &QByteArray);
        #[rust_name = "get_unchecked_QByteArray"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QByteArray, pos: isize) -> &QByteArray;
        #[rust_name = "index_of_QByteArray"]
        fn qvectorIndexOf(_: &QVector_QByteArray, _: &QByteArray) -> isize;
        #[rust_name = "insert_QByteArray"]
        fn qvectorInsert(_: &mut QVector_QByteArray, _: isize, _: &QByteArray);
        #[rust_name = "remove_QByteArray"]
        fn qvectorRemove(_: &mut QVector_QByteArray, _: isize);
        #[rust_name = "len_QByteArray"]
        fn qvectorLen(_: &QVector_QByteArray) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QByteArray, value: &ffi::QByteArray) {
    ffi::append_QByteArray(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QByteArray) -> ffi::QVector_QByteArray {
    ffi::qvector_clone_QByteArray(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QByteArray, size: isize) {
    ffi::reserve_QByteArray(v, size);
}

pub(crate) fn default() -> ffi::QVector_QByteArray {
    ffi::qvector_default_QByteArray()
}

pub(crate) fn drop(s: &mut ffi::QVector_QByteArray) {
    ffi::qvector_drop_QByteArray(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QByteArray, pos: isize) -> &ffi::QByteArray {
    ffi::get_unchecked_QByteArray(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QByteArray, value: &ffi::QByteArray) -> isize {
    ffi::index_of_QByteArray(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QByteArray, pos: isize, value: &ffi::QByteArray) {
    ffi::insert_QByteArray(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QByteArray) -> isize {
    ffi::len_QByteArray(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QByteArray, pos: isize) {
    ffi::remove_QByteArray(s, pos);
}
