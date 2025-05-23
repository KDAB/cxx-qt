// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qset/qset_i32.h");
        type QSet_i32 = crate::QSet<i32>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_i32);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_i32, _: &i32) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_i32, _: &i32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_i32"]
        fn construct(_: &QSet_i32) -> QSet_i32;
        #[rust_name = "qset_default_i32"]
        fn construct() -> QSet_i32;
        #[rust_name = "qset_drop_i32"]
        fn drop(_: &mut QSet_i32);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_i32"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qsetGetUnchecked<'a>(set: &'a QSet_i32, pos: isize) -> &'a i32;
        #[rust_name = "insert_i32"]
        fn qsetInsert(_: &mut QSet_i32, _: &i32);
        #[rust_name = "len_i32"]
        fn qsetLen(_: &QSet_i32) -> isize;
        #[rust_name = "reserve_i32"]
        fn qsetReserve(_: &mut QSet_i32, size: isize);
    }
}

pub(crate) fn clone(s: &ffi::QSet_i32) -> ffi::QSet_i32 {
    ffi::qset_clone_i32(s)
}

pub(crate) fn default() -> ffi::QSet_i32 {
    ffi::qset_default_i32()
}

pub(crate) fn drop(s: &mut ffi::QSet_i32) {
    ffi::qset_drop_i32(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_i32, pos: isize) -> &i32 {
    ffi::get_unchecked_i32(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_i32, value: &i32) {
    ffi::insert_i32(s, value);
}

pub(crate) fn len(s: &ffi::QSet_i32) -> isize {
    ffi::len_i32(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_i32, size: isize) {
    ffi::reserve_i32(s, size);
}
