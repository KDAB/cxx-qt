// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_f64 = crate::QSet<f64>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_f64);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_f64, _: &f64) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_f64, _: &f64) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "clone_f64"]
        fn construct(_: &QSet_f64) -> QSet_f64;
        #[rust_name = "default_f64"]
        fn construct() -> QSet_f64;
        #[rust_name = "drop_f64"]
        fn drop(_: &mut QSet_f64);
        #[rust_name = "get_unchecked_f64"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qsetGetUnchecked<'a>(set: &'a QSet_f64, pos: usize) -> &'a f64;
        #[rust_name = "insert_f64"]
        fn qsetInsert(_: &mut QSet_f64, _: &f64);
        #[rust_name = "len_f64"]
        fn qsetLen(_: &QSet_f64) -> usize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_f64) -> ffi::QSet_f64 {
    ffi::clone_f64(s)
}

pub(crate) fn default() -> ffi::QSet_f64 {
    ffi::default_f64()
}

pub(crate) fn drop(s: &mut ffi::QSet_f64) {
    ffi::drop_f64(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_f64, pos: usize) -> &f64 {
    ffi::get_unchecked_f64(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_f64, value: &f64) {
    ffi::insert_f64(s, value);
}

pub(crate) fn len(s: &ffi::QSet_f64) -> usize {
    ffi::len_f64(s)
}
