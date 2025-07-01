// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = crate::QUrl;

        include!("cxx-qt-lib/core/qlist/qlist_QUrl.h");
        type QList_QUrl = crate::QList<QUrl>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QUrl);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QUrl, _: &QUrl) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QUrl"]
        fn construct(_: &QList_QUrl) -> QList_QUrl;
        #[rust_name = "qlist_default_QUrl"]
        fn construct() -> QList_QUrl;
        #[rust_name = "qlist_drop_QUrl"]
        fn drop(_: &mut QList_QUrl);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QUrl"]
        fn qlistReserve(_: &mut QList_QUrl, size: isize);
        #[rust_name = "append_QUrl"]
        fn qlistAppend(_: &mut QList_QUrl, _: &QUrl);
        #[rust_name = "get_unchecked_QUrl"]
        unsafe fn qlistGetUnchecked(set: &QList_QUrl, pos: isize) -> &QUrl;
        #[rust_name = "index_of_QUrl"]
        fn qlistIndexOf(_: &QList_QUrl, _: &QUrl) -> isize;
        #[rust_name = "insert_QUrl"]
        fn qlistInsert(_: &mut QList_QUrl, _: isize, _: &QUrl);
        #[rust_name = "remove_QUrl"]
        fn qlistRemove(_: &mut QList_QUrl, _: isize);
        #[rust_name = "len_QUrl"]
        fn qlistLen(_: &QList_QUrl) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QUrl, size: isize) {
    ffi::reserve_QUrl(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QUrl, value: &ffi::QUrl) {
    ffi::append_QUrl(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QUrl) -> ffi::QList_QUrl {
    ffi::qlist_clone_QUrl(s)
}

pub(crate) fn default() -> ffi::QList_QUrl {
    ffi::qlist_default_QUrl()
}

pub(crate) fn drop(s: &mut ffi::QList_QUrl) {
    ffi::qlist_drop_QUrl(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QUrl, pos: isize) -> &ffi::QUrl {
    ffi::get_unchecked_QUrl(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QUrl, value: &ffi::QUrl) -> isize {
    ffi::index_of_QUrl(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QUrl, pos: isize, value: &ffi::QUrl) {
    ffi::insert_QUrl(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QUrl) -> isize {
    ffi::len_QUrl(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QUrl, pos: isize) {
    ffi::remove_QUrl(s, pos);
}
