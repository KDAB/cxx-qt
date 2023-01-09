// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qsize.h");
        type QSize = crate::QSize;

        include!("cxx-qt-lib/qvector.h");
        type QVector_QSize = crate::QVector<QSize>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QSize);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QSize, _: &QSize) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QSize"]
        fn construct(_: &QVector_QSize) -> QVector_QSize;
        #[rust_name = "qvector_default_QSize"]
        fn construct() -> QVector_QSize;
        #[rust_name = "qvector_drop_QSize"]
        fn drop(_: &mut QVector_QSize);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QSize"]
        fn qvectorReserve(_: &mut QVector_QSize, size: isize);
        #[rust_name = "append_QSize"]
        fn qvectorAppend(_: &mut QVector_QSize, _: &QSize);
        #[rust_name = "get_unchecked_QSize"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QSize, pos: isize) -> &QSize;
        #[rust_name = "index_of_QSize"]
        fn qvectorIndexOf(_: &QVector_QSize, _: &QSize) -> isize;
        #[rust_name = "insert_QSize"]
        fn qvectorInsert(_: &mut QVector_QSize, _: isize, _: &QSize);
        #[rust_name = "remove_QSize"]
        fn qvectorRemove(_: &mut QVector_QSize, _: isize);
        #[rust_name = "len_QSize"]
        fn qvectorLen(_: &QVector_QSize) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QSize, value: &ffi::QSize) {
    ffi::append_QSize(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QSize) -> ffi::QVector_QSize {
    ffi::qvector_clone_QSize(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QSize, size: isize) {
    ffi::reserve_QSize(v, size);
}

pub(crate) fn default() -> ffi::QVector_QSize {
    ffi::qvector_default_QSize()
}

pub(crate) fn drop(s: &mut ffi::QVector_QSize) {
    ffi::qvector_drop_QSize(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QSize, pos: isize) -> &ffi::QSize {
    ffi::get_unchecked_QSize(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QSize, value: &ffi::QSize) -> isize {
    ffi::index_of_QSize(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QSize, pos: isize, value: &ffi::QSize) {
    ffi::insert_QSize(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QSize) -> isize {
    ffi::len_QSize(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QSize, pos: isize) {
    ffi::remove_QSize(s, pos);
}
