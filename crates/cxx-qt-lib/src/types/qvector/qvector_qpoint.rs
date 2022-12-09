// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;

        include!("cxx-qt-lib/qvector.h");
        type QVector_QPoint = crate::QVector<QPoint>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QPoint);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QPoint, _: &QPoint) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QPoint"]
        fn construct(_: &QVector_QPoint) -> QVector_QPoint;
        #[rust_name = "qvector_default_QPoint"]
        fn construct() -> QVector_QPoint;
        #[rust_name = "qvector_drop_QPoint"]
        fn drop(_: &mut QVector_QPoint);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "append_QPoint"]
        fn qvectorAppend(_: &mut QVector_QPoint, _: &QPoint);
        #[rust_name = "get_unchecked_QPoint"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QPoint, pos: isize) -> &QPoint;
        #[rust_name = "index_of_QPoint"]
        fn qvectorIndexOf(_: &QVector_QPoint, _: &QPoint) -> isize;
        #[rust_name = "insert_QPoint"]
        fn qvectorInsert(_: &mut QVector_QPoint, _: isize, _: &QPoint);
        #[rust_name = "remove_QPoint"]
        fn qvectorRemove(_: &mut QVector_QPoint, _: isize);
        #[rust_name = "len_QPoint"]
        fn qvectorLen(_: &QVector_QPoint) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QPoint, value: &ffi::QPoint) {
    ffi::append_QPoint(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QPoint) -> ffi::QVector_QPoint {
    ffi::qvector_clone_QPoint(s)
}

pub(crate) fn default() -> ffi::QVector_QPoint {
    ffi::qvector_default_QPoint()
}

pub(crate) fn drop(s: &mut ffi::QVector_QPoint) {
    ffi::qvector_drop_QPoint(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QPoint, pos: isize) -> &ffi::QPoint {
    ffi::get_unchecked_QPoint(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QPoint, value: &ffi::QPoint) -> isize {
    ffi::index_of_QPoint(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QPoint, pos: isize, value: &ffi::QPoint) {
    ffi::insert_QPoint(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QPoint) -> isize {
    ffi::len_QPoint(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QPoint, pos: isize) {
    ffi::remove_QPoint(s, pos);
}
