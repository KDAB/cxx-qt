// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qsize.h");
        type QSize = crate::QSize;

        include!("cxx-qt-lib/qlist_QSize.h");
        type QList_QSize = crate::QList<QSize>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QSize);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QSize, _: &QSize) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QSize"]
        fn construct(_: &QList_QSize) -> QList_QSize;
        #[rust_name = "qlist_default_QSize"]
        fn construct() -> QList_QSize;
        #[rust_name = "qlist_drop_QSize"]
        fn drop(_: &mut QList_QSize);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QSize"]
        fn qlistReserve(_: &mut QList_QSize, size: isize);
        #[rust_name = "append_QSize"]
        fn qlistAppend(_: &mut QList_QSize, _: &QSize);
        #[rust_name = "get_unchecked_QSize"]
        unsafe fn qlistGetUnchecked(set: &QList_QSize, pos: isize) -> &QSize;
        #[rust_name = "index_of_QSize"]
        fn qlistIndexOf(_: &QList_QSize, _: &QSize) -> isize;
        #[rust_name = "insert_QSize"]
        fn qlistInsert(_: &mut QList_QSize, _: isize, _: &QSize);
        #[rust_name = "remove_QSize"]
        fn qlistRemove(_: &mut QList_QSize, _: isize);
        #[rust_name = "len_QSize"]
        fn qlistLen(_: &QList_QSize) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QSize, size: isize) {
    ffi::reserve_QSize(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QSize, value: &ffi::QSize) {
    ffi::append_QSize(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QSize) -> ffi::QList_QSize {
    ffi::qlist_clone_QSize(s)
}

pub(crate) fn default() -> ffi::QList_QSize {
    ffi::qlist_default_QSize()
}

pub(crate) fn drop(s: &mut ffi::QList_QSize) {
    ffi::qlist_drop_QSize(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QSize, pos: isize) -> &ffi::QSize {
    ffi::get_unchecked_QSize(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QSize, value: &ffi::QSize) -> isize {
    ffi::index_of_QSize(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QSize, pos: isize, value: &ffi::QSize) {
    ffi::insert_QSize(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSize) -> isize {
    ffi::len_QSize(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSize, pos: isize) {
    ffi::remove_QSize(s, pos);
}
