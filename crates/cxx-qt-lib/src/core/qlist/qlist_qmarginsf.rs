// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmarginsf.h");
        type QMarginsF = crate::QMarginsF;

        include!("cxx-qt-lib/qlist_QMarginsF.h");
        type QList_QMarginsF = crate::QList<QMarginsF>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QMarginsF);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QMarginsF, _: &QMarginsF) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QMarginsF"]
        fn construct(_: &QList_QMarginsF) -> QList_QMarginsF;
        #[rust_name = "qlist_default_QMarginsF"]
        fn construct() -> QList_QMarginsF;
        #[rust_name = "qlist_drop_QMarginsF"]
        fn drop(_: &mut QList_QMarginsF);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QMarginsF"]
        fn qlistReserve(_: &mut QList_QMarginsF, size: isize);
        #[rust_name = "append_QMarginsF"]
        fn qlistAppend(_: &mut QList_QMarginsF, _: &QMarginsF);
        #[rust_name = "get_unchecked_QMarginsF"]
        unsafe fn qlistGetUnchecked(set: &QList_QMarginsF, pos: isize) -> &QMarginsF;
        #[rust_name = "index_of_QMarginsF"]
        fn qlistIndexOf(_: &QList_QMarginsF, _: &QMarginsF) -> isize;
        #[rust_name = "insert_QMarginsF"]
        fn qlistInsert(_: &mut QList_QMarginsF, _: isize, _: &QMarginsF);
        #[rust_name = "remove_QMarginsF"]
        fn qlistRemove(_: &mut QList_QMarginsF, _: isize);
        #[rust_name = "len_QMarginsF"]
        fn qlistLen(_: &QList_QMarginsF) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QMarginsF, size: isize) {
    ffi::reserve_QMarginsF(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QMarginsF, value: &ffi::QMarginsF) {
    ffi::append_QMarginsF(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QMarginsF) -> ffi::QList_QMarginsF {
    ffi::qlist_clone_QMarginsF(s)
}

pub(crate) fn default() -> ffi::QList_QMarginsF {
    ffi::qlist_default_QMarginsF()
}

pub(crate) fn drop(s: &mut ffi::QList_QMarginsF) {
    ffi::qlist_drop_QMarginsF(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QMarginsF, pos: isize) -> &ffi::QMarginsF {
    ffi::get_unchecked_QMarginsF(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QMarginsF, value: &ffi::QMarginsF) -> isize {
    ffi::index_of_QMarginsF(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QMarginsF, pos: isize, value: &ffi::QMarginsF) {
    ffi::insert_QMarginsF(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QMarginsF) -> isize {
    ffi::len_QMarginsF(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QMarginsF, pos: isize) {
    ffi::remove_QMarginsF(s, pos);
}
