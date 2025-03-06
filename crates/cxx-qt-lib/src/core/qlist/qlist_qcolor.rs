// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;

        include!("cxx-qt-lib/qlist_QColor.h");
        type QList_QColor = crate::QList<QColor>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QColor);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QColor, _: &QColor) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QColor"]
        fn construct(_: &QList_QColor) -> QList_QColor;
        #[rust_name = "qlist_default_QColor"]
        fn construct() -> QList_QColor;
        #[rust_name = "qlist_drop_QColor"]
        fn drop(_: &mut QList_QColor);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QColor"]
        fn qlistReserve(_: &mut QList_QColor, size: isize);
        #[rust_name = "append_QColor"]
        fn qlistAppend(_: &mut QList_QColor, _: &QColor);
        #[rust_name = "get_unchecked_QColor"]
        unsafe fn qlistGetUnchecked(set: &QList_QColor, pos: isize) -> &QColor;
        #[rust_name = "index_of_QColor"]
        fn qlistIndexOf(_: &QList_QColor, _: &QColor) -> isize;
        #[rust_name = "insert_QColor"]
        fn qlistInsert(_: &mut QList_QColor, _: isize, _: &QColor);
        #[rust_name = "remove_QColor"]
        fn qlistRemove(_: &mut QList_QColor, _: isize);
        #[rust_name = "len_QColor"]
        fn qlistLen(_: &QList_QColor) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QColor, size: isize) {
    ffi::reserve_QColor(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QColor, value: &ffi::QColor) {
    ffi::append_QColor(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QColor) -> ffi::QList_QColor {
    ffi::qlist_clone_QColor(s)
}

pub(crate) fn default() -> ffi::QList_QColor {
    ffi::qlist_default_QColor()
}

pub(crate) fn drop(s: &mut ffi::QList_QColor) {
    ffi::qlist_drop_QColor(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QColor, pos: isize) -> &ffi::QColor {
    ffi::get_unchecked_QColor(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QColor, value: &ffi::QColor) -> isize {
    ffi::index_of_QColor(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QColor, pos: isize, value: &ffi::QColor) {
    ffi::insert_QColor(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QColor) -> isize {
    ffi::len_QColor(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QColor, pos: isize) {
    ffi::remove_QColor(s, pos);
}
