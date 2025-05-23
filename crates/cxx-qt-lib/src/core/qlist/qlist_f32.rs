// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qlist/qlist_f32.h");
        type QList_f32 = crate::QList<f32>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_f32);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_f32, _: &f32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_f32"]
        fn construct(_: &QList_f32) -> QList_f32;
        #[rust_name = "qlist_default_f32"]
        fn construct() -> QList_f32;
        #[rust_name = "qlist_drop_f32"]
        fn drop(_: &mut QList_f32);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_f32"]
        fn qlistReserve(_: &mut QList_f32, size: isize);
        #[rust_name = "append_f32"]
        fn qlistAppend(_: &mut QList_f32, _: &f32);
        #[rust_name = "get_unchecked_f32"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_f32, pos: isize) -> &'a f32;
        #[rust_name = "index_of_f32"]
        fn qlistIndexOf(_: &QList_f32, _: &f32) -> isize;
        #[rust_name = "insert_f32"]
        fn qlistInsert(_: &mut QList_f32, _: isize, _: &f32);
        #[rust_name = "len_f32"]
        fn qlistLen(_: &QList_f32) -> isize;
        #[rust_name = "remove_f32"]
        fn qlistRemove(_: &mut QList_f32, _: isize);
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_f32, size: isize) {
    ffi::reserve_f32(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_f32, value: &f32) {
    ffi::append_f32(v, value);
}

pub(crate) fn clone(v: &ffi::QList_f32) -> ffi::QList_f32 {
    ffi::qlist_clone_f32(v)
}

pub(crate) fn default() -> ffi::QList_f32 {
    ffi::qlist_default_f32()
}

pub(crate) fn drop(v: &mut ffi::QList_f32) {
    ffi::qlist_drop_f32(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_f32, pos: isize) -> &f32 {
    ffi::get_unchecked_f32(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_f32, value: &f32) -> isize {
    ffi::index_of_f32(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_f32, pos: isize, value: &f32) {
    ffi::insert_f32(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_f32) -> isize {
    ffi::len_f32(v)
}

pub(crate) fn remove(s: &mut ffi::QList_f32, pos: isize) {
    ffi::remove_f32(s, pos);
}
