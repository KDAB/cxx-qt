// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qlist/qlist_f64.h");
        type QList_f64 = crate::QList<f64>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_f64);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_f64, _: &f64) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_f64"]
        fn construct(_: &QList_f64) -> QList_f64;
        #[rust_name = "qlist_default_f64"]
        fn construct() -> QList_f64;
        #[rust_name = "qlist_drop_f64"]
        fn drop(_: &mut QList_f64);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_f64"]
        fn qlistReserve(_: &mut QList_f64, size: isize);
        #[rust_name = "append_f64"]
        fn qlistAppend(_: &mut QList_f64, _: &f64);
        #[rust_name = "get_unchecked_f64"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_f64, pos: isize) -> &'a f64;
        #[rust_name = "index_of_f64"]
        fn qlistIndexOf(_: &QList_f64, _: &f64) -> isize;
        #[rust_name = "insert_f64"]
        fn qlistInsert(_: &mut QList_f64, _: isize, _: &f64);
        #[rust_name = "len_f64"]
        fn qlistLen(_: &QList_f64) -> isize;
        #[rust_name = "remove_f64"]
        fn qlistRemove(_: &mut QList_f64, _: isize);
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_f64, size: isize) {
    ffi::reserve_f64(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_f64, value: &f64) {
    ffi::append_f64(v, value);
}

pub(crate) fn clone(v: &ffi::QList_f64) -> ffi::QList_f64 {
    ffi::qlist_clone_f64(v)
}

pub(crate) fn default() -> ffi::QList_f64 {
    ffi::qlist_default_f64()
}

pub(crate) fn drop(v: &mut ffi::QList_f64) {
    ffi::qlist_drop_f64(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_f64, pos: isize) -> &f64 {
    ffi::get_unchecked_f64(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_f64, value: &f64) -> isize {
    ffi::index_of_f64(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_f64, pos: isize, value: &f64) {
    ffi::insert_f64(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_f64) -> isize {
    ffi::len_f64(v)
}

pub(crate) fn remove(s: &mut ffi::QList_f64, pos: isize) {
    ffi::remove_f64(s, pos);
}
