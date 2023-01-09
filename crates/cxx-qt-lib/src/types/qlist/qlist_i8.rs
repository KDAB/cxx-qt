// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlist.h");
        type QList_i8 = crate::QList<i8>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_i8);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_i8, _: &i8) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_i8"]
        fn construct(_: &QList_i8) -> QList_i8;
        #[rust_name = "qlist_default_i8"]
        fn construct() -> QList_i8;
        #[rust_name = "qlist_drop_i8"]
        fn drop(_: &mut QList_i8);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "append_i8"]
        fn qlistAppend(_: &mut QList_i8, _: &i8);
        #[rust_name = "get_unchecked_i8"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_i8, pos: isize) -> &'a i8;
        #[rust_name = "index_of_i8"]
        fn qlistIndexOf(_: &QList_i8, _: &i8) -> isize;
        #[rust_name = "insert_i8"]
        fn qlistInsert(_: &mut QList_i8, _: isize, _: &i8);
        #[rust_name = "len_i8"]
        fn qlistLen(_: &QList_i8) -> isize;
        #[rust_name = "remove_i8"]
        fn qlistRemove(_: &mut QList_i8, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QList_i8, value: &i8) {
    ffi::append_i8(v, value);
}

pub(crate) fn clone(v: &ffi::QList_i8) -> ffi::QList_i8 {
    ffi::qlist_clone_i8(v)
}

pub(crate) fn default() -> ffi::QList_i8 {
    ffi::qlist_default_i8()
}

pub(crate) fn drop(v: &mut ffi::QList_i8) {
    ffi::qlist_drop_i8(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_i8, pos: isize) -> &i8 {
    ffi::get_unchecked_i8(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_i8, value: &i8) -> isize {
    ffi::index_of_i8(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_i8, pos: isize, value: &i8) {
    ffi::insert_i8(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_i8) -> isize {
    ffi::len_i8(v)
}

pub(crate) fn remove(s: &mut ffi::QList_i8, pos: isize) {
    ffi::remove_i8(s, pos);
}
