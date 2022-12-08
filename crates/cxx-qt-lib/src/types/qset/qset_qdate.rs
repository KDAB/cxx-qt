// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdate.h");
        type QDate = crate::QDate;

        include!("cxx-qt-lib/qset.h");
        type QSet_QDate = crate::QSet<QDate>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_QDate);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_QDate, _: &QDate) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_QDate, _: &QDate) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "clone_QDate"]
        fn construct(_: &QSet_QDate) -> QSet_QDate;
        #[rust_name = "default_QDate"]
        fn construct() -> QSet_QDate;
        #[rust_name = "drop_QDate"]
        fn drop(_: &mut QSet_QDate);
        #[rust_name = "get_unchecked_QDate"]
        unsafe fn qsetGetUnchecked(set: &QSet_QDate, pos: usize) -> &QDate;
        #[rust_name = "insert_QDate"]
        fn qsetInsert(_: &mut QSet_QDate, _: &QDate);
        #[rust_name = "len_QDate"]
        fn qsetLen(_: &QSet_QDate) -> usize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_QDate) -> ffi::QSet_QDate {
    ffi::clone_QDate(s)
}

pub(crate) fn default() -> ffi::QSet_QDate {
    ffi::default_QDate()
}

pub(crate) fn drop(s: &mut ffi::QSet_QDate) {
    ffi::drop_QDate(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_QDate, pos: usize) -> &ffi::QDate {
    ffi::get_unchecked_QDate(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QDate, value: &ffi::QDate) {
    ffi::insert_QDate(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QDate) -> usize {
    ffi::len_QDate(s)
}
