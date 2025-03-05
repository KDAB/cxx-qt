// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset_i64.h");
        type QSet_i64 = crate::QSet<i64>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_i64);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_i64, _: &i64) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_i64, _: &i64) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_i64"]
        fn construct(_: &QSet_i64) -> QSet_i64;
        #[rust_name = "qset_default_i64"]
        fn construct() -> QSet_i64;
        #[rust_name = "qset_drop_i64"]
        fn drop(_: &mut QSet_i64);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_i64"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qsetGetUnchecked<'a>(set: &'a QSet_i64, pos: isize) -> &'a i64;
        #[rust_name = "insert_i64"]
        fn qsetInsert(_: &mut QSet_i64, _: &i64);
        #[rust_name = "len_i64"]
        fn qsetLen(_: &QSet_i64) -> isize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_i64) -> ffi::QSet_i64 {
    ffi::qset_clone_i64(s)
}

pub(crate) fn default() -> ffi::QSet_i64 {
    ffi::qset_default_i64()
}

pub(crate) fn drop(s: &mut ffi::QSet_i64) {
    ffi::qset_drop_i64(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_i64, pos: isize) -> &i64 {
    ffi::get_unchecked_i64(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_i64, value: &i64) {
    ffi::insert_i64(s, value);
}

pub(crate) fn len(s: &ffi::QSet_i64) -> isize {
    ffi::len_i64(s)
}
