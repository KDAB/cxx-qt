// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qlist/qlist_i64.h");
        type QList_i64 = crate::QList<i64>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_i64);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_i64, _: &i64) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_i64"]
        fn construct(_: &QList_i64) -> QList_i64;
        #[rust_name = "qlist_default_i64"]
        fn construct() -> QList_i64;
        #[rust_name = "qlist_drop_i64"]
        fn drop(_: &mut QList_i64);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_i64"]
        fn qlistReserve(_: &mut QList_i64, size: isize);
        #[rust_name = "append_i64"]
        fn qlistAppend(_: &mut QList_i64, _: &i64);
        #[rust_name = "get_unchecked_i64"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_i64, pos: isize) -> &'a i64;
        #[rust_name = "index_of_i64"]
        fn qlistIndexOf(_: &QList_i64, _: &i64) -> isize;
        #[rust_name = "insert_i64"]
        fn qlistInsert(_: &mut QList_i64, _: isize, _: &i64);
        #[rust_name = "len_i64"]
        fn qlistLen(_: &QList_i64) -> isize;
        #[rust_name = "remove_i64"]
        fn qlistRemove(_: &mut QList_i64, _: isize);
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_i64, size: isize) {
    ffi::reserve_i64(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_i64, value: &i64) {
    ffi::append_i64(v, value);
}

pub(crate) fn clone(v: &ffi::QList_i64) -> ffi::QList_i64 {
    ffi::qlist_clone_i64(v)
}

pub(crate) fn default() -> ffi::QList_i64 {
    ffi::qlist_default_i64()
}

pub(crate) fn drop(v: &mut ffi::QList_i64) {
    ffi::qlist_drop_i64(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_i64, pos: isize) -> &i64 {
    ffi::get_unchecked_i64(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_i64, value: &i64) -> isize {
    ffi::index_of_i64(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_i64, pos: isize, value: &i64) {
    ffi::insert_i64(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_i64) -> isize {
    ffi::len_i64(v)
}

pub(crate) fn remove(s: &mut ffi::QList_i64, pos: isize) {
    ffi::remove_i64(s, pos);
}
