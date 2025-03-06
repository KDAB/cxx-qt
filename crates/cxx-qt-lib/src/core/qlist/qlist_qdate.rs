// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdate.h");
        type QDate = crate::QDate;

        include!("cxx-qt-lib/qlist_QDate.h");
        type QList_QDate = crate::QList<QDate>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QDate);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QDate, _: &QDate) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QDate"]
        fn construct(_: &QList_QDate) -> QList_QDate;
        #[rust_name = "qlist_default_QDate"]
        fn construct() -> QList_QDate;
        #[rust_name = "qlist_drop_QDate"]
        fn drop(_: &mut QList_QDate);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QDate"]
        fn qlistReserve(_: &mut QList_QDate, size: isize);
        #[rust_name = "append_QDate"]
        fn qlistAppend(_: &mut QList_QDate, _: &QDate);
        #[rust_name = "get_unchecked_QDate"]
        unsafe fn qlistGetUnchecked(set: &QList_QDate, pos: isize) -> &QDate;
        #[rust_name = "index_of_QDate"]
        fn qlistIndexOf(_: &QList_QDate, _: &QDate) -> isize;
        #[rust_name = "insert_QDate"]
        fn qlistInsert(_: &mut QList_QDate, _: isize, _: &QDate);
        #[rust_name = "remove_QDate"]
        fn qlistRemove(_: &mut QList_QDate, _: isize);
        #[rust_name = "len_QDate"]
        fn qlistLen(_: &QList_QDate) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QDate, size: isize) {
    ffi::reserve_QDate(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QDate, value: &ffi::QDate) {
    ffi::append_QDate(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QDate) -> ffi::QList_QDate {
    ffi::qlist_clone_QDate(s)
}

pub(crate) fn default() -> ffi::QList_QDate {
    ffi::qlist_default_QDate()
}

pub(crate) fn drop(s: &mut ffi::QList_QDate) {
    ffi::qlist_drop_QDate(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QDate, pos: isize) -> &ffi::QDate {
    ffi::get_unchecked_QDate(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QDate, value: &ffi::QDate) -> isize {
    ffi::index_of_QDate(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QDate, pos: isize, value: &ffi::QDate) {
    ffi::insert_QDate(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QDate) -> isize {
    ffi::len_QDate(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QDate, pos: isize) {
    ffi::remove_QDate(s, pos);
}
