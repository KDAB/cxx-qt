// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlist_u16.h");
        type QList_u16 = crate::QList<u16>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_u16);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_u16, _: &u16) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_u16"]
        fn construct(_: &QList_u16) -> QList_u16;
        #[rust_name = "qlist_default_u16"]
        fn construct() -> QList_u16;
        #[rust_name = "qlist_drop_u16"]
        fn drop(_: &mut QList_u16);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_u16"]
        fn qlistReserve(_: &mut QList_u16, size: isize);
        #[rust_name = "append_u16"]
        fn qlistAppend(_: &mut QList_u16, _: &u16);
        #[rust_name = "get_unchecked_u16"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_u16, pos: isize) -> &'a u16;
        #[rust_name = "index_of_u16"]
        fn qlistIndexOf(_: &QList_u16, _: &u16) -> isize;
        #[rust_name = "insert_u16"]
        fn qlistInsert(_: &mut QList_u16, _: isize, _: &u16);
        #[rust_name = "len_u16"]
        fn qlistLen(_: &QList_u16) -> isize;
        #[rust_name = "remove_u16"]
        fn qlistRemove(_: &mut QList_u16, _: isize);
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_u16, size: isize) {
    ffi::reserve_u16(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_u16, value: &u16) {
    ffi::append_u16(v, value);
}

pub(crate) fn clone(v: &ffi::QList_u16) -> ffi::QList_u16 {
    ffi::qlist_clone_u16(v)
}

pub(crate) fn default() -> ffi::QList_u16 {
    ffi::qlist_default_u16()
}

pub(crate) fn drop(v: &mut ffi::QList_u16) {
    ffi::qlist_drop_u16(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_u16, pos: isize) -> &u16 {
    ffi::get_unchecked_u16(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_u16, value: &u16) -> isize {
    ffi::index_of_u16(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_u16, pos: isize, value: &u16) {
    ffi::insert_u16(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_u16) -> isize {
    ffi::len_u16(v)
}

pub(crate) fn remove(s: &mut ffi::QList_u16, pos: isize) {
    ffi::remove_u16(s, pos);
}
