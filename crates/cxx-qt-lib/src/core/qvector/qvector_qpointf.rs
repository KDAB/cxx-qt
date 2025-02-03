// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;

        include!("cxx-qt-lib/qvector_QPointF.h");
        type QVector_QPointF = crate::QVector<QPointF>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QPointF);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QPointF, _: &QPointF) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QPointF"]
        fn construct(_: &QVector_QPointF) -> QVector_QPointF;
        #[rust_name = "qvector_default_QPointF"]
        fn construct() -> QVector_QPointF;
        #[rust_name = "qvector_drop_QPointF"]
        fn drop(_: &mut QVector_QPointF);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QPointF"]
        fn qvectorReserve(_: &mut QVector_QPointF, size: isize);
        #[rust_name = "append_QPointF"]
        fn qvectorAppend(_: &mut QVector_QPointF, _: &QPointF);
        #[rust_name = "get_unchecked_QPointF"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QPointF, pos: isize) -> &QPointF;
        #[rust_name = "index_of_QPointF"]
        fn qvectorIndexOf(_: &QVector_QPointF, _: &QPointF) -> isize;
        #[rust_name = "insert_QPointF"]
        fn qvectorInsert(_: &mut QVector_QPointF, _: isize, _: &QPointF);
        #[rust_name = "remove_QPointF"]
        fn qvectorRemove(_: &mut QVector_QPointF, _: isize);
        #[rust_name = "len_QPointF"]
        fn qvectorLen(_: &QVector_QPointF) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QPointF, value: &ffi::QPointF) {
    ffi::append_QPointF(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QPointF) -> ffi::QVector_QPointF {
    ffi::qvector_clone_QPointF(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QPointF, size: isize) {
    ffi::reserve_QPointF(v, size);
}

pub(crate) fn default() -> ffi::QVector_QPointF {
    ffi::qvector_default_QPointF()
}

pub(crate) fn drop(s: &mut ffi::QVector_QPointF) {
    ffi::qvector_drop_QPointF(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QPointF, pos: isize) -> &ffi::QPointF {
    ffi::get_unchecked_QPointF(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QPointF, value: &ffi::QPointF) -> isize {
    ffi::index_of_QPointF(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QPointF, pos: isize, value: &ffi::QPointF) {
    ffi::insert_QPointF(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QPointF) -> isize {
    ffi::len_QPointF(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QPointF, pos: isize) {
    ffi::remove_QPointF(s, pos);
}
