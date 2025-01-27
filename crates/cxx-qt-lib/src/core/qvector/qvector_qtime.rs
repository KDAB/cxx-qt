// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qtime.h");
        type QTime = crate::QTime;

        include!("cxx-qt-lib/qvector_QTime.h");
        type QVector_QTime = crate::QVector<QTime>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QTime);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QTime, _: &QTime) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QTime"]
        fn construct(_: &QVector_QTime) -> QVector_QTime;
        #[rust_name = "qvector_default_QTime"]
        fn construct() -> QVector_QTime;
        #[rust_name = "qvector_drop_QTime"]
        fn drop(_: &mut QVector_QTime);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QTime"]
        fn qvectorReserve(_: &mut QVector_QTime, size: isize);
        #[rust_name = "append_QTime"]
        fn qvectorAppend(_: &mut QVector_QTime, _: &QTime);
        #[rust_name = "get_unchecked_QTime"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QTime, pos: isize) -> &QTime;
        #[rust_name = "index_of_QTime"]
        fn qvectorIndexOf(_: &QVector_QTime, _: &QTime) -> isize;
        #[rust_name = "insert_QTime"]
        fn qvectorInsert(_: &mut QVector_QTime, _: isize, _: &QTime);
        #[rust_name = "remove_QTime"]
        fn qvectorRemove(_: &mut QVector_QTime, _: isize);
        #[rust_name = "len_QTime"]
        fn qvectorLen(_: &QVector_QTime) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QTime, value: &ffi::QTime) {
    ffi::append_QTime(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QTime) -> ffi::QVector_QTime {
    ffi::qvector_clone_QTime(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QTime, size: isize) {
    ffi::reserve_QTime(v, size);
}

pub(crate) fn default() -> ffi::QVector_QTime {
    ffi::qvector_default_QTime()
}

pub(crate) fn drop(s: &mut ffi::QVector_QTime) {
    ffi::qvector_drop_QTime(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QTime, pos: isize) -> &ffi::QTime {
    ffi::get_unchecked_QTime(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QTime, value: &ffi::QTime) -> isize {
    ffi::index_of_QTime(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QTime, pos: isize, value: &ffi::QTime) {
    ffi::insert_QTime(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QTime) -> isize {
    ffi::len_QTime(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QTime, pos: isize) {
    ffi::remove_QTime(s, pos);
}
