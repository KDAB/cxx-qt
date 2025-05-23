// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmargins.h");
        type QMargins = crate::QMargins;

        include!("cxx-qt-lib/core/qlist/qlist_QMargins.h");
        type QList_QMargins = crate::QList<QMargins>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QMargins);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QMargins, _: &QMargins) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QMargins"]
        fn construct(_: &QList_QMargins) -> QList_QMargins;
        #[rust_name = "qlist_default_QMargins"]
        fn construct() -> QList_QMargins;
        #[rust_name = "qlist_drop_QMargins"]
        fn drop(_: &mut QList_QMargins);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QMargins"]
        fn qlistReserve(_: &mut QList_QMargins, size: isize);
        #[rust_name = "append_QMargins"]
        fn qlistAppend(_: &mut QList_QMargins, _: &QMargins);
        #[rust_name = "get_unchecked_QMargins"]
        unsafe fn qlistGetUnchecked(set: &QList_QMargins, pos: isize) -> &QMargins;
        #[rust_name = "index_of_QMargins"]
        fn qlistIndexOf(_: &QList_QMargins, _: &QMargins) -> isize;
        #[rust_name = "insert_QMargins"]
        fn qlistInsert(_: &mut QList_QMargins, _: isize, _: &QMargins);
        #[rust_name = "remove_QMargins"]
        fn qlistRemove(_: &mut QList_QMargins, _: isize);
        #[rust_name = "len_QMargins"]
        fn qlistLen(_: &QList_QMargins) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QMargins, size: isize) {
    ffi::reserve_QMargins(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QMargins, value: &ffi::QMargins) {
    ffi::append_QMargins(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QMargins) -> ffi::QList_QMargins {
    ffi::qlist_clone_QMargins(s)
}

pub(crate) fn default() -> ffi::QList_QMargins {
    ffi::qlist_default_QMargins()
}

pub(crate) fn drop(s: &mut ffi::QList_QMargins) {
    ffi::qlist_drop_QMargins(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QMargins, pos: isize) -> &ffi::QMargins {
    ffi::get_unchecked_QMargins(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QMargins, value: &ffi::QMargins) -> isize {
    ffi::index_of_QMargins(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QMargins, pos: isize, value: &ffi::QMargins) {
    ffi::insert_QMargins(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QMargins) -> isize {
    ffi::len_QMargins(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QMargins, pos: isize) {
    ffi::remove_QMargins(s, pos);
}
