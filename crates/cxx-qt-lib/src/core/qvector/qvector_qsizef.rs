// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = crate::QSizeF;

        include!("cxx-qt-lib/qvector_QSizeF.h");
        type QVector_QSizeF = crate::QVector<QSizeF>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QSizeF);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QSizeF, _: &QSizeF) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QSizeF"]
        fn construct(_: &QVector_QSizeF) -> QVector_QSizeF;
        #[rust_name = "qvector_default_QSizeF"]
        fn construct() -> QVector_QSizeF;
        #[rust_name = "qvector_drop_QSizeF"]
        fn drop(_: &mut QVector_QSizeF);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QSizeF"]
        fn qvectorReserve(_: &mut QVector_QSizeF, size: isize);
        #[rust_name = "append_QSizeF"]
        fn qvectorAppend(_: &mut QVector_QSizeF, _: &QSizeF);
        #[rust_name = "get_unchecked_QSizeF"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QSizeF, pos: isize) -> &QSizeF;
        #[rust_name = "index_of_QSizeF"]
        fn qvectorIndexOf(_: &QVector_QSizeF, _: &QSizeF) -> isize;
        #[rust_name = "insert_QSizeF"]
        fn qvectorInsert(_: &mut QVector_QSizeF, _: isize, _: &QSizeF);
        #[rust_name = "remove_QSizeF"]
        fn qvectorRemove(_: &mut QVector_QSizeF, _: isize);
        #[rust_name = "len_QSizeF"]
        fn qvectorLen(_: &QVector_QSizeF) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QSizeF, value: &ffi::QSizeF) {
    ffi::append_QSizeF(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QSizeF) -> ffi::QVector_QSizeF {
    ffi::qvector_clone_QSizeF(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QSizeF, size: isize) {
    ffi::reserve_QSizeF(v, size);
}

pub(crate) fn default() -> ffi::QVector_QSizeF {
    ffi::qvector_default_QSizeF()
}

pub(crate) fn drop(s: &mut ffi::QVector_QSizeF) {
    ffi::qvector_drop_QSizeF(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QSizeF, pos: isize) -> &ffi::QSizeF {
    ffi::get_unchecked_QSizeF(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QSizeF, value: &ffi::QSizeF) -> isize {
    ffi::index_of_QSizeF(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QSizeF, pos: isize, value: &ffi::QSizeF) {
    ffi::insert_QSizeF(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QSizeF) -> isize {
    ffi::len_QSizeF(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QSizeF, pos: isize) {
    ffi::remove_QSizeF(s, pos);
}
