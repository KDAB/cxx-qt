// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;

        include!("cxx-qt-lib/qlist_QRect.h");
        type QList_QRect = crate::QList<QRect>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QRect);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QRect, _: &QRect) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QRect"]
        fn construct(_: &QList_QRect) -> QList_QRect;
        #[rust_name = "qlist_default_QRect"]
        fn construct() -> QList_QRect;
        #[rust_name = "qlist_drop_QRect"]
        fn drop(_: &mut QList_QRect);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QRect"]
        fn qlistReserve(_: &mut QList_QRect, size: isize);
        #[rust_name = "append_QRect"]
        fn qlistAppend(_: &mut QList_QRect, _: &QRect);
        #[rust_name = "get_unchecked_QRect"]
        unsafe fn qlistGetUnchecked(set: &QList_QRect, pos: isize) -> &QRect;
        #[rust_name = "index_of_QRect"]
        fn qlistIndexOf(_: &QList_QRect, _: &QRect) -> isize;
        #[rust_name = "insert_QRect"]
        fn qlistInsert(_: &mut QList_QRect, _: isize, _: &QRect);
        #[rust_name = "remove_QRect"]
        fn qlistRemove(_: &mut QList_QRect, _: isize);
        #[rust_name = "len_QRect"]
        fn qlistLen(_: &QList_QRect) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QRect, size: isize) {
    ffi::reserve_QRect(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QRect, value: &ffi::QRect) {
    ffi::append_QRect(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QRect) -> ffi::QList_QRect {
    ffi::qlist_clone_QRect(s)
}

pub(crate) fn default() -> ffi::QList_QRect {
    ffi::qlist_default_QRect()
}

pub(crate) fn drop(s: &mut ffi::QList_QRect) {
    ffi::qlist_drop_QRect(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QRect, pos: isize) -> &ffi::QRect {
    ffi::get_unchecked_QRect(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QRect, value: &ffi::QRect) -> isize {
    ffi::index_of_QRect(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QRect, pos: isize, value: &ffi::QRect) {
    ffi::insert_QRect(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QRect) -> isize {
    ffi::len_QRect(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QRect, pos: isize) {
    ffi::remove_QRect(s, pos);
}
