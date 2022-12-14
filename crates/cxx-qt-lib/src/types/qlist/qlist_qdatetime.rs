// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = crate::QDateTime;

        include!("cxx-qt-lib/qlist.h");
        type QList_QDateTime = crate::QList<QDateTime>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QDateTime);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QDateTime, _: &QDateTime) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QDateTime"]
        fn construct(_: &QList_QDateTime) -> QList_QDateTime;
        #[rust_name = "qlist_default_QDateTime"]
        fn construct() -> QList_QDateTime;
        #[rust_name = "qlist_drop_QDateTime"]
        fn drop(_: &mut QList_QDateTime);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "append_QDateTime"]
        fn qlistAppend(_: &mut QList_QDateTime, _: &QDateTime);
        #[rust_name = "get_unchecked_QDateTime"]
        unsafe fn qlistGetUnchecked(set: &QList_QDateTime, pos: isize) -> &QDateTime;
        #[rust_name = "index_of_QDateTime"]
        fn qlistIndexOf(_: &QList_QDateTime, _: &QDateTime) -> isize;
        #[rust_name = "insert_QDateTime"]
        fn qlistInsert(_: &mut QList_QDateTime, _: isize, _: &QDateTime);
        #[rust_name = "remove_QDateTime"]
        fn qlistRemove(_: &mut QList_QDateTime, _: isize);
        #[rust_name = "len_QDateTime"]
        fn qlistLen(_: &QList_QDateTime) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QList_QDateTime, value: &ffi::QDateTime) {
    ffi::append_QDateTime(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QDateTime) -> ffi::QList_QDateTime {
    ffi::qlist_clone_QDateTime(s)
}

pub(crate) fn default() -> ffi::QList_QDateTime {
    ffi::qlist_default_QDateTime()
}

pub(crate) fn drop(s: &mut ffi::QList_QDateTime) {
    ffi::qlist_drop_QDateTime(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QDateTime, pos: isize) -> &ffi::QDateTime {
    ffi::get_unchecked_QDateTime(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QDateTime, value: &ffi::QDateTime) -> isize {
    ffi::index_of_QDateTime(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QDateTime, pos: isize, value: &ffi::QDateTime) {
    ffi::insert_QDateTime(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QDateTime) -> isize {
    ffi::len_QDateTime(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QDateTime, pos: isize) {
    ffi::remove_QDateTime(s, pos);
}
