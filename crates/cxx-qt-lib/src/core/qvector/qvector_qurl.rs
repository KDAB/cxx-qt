// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = crate::QUrl;

        include!("cxx-qt-lib/core/qvector/qvector_QUrl.h");
        type QVector_QUrl = crate::QVector<QUrl>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QUrl);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QUrl, _: &QUrl) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QUrl"]
        fn construct(_: &QVector_QUrl) -> QVector_QUrl;
        #[rust_name = "qvector_default_QUrl"]
        fn construct() -> QVector_QUrl;
        #[rust_name = "qvector_drop_QUrl"]
        fn drop(_: &mut QVector_QUrl);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QUrl"]
        fn qvectorReserve(_: &mut QVector_QUrl, size: isize);
        #[rust_name = "append_QUrl"]
        fn qvectorAppend(_: &mut QVector_QUrl, _: &QUrl);
        #[rust_name = "get_unchecked_QUrl"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QUrl, pos: isize) -> &QUrl;
        #[rust_name = "index_of_QUrl"]
        fn qvectorIndexOf(_: &QVector_QUrl, _: &QUrl) -> isize;
        #[rust_name = "insert_QUrl"]
        fn qvectorInsert(_: &mut QVector_QUrl, _: isize, _: &QUrl);
        #[rust_name = "remove_QUrl"]
        fn qvectorRemove(_: &mut QVector_QUrl, _: isize);
        #[rust_name = "len_QUrl"]
        fn qvectorLen(_: &QVector_QUrl) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QUrl, value: &ffi::QUrl) {
    ffi::append_QUrl(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QUrl) -> ffi::QVector_QUrl {
    ffi::qvector_clone_QUrl(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QUrl, size: isize) {
    ffi::reserve_QUrl(v, size);
}

pub(crate) fn default() -> ffi::QVector_QUrl {
    ffi::qvector_default_QUrl()
}

pub(crate) fn drop(s: &mut ffi::QVector_QUrl) {
    ffi::qvector_drop_QUrl(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QUrl, pos: isize) -> &ffi::QUrl {
    ffi::get_unchecked_QUrl(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QUrl, value: &ffi::QUrl) -> isize {
    ffi::index_of_QUrl(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QUrl, pos: isize, value: &ffi::QUrl) {
    ffi::insert_QUrl(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QUrl) -> isize {
    ffi::len_QUrl(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QUrl, pos: isize) {
    ffi::remove_QUrl(s, pos);
}
