// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qlist/qlist_u64.h");
        type QList_u64 = crate::QList<u64>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_u64);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_u64, _: &u64) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_u64"]
        fn construct(_: &QList_u64) -> QList_u64;
        #[rust_name = "qlist_default_u64"]
        fn construct() -> QList_u64;
        #[rust_name = "qlist_drop_u64"]
        fn drop(_: &mut QList_u64);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_u64"]
        fn qlistReserve(_: &mut QList_u64, size: isize);
        #[rust_name = "append_u64"]
        fn qlistAppend(_: &mut QList_u64, _: &u64);
        #[rust_name = "get_unchecked_u64"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_u64, pos: isize) -> &'a u64;
        #[rust_name = "index_of_u64"]
        fn qlistIndexOf(_: &QList_u64, _: &u64) -> isize;
        #[rust_name = "insert_u64"]
        fn qlistInsert(_: &mut QList_u64, _: isize, _: &u64);
        #[rust_name = "len_u64"]
        fn qlistLen(_: &QList_u64) -> isize;
        #[rust_name = "remove_u64"]
        fn qlistRemove(_: &mut QList_u64, _: isize);
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_u64, size: isize) {
    ffi::reserve_u64(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_u64, value: &u64) {
    ffi::append_u64(v, value);
}

pub(crate) fn clone(v: &ffi::QList_u64) -> ffi::QList_u64 {
    ffi::qlist_clone_u64(v)
}

pub(crate) fn default() -> ffi::QList_u64 {
    ffi::qlist_default_u64()
}

pub(crate) fn drop(v: &mut ffi::QList_u64) {
    ffi::qlist_drop_u64(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_u64, pos: isize) -> &u64 {
    ffi::get_unchecked_u64(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_u64, value: &u64) -> isize {
    ffi::index_of_u64(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_u64, pos: isize, value: &u64) {
    ffi::insert_u64(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_u64) -> isize {
    ffi::len_u64(v)
}

pub(crate) fn remove(s: &mut ffi::QList_u64, pos: isize) {
    ffi::remove_u64(s, pos);
}
