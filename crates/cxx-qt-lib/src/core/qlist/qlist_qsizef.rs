// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = crate::QSizeF;

        include!("cxx-qt-lib/qlist_QSizeF.h");
        type QList_QSizeF = crate::QList<QSizeF>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QSizeF);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QSizeF, _: &QSizeF) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QSizeF"]
        fn construct(_: &QList_QSizeF) -> QList_QSizeF;
        #[rust_name = "qlist_default_QSizeF"]
        fn construct() -> QList_QSizeF;
        #[rust_name = "qlist_drop_QSizeF"]
        fn drop(_: &mut QList_QSizeF);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QSizeF"]
        fn qlistReserve(_: &mut QList_QSizeF, size: isize);
        #[rust_name = "append_QSizeF"]
        fn qlistAppend(_: &mut QList_QSizeF, _: &QSizeF);
        #[rust_name = "get_unchecked_QSizeF"]
        unsafe fn qlistGetUnchecked(set: &QList_QSizeF, pos: isize) -> &QSizeF;
        #[rust_name = "index_of_QSizeF"]
        fn qlistIndexOf(_: &QList_QSizeF, _: &QSizeF) -> isize;
        #[rust_name = "insert_QSizeF"]
        fn qlistInsert(_: &mut QList_QSizeF, _: isize, _: &QSizeF);
        #[rust_name = "remove_QSizeF"]
        fn qlistRemove(_: &mut QList_QSizeF, _: isize);
        #[rust_name = "len_QSizeF"]
        fn qlistLen(_: &QList_QSizeF) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QSizeF, size: isize) {
    ffi::reserve_QSizeF(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QSizeF, value: &ffi::QSizeF) {
    ffi::append_QSizeF(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QSizeF) -> ffi::QList_QSizeF {
    ffi::qlist_clone_QSizeF(s)
}

pub(crate) fn default() -> ffi::QList_QSizeF {
    ffi::qlist_default_QSizeF()
}

pub(crate) fn drop(s: &mut ffi::QList_QSizeF) {
    ffi::qlist_drop_QSizeF(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QSizeF, pos: isize) -> &ffi::QSizeF {
    ffi::get_unchecked_QSizeF(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QSizeF, value: &ffi::QSizeF) -> isize {
    ffi::index_of_QSizeF(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QSizeF, pos: isize, value: &ffi::QSizeF) {
    ffi::insert_QSizeF(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSizeF) -> isize {
    ffi::len_QSizeF(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSizeF, pos: isize) {
    ffi::remove_QSizeF(s, pos);
}
