// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;

        include!("cxx-qt-lib/core/qlist/qlist_QByteArray.h");
        type QList_QByteArray = crate::QList<QByteArray>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QByteArray);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QByteArray, _: &QByteArray) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QByteArray"]
        fn construct(_: &QList_QByteArray) -> QList_QByteArray;
        #[rust_name = "qlist_default_QByteArray"]
        fn construct() -> QList_QByteArray;
        #[rust_name = "qlist_drop_QByteArray"]
        fn drop(_: &mut QList_QByteArray);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QByteArray"]
        fn qlistReserve(_: &mut QList_QByteArray, size: isize);
        #[rust_name = "append_QByteArray"]
        fn qlistAppend(_: &mut QList_QByteArray, _: &QByteArray);
        #[rust_name = "get_unchecked_QByteArray"]
        unsafe fn qlistGetUnchecked(set: &QList_QByteArray, pos: isize) -> &QByteArray;
        #[rust_name = "index_of_QByteArray"]
        fn qlistIndexOf(_: &QList_QByteArray, _: &QByteArray) -> isize;
        #[rust_name = "insert_QByteArray"]
        fn qlistInsert(_: &mut QList_QByteArray, _: isize, _: &QByteArray);
        #[rust_name = "remove_QByteArray"]
        fn qlistRemove(_: &mut QList_QByteArray, _: isize);
        #[rust_name = "len_QByteArray"]
        fn qlistLen(_: &QList_QByteArray) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QByteArray, size: isize) {
    ffi::reserve_QByteArray(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QByteArray, value: &ffi::QByteArray) {
    ffi::append_QByteArray(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QByteArray) -> ffi::QList_QByteArray {
    ffi::qlist_clone_QByteArray(s)
}

pub(crate) fn default() -> ffi::QList_QByteArray {
    ffi::qlist_default_QByteArray()
}

pub(crate) fn drop(s: &mut ffi::QList_QByteArray) {
    ffi::qlist_drop_QByteArray(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QByteArray, pos: isize) -> &ffi::QByteArray {
    ffi::get_unchecked_QByteArray(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QByteArray, value: &ffi::QByteArray) -> isize {
    ffi::index_of_QByteArray(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QByteArray, pos: isize, value: &ffi::QByteArray) {
    ffi::insert_QByteArray(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QByteArray) -> isize {
    ffi::len_QByteArray(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QByteArray, pos: isize) {
    ffi::remove_QByteArray(s, pos);
}
