// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;

        include!("cxx-qt-lib/qlist.h");
        type QList_QPointF = crate::QList<QPointF>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QPointF);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QPointF, _: &QPointF) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QPointF"]
        fn construct(_: &QList_QPointF) -> QList_QPointF;
        #[rust_name = "qlist_default_QPointF"]
        fn construct() -> QList_QPointF;
        #[rust_name = "qlist_drop_QPointF"]
        fn drop(_: &mut QList_QPointF);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "append_QPointF"]
        fn qlistAppend(_: &mut QList_QPointF, _: &QPointF);
        #[rust_name = "get_unchecked_QPointF"]
        unsafe fn qlistGetUnchecked(set: &QList_QPointF, pos: isize) -> &QPointF;
        #[rust_name = "index_of_QPointF"]
        fn qlistIndexOf(_: &QList_QPointF, _: &QPointF) -> isize;
        #[rust_name = "insert_QPointF"]
        fn qlistInsert(_: &mut QList_QPointF, _: isize, _: &QPointF);
        #[rust_name = "remove_QPointF"]
        fn qlistRemove(_: &mut QList_QPointF, _: isize);
        #[rust_name = "len_QPointF"]
        fn qlistLen(_: &QList_QPointF) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QList_QPointF, value: &ffi::QPointF) {
    ffi::append_QPointF(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QPointF) -> ffi::QList_QPointF {
    ffi::qlist_clone_QPointF(s)
}

pub(crate) fn default() -> ffi::QList_QPointF {
    ffi::qlist_default_QPointF()
}

pub(crate) fn drop(s: &mut ffi::QList_QPointF) {
    ffi::qlist_drop_QPointF(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QPointF, pos: isize) -> &ffi::QPointF {
    ffi::get_unchecked_QPointF(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QPointF, value: &ffi::QPointF) -> isize {
    ffi::index_of_QPointF(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QPointF, pos: isize, value: &ffi::QPointF) {
    ffi::insert_QPointF(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QPointF) -> isize {
    ffi::len_QPointF(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QPointF, pos: isize) {
    ffi::remove_QPointF(s, pos);
}
