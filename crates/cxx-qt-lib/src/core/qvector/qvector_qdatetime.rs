// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = crate::QDateTime;

        include!("cxx-qt-lib/qvector.h");
        type QVector_QDateTime = crate::QVector<QDateTime>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QDateTime);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QDateTime, _: &QDateTime) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QDateTime"]
        fn construct(_: &QVector_QDateTime) -> QVector_QDateTime;
        #[rust_name = "qvector_default_QDateTime"]
        fn construct() -> QVector_QDateTime;
        #[rust_name = "qvector_drop_QDateTime"]
        fn drop(_: &mut QVector_QDateTime);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QDateTime"]
        fn qvectorReserve(_: &mut QVector_QDateTime, size: isize);
        #[rust_name = "append_QDateTime"]
        fn qvectorAppend(_: &mut QVector_QDateTime, _: &QDateTime);
        #[rust_name = "get_unchecked_QDateTime"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QDateTime, pos: isize) -> &QDateTime;
        #[rust_name = "index_of_QDateTime"]
        fn qvectorIndexOf(_: &QVector_QDateTime, _: &QDateTime) -> isize;
        #[rust_name = "insert_QDateTime"]
        fn qvectorInsert(_: &mut QVector_QDateTime, _: isize, _: &QDateTime);
        #[rust_name = "remove_QDateTime"]
        fn qvectorRemove(_: &mut QVector_QDateTime, _: isize);
        #[rust_name = "len_QDateTime"]
        fn qvectorLen(_: &QVector_QDateTime) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QDateTime, value: &ffi::QDateTime) {
    ffi::append_QDateTime(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QDateTime) -> ffi::QVector_QDateTime {
    ffi::qvector_clone_QDateTime(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QDateTime, size: isize) {
    ffi::reserve_QDateTime(v, size);
}

pub(crate) fn default() -> ffi::QVector_QDateTime {
    ffi::qvector_default_QDateTime()
}

pub(crate) fn drop(s: &mut ffi::QVector_QDateTime) {
    ffi::qvector_drop_QDateTime(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QDateTime, pos: isize) -> &ffi::QDateTime {
    ffi::get_unchecked_QDateTime(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QDateTime, value: &ffi::QDateTime) -> isize {
    ffi::index_of_QDateTime(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QDateTime, pos: isize, value: &ffi::QDateTime) {
    ffi::insert_QDateTime(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QDateTime) -> isize {
    ffi::len_QDateTime(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QDateTime, pos: isize) {
    ffi::remove_QDateTime(s, pos);
}
