// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_f32 = crate::QSet<f32>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_f32);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_f32, _: &f32) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_f32, _: &f32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[rust_name = "clone_f32"]
        fn qset_clone(_: &QSet_f32) -> QSet_f32;
        #[rust_name = "default_f32"]
        fn qset_default() -> QSet_f32;
        #[rust_name = "drop_f32"]
        fn qset_drop(_: &mut QSet_f32);
        #[rust_name = "get_unchecked_f32"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qset_get_unchecked<'a>(set: &'a QSet_f32, pos: usize) -> &'a f32;
        #[rust_name = "insert_f32"]
        fn qset_insert(_: &mut QSet_f32, _: &f32);
        #[rust_name = "len_f32"]
        fn qset_len(_: &QSet_f32) -> usize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_f32) -> ffi::QSet_f32 {
    ffi::clone_f32(s)
}

pub(crate) fn default() -> ffi::QSet_f32 {
    ffi::default_f32()
}

pub(crate) fn drop(s: &mut ffi::QSet_f32) {
    ffi::drop_f32(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_f32, pos: usize) -> &f32 {
    ffi::get_unchecked_f32(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_f32, value: &f32) {
    ffi::insert_f32(s, value);
}

pub(crate) fn len(s: &ffi::QSet_f32) -> usize {
    ffi::len_f32(s)
}
