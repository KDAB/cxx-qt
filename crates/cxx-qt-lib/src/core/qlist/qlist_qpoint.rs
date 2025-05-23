// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;

        include!("cxx-qt-lib/core/qlist/qlist_QPoint.h");
        type QList_QPoint = crate::QList<QPoint>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QPoint);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QPoint, _: &QPoint) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QPoint"]
        fn construct(_: &QList_QPoint) -> QList_QPoint;
        #[rust_name = "qlist_default_QPoint"]
        fn construct() -> QList_QPoint;
        #[rust_name = "qlist_drop_QPoint"]
        fn drop(_: &mut QList_QPoint);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QPoint"]
        fn qlistReserve(_: &mut QList_QPoint, size: isize);
        #[rust_name = "append_QPoint"]
        fn qlistAppend(_: &mut QList_QPoint, _: &QPoint);
        #[rust_name = "get_unchecked_QPoint"]
        unsafe fn qlistGetUnchecked(set: &QList_QPoint, pos: isize) -> &QPoint;
        #[rust_name = "index_of_QPoint"]
        fn qlistIndexOf(_: &QList_QPoint, _: &QPoint) -> isize;
        #[rust_name = "insert_QPoint"]
        fn qlistInsert(_: &mut QList_QPoint, _: isize, _: &QPoint);
        #[rust_name = "remove_QPoint"]
        fn qlistRemove(_: &mut QList_QPoint, _: isize);
        #[rust_name = "len_QPoint"]
        fn qlistLen(_: &QList_QPoint) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QPoint, size: isize) {
    ffi::reserve_QPoint(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QPoint, value: &ffi::QPoint) {
    ffi::append_QPoint(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QPoint) -> ffi::QList_QPoint {
    ffi::qlist_clone_QPoint(s)
}

pub(crate) fn default() -> ffi::QList_QPoint {
    ffi::qlist_default_QPoint()
}

pub(crate) fn drop(s: &mut ffi::QList_QPoint) {
    ffi::qlist_drop_QPoint(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QPoint, pos: isize) -> &ffi::QPoint {
    ffi::get_unchecked_QPoint(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QPoint, value: &ffi::QPoint) -> isize {
    ffi::index_of_QPoint(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QPoint, pos: isize, value: &ffi::QPoint) {
    ffi::insert_QPoint(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QPoint) -> isize {
    ffi::len_QPoint(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QPoint, pos: isize) {
    ffi::remove_QPoint(s, pos);
}
