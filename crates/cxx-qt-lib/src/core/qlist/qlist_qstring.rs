// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qlist_QString.h");
        type QList_QString = crate::QList<QString>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QString);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QString, _: &QString) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QString"]
        fn construct(_: &QList_QString) -> QList_QString;
        #[rust_name = "qlist_default_QString"]
        fn construct() -> QList_QString;
        #[rust_name = "qlist_drop_QString"]
        fn drop(_: &mut QList_QString);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QString"]
        fn qlistReserve(_: &mut QList_QString, size: isize);
        #[rust_name = "append_QString"]
        fn qlistAppend(_: &mut QList_QString, _: &QString);
        #[rust_name = "get_unchecked_QString"]
        unsafe fn qlistGetUnchecked(set: &QList_QString, pos: isize) -> &QString;
        #[rust_name = "index_of_QString"]
        fn qlistIndexOf(_: &QList_QString, _: &QString) -> isize;
        #[rust_name = "insert_QString"]
        fn qlistInsert(_: &mut QList_QString, _: isize, _: &QString);
        #[rust_name = "remove_QString"]
        fn qlistRemove(_: &mut QList_QString, _: isize);
        #[rust_name = "len_QString"]
        fn qlistLen(_: &QList_QString) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QString, size: isize) {
    ffi::reserve_QString(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QString, value: &ffi::QString) {
    ffi::append_QString(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QString) -> ffi::QList_QString {
    ffi::qlist_clone_QString(s)
}

pub(crate) fn default() -> ffi::QList_QString {
    ffi::qlist_default_QString()
}

pub(crate) fn drop(s: &mut ffi::QList_QString) {
    ffi::qlist_drop_QString(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QString, pos: isize) -> &ffi::QString {
    ffi::get_unchecked_QString(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QString, value: &ffi::QString) -> isize {
    ffi::index_of_QString(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QString, pos: isize, value: &ffi::QString) {
    ffi::insert_QString(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QString) -> isize {
    ffi::len_QString(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QString, pos: isize) {
    ffi::remove_QString(s, pos);
}
