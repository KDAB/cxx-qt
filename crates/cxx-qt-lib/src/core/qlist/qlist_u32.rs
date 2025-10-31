// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qlist/qlist_u32.h");
        type QList_u32 = crate::QList<u32>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_u32);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_u32, _: &u32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_u32"]
        fn construct(_: &QList_u32) -> QList_u32;
        #[rust_name = "qlist_default_u32"]
        fn construct() -> QList_u32;
        #[rust_name = "qlist_drop_u32"]
        fn drop(_: &mut QList_u32);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_u32"]
        fn qlistReserve(_: &mut QList_u32, size: isize);
        #[rust_name = "append_u32"]
        fn qlistAppend(_: &mut QList_u32, _: &u32);
        #[rust_name = "get_unchecked_u32"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_u32, pos: isize) -> &'a u32;
        #[rust_name = "index_of_u32"]
        fn qlistIndexOf(_: &QList_u32, _: &u32) -> isize;
        #[rust_name = "insert_u32"]
        fn qlistInsert(_: &mut QList_u32, _: isize, _: &u32);
        #[rust_name = "len_u32"]
        fn qlistLen(_: &QList_u32) -> isize;
        #[rust_name = "remove_u32"]
        fn qlistRemove(_: &mut QList_u32, _: isize);
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_u32, size: isize) {
    ffi::reserve_u32(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_u32, value: &u32) {
    ffi::append_u32(v, value);
}

pub(crate) fn clone(v: &ffi::QList_u32) -> ffi::QList_u32 {
    ffi::qlist_clone_u32(v)
}

pub(crate) fn default() -> ffi::QList_u32 {
    ffi::qlist_default_u32()
}

pub(crate) fn drop(v: &mut ffi::QList_u32) {
    ffi::qlist_drop_u32(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_u32, pos: isize) -> &u32 {
    ffi::get_unchecked_u32(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_u32, value: &u32) -> isize {
    ffi::index_of_u32(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_u32, pos: isize, value: &u32) {
    ffi::insert_u32(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_u32) -> isize {
    ffi::len_u32(v)
}

pub(crate) fn remove(s: &mut ffi::QList_u32, pos: isize) {
    ffi::remove_u32(s, pos);
}
