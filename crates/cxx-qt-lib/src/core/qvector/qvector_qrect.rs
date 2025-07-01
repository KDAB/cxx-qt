// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;

        include!("cxx-qt-lib/core/qvector/qvector_QRect.h");
        type QVector_QRect = crate::QVector<QRect>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QRect);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QRect, _: &QRect) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QRect"]
        fn construct(_: &QVector_QRect) -> QVector_QRect;
        #[rust_name = "qvector_default_QRect"]
        fn construct() -> QVector_QRect;
        #[rust_name = "qvector_drop_QRect"]
        fn drop(_: &mut QVector_QRect);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QRect"]
        fn qvectorReserve(_: &mut QVector_QRect, size: isize);
        #[rust_name = "append_QRect"]
        fn qvectorAppend(_: &mut QVector_QRect, _: &QRect);
        #[rust_name = "get_unchecked_QRect"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QRect, pos: isize) -> &QRect;
        #[rust_name = "index_of_QRect"]
        fn qvectorIndexOf(_: &QVector_QRect, _: &QRect) -> isize;
        #[rust_name = "insert_QRect"]
        fn qvectorInsert(_: &mut QVector_QRect, _: isize, _: &QRect);
        #[rust_name = "remove_QRect"]
        fn qvectorRemove(_: &mut QVector_QRect, _: isize);
        #[rust_name = "len_QRect"]
        fn qvectorLen(_: &QVector_QRect) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QRect, value: &ffi::QRect) {
    ffi::append_QRect(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QRect) -> ffi::QVector_QRect {
    ffi::qvector_clone_QRect(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QRect, size: isize) {
    ffi::reserve_QRect(v, size);
}

pub(crate) fn default() -> ffi::QVector_QRect {
    ffi::qvector_default_QRect()
}

pub(crate) fn drop(s: &mut ffi::QVector_QRect) {
    ffi::qvector_drop_QRect(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QRect, pos: isize) -> &ffi::QRect {
    ffi::get_unchecked_QRect(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QRect, value: &ffi::QRect) -> isize {
    ffi::index_of_QRect(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QRect, pos: isize, value: &ffi::QRect) {
    ffi::insert_QRect(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QRect) -> isize {
    ffi::len_QRect(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QRect, pos: isize) {
    ffi::remove_QRect(s, pos);
}
