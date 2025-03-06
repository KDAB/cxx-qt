// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlinef.h");
        type QLineF = crate::QLineF;

        include!("cxx-qt-lib/qlist_QLineF.h");
        type QList_QLineF = crate::QList<QLineF>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QLineF);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QLineF, _: &QLineF) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QLineF"]
        fn construct(_: &QList_QLineF) -> QList_QLineF;
        #[rust_name = "qlist_default_QLineF"]
        fn construct() -> QList_QLineF;
        #[rust_name = "qlist_drop_QLineF"]
        fn drop(_: &mut QList_QLineF);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QLineF"]
        fn qlistReserve(_: &mut QList_QLineF, size: isize);
        #[rust_name = "append_QLineF"]
        fn qlistAppend(_: &mut QList_QLineF, _: &QLineF);
        #[rust_name = "get_unchecked_QLineF"]
        unsafe fn qlistGetUnchecked(set: &QList_QLineF, pos: isize) -> &QLineF;
        #[rust_name = "index_of_QLineF"]
        fn qlistIndexOf(_: &QList_QLineF, _: &QLineF) -> isize;
        #[rust_name = "insert_QLineF"]
        fn qlistInsert(_: &mut QList_QLineF, _: isize, _: &QLineF);
        #[rust_name = "remove_QLineF"]
        fn qlistRemove(_: &mut QList_QLineF, _: isize);
        #[rust_name = "len_QLineF"]
        fn qlistLen(_: &QList_QLineF) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QLineF, size: isize) {
    ffi::reserve_QLineF(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QLineF, value: &ffi::QLineF) {
    ffi::append_QLineF(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QLineF) -> ffi::QList_QLineF {
    ffi::qlist_clone_QLineF(s)
}

pub(crate) fn default() -> ffi::QList_QLineF {
    ffi::qlist_default_QLineF()
}

pub(crate) fn drop(s: &mut ffi::QList_QLineF) {
    ffi::qlist_drop_QLineF(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QLineF, pos: isize) -> &ffi::QLineF {
    ffi::get_unchecked_QLineF(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QLineF, value: &ffi::QLineF) -> isize {
    ffi::index_of_QLineF(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QLineF, pos: isize, value: &ffi::QLineF) {
    ffi::insert_QLineF(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QLineF) -> isize {
    ffi::len_QLineF(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QLineF, pos: isize) {
    ffi::remove_QLineF(s, pos);
}
