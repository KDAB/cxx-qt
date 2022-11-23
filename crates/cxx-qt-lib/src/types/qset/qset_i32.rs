// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
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
        #[rust_name = "clone_i32"]
        fn qset_clone(_: &QSet_i32) -> QSet_i32;
        #[rust_name = "default_i32"]
        fn qset_default() -> QSet_i32;
        #[rust_name = "drop_i32"]
        fn qset_drop(_: &mut QSet_i32);
        #[rust_name = "get_unchecked_i32"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qset_get_unchecked<'a>(set: &'a QSet_i32, pos: usize) -> &'a i32;
        #[rust_name = "insert_i32"]
        fn qset_insert(_: &mut QSet_i32, _: &i32);
        #[rust_name = "len_i32"]
        fn qset_len(_: &QSet_i32) -> usize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_i32) -> ffi::QSet_i32 {
    ffi::clone_i32(s)
}

pub(crate) fn default() -> ffi::QSet_i32 {
    ffi::default_i32()
}

pub(crate) fn drop(s: &mut ffi::QSet_i32) {
    ffi::drop_i32(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_i32, pos: usize) -> &i32 {
    ffi::get_unchecked_i32(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_i32, value: &i32) {
    ffi::insert_i32(s, value);
}

pub(crate) fn len(s: &ffi::QSet_i32) -> usize {
    ffi::len_i32(s)
}
