// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlist.h");
        type QList_i16 = crate::QList<i16>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_i16);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_i16, _: &i16) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_i16"]
        fn construct(_: &QList_i16) -> QList_i16;
        #[rust_name = "qlist_default_i16"]
        fn construct() -> QList_i16;
        #[rust_name = "qlist_drop_i16"]
        fn drop(_: &mut QList_i16);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "append_i16"]
        fn qlistAppend(_: &mut QList_i16, _: &i16);
        #[rust_name = "get_unchecked_i16"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_i16, pos: isize) -> &'a i16;
        #[rust_name = "index_of_i16"]
        fn qlistIndexOf(_: &QList_i16, _: &i16) -> isize;
        #[rust_name = "insert_i16"]
        fn qlistInsert(_: &mut QList_i16, _: isize, _: &i16);
        #[rust_name = "len_i16"]
        fn qlistLen(_: &QList_i16) -> isize;
        #[rust_name = "remove_i16"]
        fn qlistRemove(_: &mut QList_i16, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QList_i16, value: &i16) {
    ffi::append_i16(v, value);
}

pub(crate) fn clone(v: &ffi::QList_i16) -> ffi::QList_i16 {
    ffi::qlist_clone_i16(v)
}

pub(crate) fn default() -> ffi::QList_i16 {
    ffi::qlist_default_i16()
}

pub(crate) fn drop(v: &mut ffi::QList_i16) {
    ffi::qlist_drop_i16(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_i16, pos: isize) -> &i16 {
    ffi::get_unchecked_i16(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_i16, value: &i16) -> isize {
    ffi::index_of_i16(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_i16, pos: isize, value: &i16) {
    ffi::insert_i16(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_i16) -> isize {
    ffi::len_i16(v)
}

pub(crate) fn remove(s: &mut ffi::QList_i16, pos: isize) {
    ffi::remove_i16(s, pos);
}
