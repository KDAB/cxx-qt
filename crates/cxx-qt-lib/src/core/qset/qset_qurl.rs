// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = crate::QUrl;

        include!("cxx-qt-lib/qset.h");
        type QSet_QUrl = crate::QSet<QUrl>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_QUrl);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_QUrl, _: &QUrl) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_QUrl, _: &QUrl) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QUrl"]
        fn construct(_: &QSet_QUrl) -> QSet_QUrl;
        #[rust_name = "qset_default_QUrl"]
        fn construct() -> QSet_QUrl;
        #[rust_name = "qset_drop_QUrl"]
        fn drop(_: &mut QSet_QUrl);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_QUrl"]
        unsafe fn qsetGetUnchecked(set: &QSet_QUrl, pos: isize) -> &QUrl;
        #[rust_name = "insert_QUrl"]
        fn qsetInsert(_: &mut QSet_QUrl, _: &QUrl);
        #[rust_name = "len_QUrl"]
        fn qsetLen(_: &QSet_QUrl) -> isize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_QUrl) -> ffi::QSet_QUrl {
    ffi::qset_clone_QUrl(s)
}

pub(crate) fn default() -> ffi::QSet_QUrl {
    ffi::qset_default_QUrl()
}

pub(crate) fn drop(s: &mut ffi::QSet_QUrl) {
    ffi::qset_drop_QUrl(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_QUrl, pos: isize) -> &ffi::QUrl {
    ffi::get_unchecked_QUrl(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QUrl, value: &ffi::QUrl) {
    ffi::insert_QUrl(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QUrl) -> isize {
    ffi::len_QUrl(s)
}
