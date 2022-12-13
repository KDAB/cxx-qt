// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_u64 = crate::QSet<u64>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_u64);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_u64, _: &u64) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_u64, _: &u64) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "clone_u64"]
        fn construct(_: &QSet_u64) -> QSet_u64;
        #[rust_name = "default_u64"]
        fn construct() -> QSet_u64;
        #[rust_name = "drop_u64"]
        fn drop(_: &mut QSet_u64);
        #[rust_name = "get_unchecked_u64"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qsetGetUnchecked<'a>(set: &'a QSet_u64, pos: isize) -> &'a u64;
        #[rust_name = "insert_u64"]
        fn qsetInsert(_: &mut QSet_u64, _: &u64);
        #[rust_name = "len_u64"]
        fn qsetLen(_: &QSet_u64) -> isize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_u64) -> ffi::QSet_u64 {
    ffi::clone_u64(s)
}

pub(crate) fn default() -> ffi::QSet_u64 {
    ffi::default_u64()
}

pub(crate) fn drop(s: &mut ffi::QSet_u64) {
    ffi::drop_u64(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_u64, pos: isize) -> &u64 {
    ffi::get_unchecked_u64(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_u64, value: &u64) {
    ffi::insert_u64(s, value);
}

pub(crate) fn len(s: &ffi::QSet_u64) -> isize {
    ffi::len_u64(s)
}
