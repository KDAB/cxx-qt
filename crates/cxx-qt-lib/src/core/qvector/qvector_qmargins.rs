// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmargins.h");
        type QMargins = crate::QMargins;

        include!("cxx-qt-lib/qvector_QMargins.h");
        type QVector_QMargins = crate::QVector<QMargins>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QMargins);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QMargins, _: &QMargins) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QMargins"]
        fn construct(_: &QVector_QMargins) -> QVector_QMargins;
        #[rust_name = "qvector_default_QMargins"]
        fn construct() -> QVector_QMargins;
        #[rust_name = "qvector_drop_QMargins"]
        fn drop(_: &mut QVector_QMargins);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QMargins"]
        fn qvectorReserve(_: &mut QVector_QMargins, size: isize);
        #[rust_name = "append_QMargins"]
        fn qvectorAppend(_: &mut QVector_QMargins, _: &QMargins);
        #[rust_name = "get_unchecked_QMargins"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QMargins, pos: isize) -> &QMargins;
        #[rust_name = "index_of_QMargins"]
        fn qvectorIndexOf(_: &QVector_QMargins, _: &QMargins) -> isize;
        #[rust_name = "insert_QMargins"]
        fn qvectorInsert(_: &mut QVector_QMargins, _: isize, _: &QMargins);
        #[rust_name = "remove_QMargins"]
        fn qvectorRemove(_: &mut QVector_QMargins, _: isize);
        #[rust_name = "len_QMargins"]
        fn qvectorLen(_: &QVector_QMargins) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QMargins, value: &ffi::QMargins) {
    ffi::append_QMargins(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QMargins) -> ffi::QVector_QMargins {
    ffi::qvector_clone_QMargins(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QMargins, size: isize) {
    ffi::reserve_QMargins(v, size);
}

pub(crate) fn default() -> ffi::QVector_QMargins {
    ffi::qvector_default_QMargins()
}

pub(crate) fn drop(s: &mut ffi::QVector_QMargins) {
    ffi::qvector_drop_QMargins(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QMargins, pos: isize) -> &ffi::QMargins {
    ffi::get_unchecked_QMargins(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QMargins, value: &ffi::QMargins) -> isize {
    ffi::index_of_QMargins(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QMargins, pos: isize, value: &ffi::QMargins) {
    ffi::insert_QMargins(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QMargins) -> isize {
    ffi::len_QMargins(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QMargins, pos: isize) {
    ffi::remove_QMargins(s, pos);
}
