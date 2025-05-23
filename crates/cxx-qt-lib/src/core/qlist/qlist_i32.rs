// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qlist/qlist_i32.h");
        type QList_i32 = crate::QList<i32>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_i32);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_i32, _: &i32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_i32"]
        fn construct(_: &QList_i32) -> QList_i32;
        #[rust_name = "qlist_default_i32"]
        fn construct() -> QList_i32;
        #[rust_name = "qlist_drop_i32"]
        fn drop(_: &mut QList_i32);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_i32"]
        fn qlistReserve(_: &mut QList_i32, size: isize);
        #[rust_name = "append_i32"]
        fn qlistAppend(_: &mut QList_i32, _: &i32);
        #[rust_name = "get_unchecked_i32"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_i32, pos: isize) -> &'a i32;
        #[rust_name = "index_of_i32"]
        fn qlistIndexOf(_: &QList_i32, _: &i32) -> isize;
        #[rust_name = "insert_i32"]
        fn qlistInsert(_: &mut QList_i32, _: isize, _: &i32);
        #[rust_name = "len_i32"]
        fn qlistLen(_: &QList_i32) -> isize;
        #[rust_name = "remove_i32"]
        fn qlistRemove(_: &mut QList_i32, _: isize);
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_i32, size: isize) {
    ffi::reserve_i32(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_i32, value: &i32) {
    ffi::append_i32(v, value);
}

pub(crate) fn clone(v: &ffi::QList_i32) -> ffi::QList_i32 {
    ffi::qlist_clone_i32(v)
}

pub(crate) fn default() -> ffi::QList_i32 {
    ffi::qlist_default_i32()
}

pub(crate) fn drop(v: &mut ffi::QList_i32) {
    ffi::qlist_drop_i32(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_i32, pos: isize) -> &i32 {
    ffi::get_unchecked_i32(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_i32, value: &i32) -> isize {
    ffi::index_of_i32(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_i32, pos: isize, value: &i32) {
    ffi::insert_i32(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_i32) -> isize {
    ffi::len_i32(v)
}

pub(crate) fn remove(s: &mut ffi::QList_i32, pos: isize) {
    ffi::remove_i32(s, pos);
}
