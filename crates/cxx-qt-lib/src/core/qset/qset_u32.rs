// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset_u32.h");
        type QSet_u32 = crate::QSet<u32>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_u32);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_u32, _: &u32) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_u32, _: &u32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_u32"]
        fn construct(_: &QSet_u32) -> QSet_u32;
        #[rust_name = "qset_default_u32"]
        fn construct() -> QSet_u32;
        #[rust_name = "qset_drop_u32"]
        fn drop(_: &mut QSet_u32);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_u32"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qsetGetUnchecked<'a>(set: &'a QSet_u32, pos: isize) -> &'a u32;
        #[rust_name = "insert_u32"]
        fn qsetInsert(_: &mut QSet_u32, _: &u32);
        #[rust_name = "len_u32"]
        fn qsetLen(_: &QSet_u32) -> isize;
        #[rust_name = "reserve_u32"]
        fn qsetReserve(_: &mut QSet_u32, size: isize);
    }
}

pub(crate) fn clone(s: &ffi::QSet_u32) -> ffi::QSet_u32 {
    ffi::qset_clone_u32(s)
}

pub(crate) fn default() -> ffi::QSet_u32 {
    ffi::qset_default_u32()
}

pub(crate) fn drop(s: &mut ffi::QSet_u32) {
    ffi::qset_drop_u32(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_u32, pos: isize) -> &u32 {
    ffi::get_unchecked_u32(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_u32, value: &u32) {
    ffi::insert_u32(s, value);
}

pub(crate) fn len(s: &ffi::QSet_u32) -> isize {
    ffi::len_u32(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_u32, size: isize) {
    ffi::reserve_u32(s, size);
}
