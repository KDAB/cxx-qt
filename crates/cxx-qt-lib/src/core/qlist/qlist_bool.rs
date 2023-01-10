// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlist.h");
        type QList_bool = crate::QList<bool>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_bool);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_bool, _: &bool) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_bool"]
        fn construct(_: &QList_bool) -> QList_bool;
        #[rust_name = "qlist_default_bool"]
        fn construct() -> QList_bool;
        #[rust_name = "qlist_drop_bool"]
        fn drop(_: &mut QList_bool);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_bool"]
        fn qlistReserve(_: &mut QList_bool, size: isize);
        #[rust_name = "append_bool"]
        fn qlistAppend(_: &mut QList_bool, _: &bool);
        #[rust_name = "get_unchecked_bool"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_bool, pos: isize) -> &'a bool;
        #[rust_name = "index_of_bool"]
        fn qlistIndexOf(_: &QList_bool, _: &bool) -> isize;
        #[rust_name = "insert_bool"]
        fn qlistInsert(_: &mut QList_bool, _: isize, _: &bool);
        #[rust_name = "len_bool"]
        fn qlistLen(_: &QList_bool) -> isize;
        #[rust_name = "remove_bool"]
        fn qlistRemove(_: &mut QList_bool, _: isize);
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_bool, size: isize) {
    ffi::reserve_bool(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_bool, value: &bool) {
    ffi::append_bool(v, value);
}

pub(crate) fn clone(v: &ffi::QList_bool) -> ffi::QList_bool {
    ffi::qlist_clone_bool(v)
}

pub(crate) fn default() -> ffi::QList_bool {
    ffi::qlist_default_bool()
}

pub(crate) fn drop(v: &mut ffi::QList_bool) {
    ffi::qlist_drop_bool(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_bool, pos: isize) -> &bool {
    ffi::get_unchecked_bool(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_bool, value: &bool) -> isize {
    ffi::index_of_bool(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_bool, pos: isize, value: &bool) {
    ffi::insert_bool(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_bool) -> isize {
    ffi::len_bool(v)
}

pub(crate) fn remove(s: &mut ffi::QList_bool, pos: isize) {
    ffi::remove_bool(s, pos);
}
