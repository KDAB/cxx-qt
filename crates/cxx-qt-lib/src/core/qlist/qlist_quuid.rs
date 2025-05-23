// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/quuid.h");
        type QUuid = crate::QUuid;

        include!("cxx-qt-lib/core/qlist/qlist_QUuid.h");
        type QList_QUuid = crate::QList<QUuid>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QUuid);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QUuid, _: &QUuid) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QUuid"]
        fn construct(_: &QList_QUuid) -> QList_QUuid;
        #[rust_name = "qlist_default_QUuid"]
        fn construct() -> QList_QUuid;
        #[rust_name = "qlist_drop_QUuid"]
        fn drop(_: &mut QList_QUuid);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QUuid"]
        fn qlistReserve(_: &mut QList_QUuid, size: isize);
        #[rust_name = "append_QUuid"]
        fn qlistAppend(_: &mut QList_QUuid, _: &QUuid);
        #[rust_name = "get_unchecked_QUuid"]
        unsafe fn qlistGetUnchecked(set: &QList_QUuid, pos: isize) -> &QUuid;
        #[rust_name = "index_of_QUuid"]
        fn qlistIndexOf(_: &QList_QUuid, _: &QUuid) -> isize;
        #[rust_name = "insert_QUuid"]
        fn qlistInsert(_: &mut QList_QUuid, _: isize, _: &QUuid);
        #[rust_name = "remove_QUuid"]
        fn qlistRemove(_: &mut QList_QUuid, _: isize);
        #[rust_name = "len_QUuid"]
        fn qlistLen(_: &QList_QUuid) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QUuid, size: isize) {
    ffi::reserve_QUuid(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QUuid, value: &ffi::QUuid) {
    ffi::append_QUuid(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QUuid) -> ffi::QList_QUuid {
    ffi::qlist_clone_QUuid(s)
}

pub(crate) fn default() -> ffi::QList_QUuid {
    ffi::qlist_default_QUuid()
}

pub(crate) fn drop(s: &mut ffi::QList_QUuid) {
    ffi::qlist_drop_QUuid(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QUuid, pos: isize) -> &ffi::QUuid {
    ffi::get_unchecked_QUuid(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QUuid, value: &ffi::QUuid) -> isize {
    ffi::index_of_QUuid(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QUuid, pos: isize, value: &ffi::QUuid) {
    ffi::insert_QUuid(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QUuid) -> isize {
    ffi::len_QUuid(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QUuid, pos: isize) {
    ffi::remove_QUuid(s, pos);
}
