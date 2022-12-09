// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdate.h");
        type QDate = crate::QDate;

        include!("cxx-qt-lib/qvector.h");
        type QVector_QDate = crate::QVector<QDate>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QDate);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QDate, _: &QDate) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QDate"]
        fn construct(_: &QVector_QDate) -> QVector_QDate;
        #[rust_name = "qvector_default_QDate"]
        fn construct() -> QVector_QDate;
        #[rust_name = "qvector_drop_QDate"]
        fn drop(_: &mut QVector_QDate);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "append_QDate"]
        fn qvectorAppend(_: &mut QVector_QDate, _: &QDate);
        #[rust_name = "get_unchecked_QDate"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QDate, pos: isize) -> &QDate;
        #[rust_name = "index_of_QDate"]
        fn qvectorIndexOf(_: &QVector_QDate, _: &QDate) -> isize;
        #[rust_name = "insert_QDate"]
        fn qvectorInsert(_: &mut QVector_QDate, _: isize, _: &QDate);
        #[rust_name = "remove_QDate"]
        fn qvectorRemove(_: &mut QVector_QDate, _: isize);
        #[rust_name = "len_QDate"]
        fn qvectorLen(_: &QVector_QDate) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QDate, value: &ffi::QDate) {
    ffi::append_QDate(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QDate) -> ffi::QVector_QDate {
    ffi::qvector_clone_QDate(s)
}

pub(crate) fn default() -> ffi::QVector_QDate {
    ffi::qvector_default_QDate()
}

pub(crate) fn drop(s: &mut ffi::QVector_QDate) {
    ffi::qvector_drop_QDate(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QDate, pos: isize) -> &ffi::QDate {
    ffi::get_unchecked_QDate(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QDate, value: &ffi::QDate) -> isize {
    ffi::index_of_QDate(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QDate, pos: isize, value: &ffi::QDate) {
    ffi::insert_QDate(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QDate) -> isize {
    ffi::len_QDate(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QDate, pos: isize) {
    ffi::remove_QDate(s, pos);
}
