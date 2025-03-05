// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlist_u8.h");
        type QList_u8 = crate::QList<u8>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_u8);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_u8, _: &u8) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_u8"]
        fn construct(_: &QList_u8) -> QList_u8;
        #[rust_name = "qlist_default_u8"]
        fn construct() -> QList_u8;
        #[rust_name = "qlist_drop_u8"]
        fn drop(_: &mut QList_u8);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_u8"]
        fn qlistReserve(_: &mut QList_u8, size: isize);
        #[rust_name = "append_u8"]
        fn qlistAppend(_: &mut QList_u8, _: &u8);
        #[rust_name = "get_unchecked_u8"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_u8, pos: isize) -> &'a u8;
        #[rust_name = "index_of_u8"]
        fn qlistIndexOf(_: &QList_u8, _: &u8) -> isize;
        #[rust_name = "insert_u8"]
        fn qlistInsert(_: &mut QList_u8, _: isize, _: &u8);
        #[rust_name = "len_u8"]
        fn qlistLen(_: &QList_u8) -> isize;
        #[rust_name = "remove_u8"]
        fn qlistRemove(_: &mut QList_u8, _: isize);
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_u8, size: isize) {
    ffi::reserve_u8(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_u8, value: &u8) {
    ffi::append_u8(v, value);
}

pub(crate) fn clone(v: &ffi::QList_u8) -> ffi::QList_u8 {
    ffi::qlist_clone_u8(v)
}

pub(crate) fn default() -> ffi::QList_u8 {
    ffi::qlist_default_u8()
}

pub(crate) fn drop(v: &mut ffi::QList_u8) {
    ffi::qlist_drop_u8(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_u8, pos: isize) -> &u8 {
    ffi::get_unchecked_u8(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_u8, value: &u8) -> isize {
    ffi::index_of_u8(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_u8, pos: isize, value: &u8) {
    ffi::insert_u8(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_u8) -> isize {
    ffi::len_u8(v)
}

pub(crate) fn remove(s: &mut ffi::QList_u8, pos: isize) {
    ffi::remove_u8(s, pos);
}
