// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;

        include!("cxx-qt-lib/core/qlist/qlist_QVariant.h");
        type QList_QVariant = crate::QList<QVariant>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QVariant);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QVariant, _: &QVariant) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QVariant"]
        fn construct(_: &QList_QVariant) -> QList_QVariant;
        #[rust_name = "qlist_default_QVariant"]
        fn construct() -> QList_QVariant;
        #[rust_name = "qlist_drop_QVariant"]
        fn drop(_: &mut QList_QVariant);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QVariant"]
        fn qlistReserve(_: &mut QList_QVariant, size: isize);
        #[rust_name = "append_QVariant"]
        fn qlistAppend(_: &mut QList_QVariant, _: &QVariant);
        #[rust_name = "get_unchecked_QVariant"]
        unsafe fn qlistGetUnchecked(set: &QList_QVariant, pos: isize) -> &QVariant;
        #[rust_name = "index_of_QVariant"]
        fn qlistIndexOf(_: &QList_QVariant, _: &QVariant) -> isize;
        #[rust_name = "insert_QVariant"]
        fn qlistInsert(_: &mut QList_QVariant, _: isize, _: &QVariant);
        #[rust_name = "remove_QVariant"]
        fn qlistRemove(_: &mut QList_QVariant, _: isize);
        #[rust_name = "len_QVariant"]
        fn qlistLen(_: &QList_QVariant) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QVariant, size: isize) {
    ffi::reserve_QVariant(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QVariant, value: &ffi::QVariant) {
    ffi::append_QVariant(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QVariant) -> ffi::QList_QVariant {
    ffi::qlist_clone_QVariant(s)
}

pub(crate) fn default() -> ffi::QList_QVariant {
    ffi::qlist_default_QVariant()
}

pub(crate) fn drop(s: &mut ffi::QList_QVariant) {
    ffi::qlist_drop_QVariant(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QVariant, pos: isize) -> &ffi::QVariant {
    ffi::get_unchecked_QVariant(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QVariant, value: &ffi::QVariant) -> isize {
    ffi::index_of_QVariant(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QVariant, pos: isize, value: &ffi::QVariant) {
    ffi::insert_QVariant(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QVariant) -> isize {
    ffi::len_QVariant(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QVariant, pos: isize) {
    ffi::remove_QVariant(s, pos);
}
