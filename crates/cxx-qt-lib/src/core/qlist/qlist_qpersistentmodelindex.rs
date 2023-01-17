// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpersistentmodelindex.h");
        type QPersistentModelIndex = crate::QPersistentModelIndex;

        include!("cxx-qt-lib/qlist.h");
        type QList_QPersistentModelIndex = crate::QList<QPersistentModelIndex>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QPersistentModelIndex);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QPersistentModelIndex, _: &QPersistentModelIndex) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QPersistentModelIndex"]
        fn construct(_: &QList_QPersistentModelIndex) -> QList_QPersistentModelIndex;
        #[rust_name = "qlist_default_QPersistentModelIndex"]
        fn construct() -> QList_QPersistentModelIndex;
        #[rust_name = "qlist_drop_QPersistentModelIndex"]
        fn drop(_: &mut QList_QPersistentModelIndex);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QPersistentModelIndex"]
        fn qlistReserve(_: &mut QList_QPersistentModelIndex, size: isize);
        #[rust_name = "append_QPersistentModelIndex"]
        fn qlistAppend(_: &mut QList_QPersistentModelIndex, _: &QPersistentModelIndex);
        #[rust_name = "get_unchecked_QPersistentModelIndex"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QPersistentModelIndex,
            pos: isize,
        ) -> &QPersistentModelIndex;
        #[rust_name = "index_of_QPersistentModelIndex"]
        fn qlistIndexOf(_: &QList_QPersistentModelIndex, _: &QPersistentModelIndex) -> isize;
        #[rust_name = "insert_QPersistentModelIndex"]
        fn qlistInsert(_: &mut QList_QPersistentModelIndex, _: isize, _: &QPersistentModelIndex);
        #[rust_name = "remove_QPersistentModelIndex"]
        fn qlistRemove(_: &mut QList_QPersistentModelIndex, _: isize);
        #[rust_name = "len_QPersistentModelIndex"]
        fn qlistLen(_: &QList_QPersistentModelIndex) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QPersistentModelIndex, size: isize) {
    ffi::reserve_QPersistentModelIndex(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QPersistentModelIndex, value: &ffi::QPersistentModelIndex) {
    ffi::append_QPersistentModelIndex(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QPersistentModelIndex) -> ffi::QList_QPersistentModelIndex {
    ffi::qlist_clone_QPersistentModelIndex(s)
}

pub(crate) fn default() -> ffi::QList_QPersistentModelIndex {
    ffi::qlist_default_QPersistentModelIndex()
}

pub(crate) fn drop(s: &mut ffi::QList_QPersistentModelIndex) {
    ffi::qlist_drop_QPersistentModelIndex(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QPersistentModelIndex,
    pos: isize,
) -> &ffi::QPersistentModelIndex {
    ffi::get_unchecked_QPersistentModelIndex(s, pos)
}

pub(crate) fn index_of(
    v: &ffi::QList_QPersistentModelIndex,
    value: &ffi::QPersistentModelIndex,
) -> isize {
    ffi::index_of_QPersistentModelIndex(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QList_QPersistentModelIndex,
    pos: isize,
    value: &ffi::QPersistentModelIndex,
) {
    ffi::insert_QPersistentModelIndex(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QPersistentModelIndex) -> isize {
    ffi::len_QPersistentModelIndex(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QPersistentModelIndex, pos: isize) {
    ffi::remove_QPersistentModelIndex(s, pos);
}
