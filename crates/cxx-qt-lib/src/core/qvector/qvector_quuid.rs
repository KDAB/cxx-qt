// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/quuid.h");
        type QUuid = crate::QUuid;

        include!("cxx-qt-lib/core/qvector/qvector_QUuid.h");
        type QVector_QUuid = crate::QVector<QUuid>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QUuid);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QUuid, _: &QUuid) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QUuid"]
        fn construct(_: &QVector_QUuid) -> QVector_QUuid;
        #[rust_name = "qvector_default_QUuid"]
        fn construct() -> QVector_QUuid;
        #[rust_name = "qvector_drop_QUuid"]
        fn drop(_: &mut QVector_QUuid);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QUuid"]
        fn qvectorReserve(_: &mut QVector_QUuid, size: isize);
        #[rust_name = "append_QUuid"]
        fn qvectorAppend(_: &mut QVector_QUuid, _: &QUuid);
        #[rust_name = "get_unchecked_QUuid"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QUuid, pos: isize) -> &QUuid;
        #[rust_name = "index_of_QUuid"]
        fn qvectorIndexOf(_: &QVector_QUuid, _: &QUuid) -> isize;
        #[rust_name = "insert_QUuid"]
        fn qvectorInsert(_: &mut QVector_QUuid, _: isize, _: &QUuid);
        #[rust_name = "remove_QUuid"]
        fn qvectorRemove(_: &mut QVector_QUuid, _: isize);
        #[rust_name = "len_QUuid"]
        fn qvectorLen(_: &QVector_QUuid) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QUuid, value: &ffi::QUuid) {
    ffi::append_QUuid(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QUuid) -> ffi::QVector_QUuid {
    ffi::qvector_clone_QUuid(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QUuid, size: isize) {
    ffi::reserve_QUuid(v, size);
}

pub(crate) fn default() -> ffi::QVector_QUuid {
    ffi::qvector_default_QUuid()
}

pub(crate) fn drop(s: &mut ffi::QVector_QUuid) {
    ffi::qvector_drop_QUuid(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QUuid, pos: isize) -> &ffi::QUuid {
    ffi::get_unchecked_QUuid(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QUuid, value: &ffi::QUuid) -> isize {
    ffi::index_of_QUuid(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QUuid, pos: isize, value: &ffi::QUuid) {
    ffi::insert_QUuid(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QUuid) -> isize {
    ffi::len_QUuid(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QUuid, pos: isize) {
    ffi::remove_QUuid(s, pos);
}
