// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/core/qvector/qvector_QString.h");
        type QVector_QString = crate::QVector<QString>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QString);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QString, _: &QString) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QString"]
        fn construct(_: &QVector_QString) -> QVector_QString;
        #[rust_name = "qvector_default_QString"]
        fn construct() -> QVector_QString;
        #[rust_name = "qvector_drop_QString"]
        fn drop(_: &mut QVector_QString);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QString"]
        fn qvectorReserve(_: &mut QVector_QString, size: isize);
        #[rust_name = "append_QString"]
        fn qvectorAppend(_: &mut QVector_QString, _: &QString);
        #[rust_name = "get_unchecked_QString"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QString, pos: isize) -> &QString;
        #[rust_name = "index_of_QString"]
        fn qvectorIndexOf(_: &QVector_QString, _: &QString) -> isize;
        #[rust_name = "insert_QString"]
        fn qvectorInsert(_: &mut QVector_QString, _: isize, _: &QString);
        #[rust_name = "remove_QString"]
        fn qvectorRemove(_: &mut QVector_QString, _: isize);
        #[rust_name = "len_QString"]
        fn qvectorLen(_: &QVector_QString) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QString, value: &ffi::QString) {
    ffi::append_QString(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QString) -> ffi::QVector_QString {
    ffi::qvector_clone_QString(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QString, size: isize) {
    ffi::reserve_QString(v, size);
}

pub(crate) fn default() -> ffi::QVector_QString {
    ffi::qvector_default_QString()
}

pub(crate) fn drop(s: &mut ffi::QVector_QString) {
    ffi::qvector_drop_QString(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QString, pos: isize) -> &ffi::QString {
    ffi::get_unchecked_QString(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QString, value: &ffi::QString) -> isize {
    ffi::index_of_QString(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QString, pos: isize, value: &ffi::QString) {
    ffi::insert_QString(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QString) -> isize {
    ffi::len_QString(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QString, pos: isize) {
    ffi::remove_QString(s, pos);
}
