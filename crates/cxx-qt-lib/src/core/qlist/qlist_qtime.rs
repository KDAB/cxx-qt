// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qtime.h");
        type QTime = crate::QTime;

        include!("cxx-qt-lib/qlist_QTime.h");
        type QList_QTime = crate::QList<QTime>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QTime);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QTime, _: &QTime) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QTime"]
        fn construct(_: &QList_QTime) -> QList_QTime;
        #[rust_name = "qlist_default_QTime"]
        fn construct() -> QList_QTime;
        #[rust_name = "qlist_drop_QTime"]
        fn drop(_: &mut QList_QTime);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QTime"]
        fn qlistReserve(_: &mut QList_QTime, size: isize);
        #[rust_name = "append_QTime"]
        fn qlistAppend(_: &mut QList_QTime, _: &QTime);
        #[rust_name = "get_unchecked_QTime"]
        unsafe fn qlistGetUnchecked(set: &QList_QTime, pos: isize) -> &QTime;
        #[rust_name = "index_of_QTime"]
        fn qlistIndexOf(_: &QList_QTime, _: &QTime) -> isize;
        #[rust_name = "insert_QTime"]
        fn qlistInsert(_: &mut QList_QTime, _: isize, _: &QTime);
        #[rust_name = "remove_QTime"]
        fn qlistRemove(_: &mut QList_QTime, _: isize);
        #[rust_name = "len_QTime"]
        fn qlistLen(_: &QList_QTime) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QTime, size: isize) {
    ffi::reserve_QTime(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QTime, value: &ffi::QTime) {
    ffi::append_QTime(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QTime) -> ffi::QList_QTime {
    ffi::qlist_clone_QTime(s)
}

pub(crate) fn default() -> ffi::QList_QTime {
    ffi::qlist_default_QTime()
}

pub(crate) fn drop(s: &mut ffi::QList_QTime) {
    ffi::qlist_drop_QTime(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QTime, pos: isize) -> &ffi::QTime {
    ffi::get_unchecked_QTime(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QTime, value: &ffi::QTime) -> isize {
    ffi::index_of_QTime(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QTime, pos: isize, value: &ffi::QTime) {
    ffi::insert_QTime(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QTime) -> isize {
    ffi::len_QTime(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QTime, pos: isize) {
    ffi::remove_QTime(s, pos);
}
