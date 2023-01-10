// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = crate::QDateTime;

        include!("cxx-qt-lib/qset.h");
        type QSet_QDateTime = crate::QSet<QDateTime>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_QDateTime);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_QDateTime, _: &QDateTime) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_QDateTime, _: &QDateTime) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QDateTime"]
        fn construct(_: &QSet_QDateTime) -> QSet_QDateTime;
        #[rust_name = "qset_default_QDateTime"]
        fn construct() -> QSet_QDateTime;
        #[rust_name = "qset_drop_QDateTime"]
        fn drop(_: &mut QSet_QDateTime);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_QDateTime"]
        unsafe fn qsetGetUnchecked(set: &QSet_QDateTime, pos: isize) -> &QDateTime;
        #[rust_name = "insert_QDateTime"]
        fn qsetInsert(_: &mut QSet_QDateTime, _: &QDateTime);
        #[rust_name = "len_QDateTime"]
        fn qsetLen(_: &QSet_QDateTime) -> isize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_QDateTime) -> ffi::QSet_QDateTime {
    ffi::qset_clone_QDateTime(s)
}

pub(crate) fn default() -> ffi::QSet_QDateTime {
    ffi::qset_default_QDateTime()
}

pub(crate) fn drop(s: &mut ffi::QSet_QDateTime) {
    ffi::qset_drop_QDateTime(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_QDateTime, pos: isize) -> &ffi::QDateTime {
    ffi::get_unchecked_QDateTime(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QDateTime, value: &ffi::QDateTime) {
    ffi::insert_QDateTime(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QDateTime) -> isize {
    ffi::len_QDateTime(s)
}
