// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_u8 = crate::QSet<u8>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_u8);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_u8, _: &u8) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_u8, _: &u8) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_u8"]
        fn construct(_: &QSet_u8) -> QSet_u8;
        #[rust_name = "qset_default_u8"]
        fn construct() -> QSet_u8;
        #[rust_name = "qset_drop_u8"]
        fn drop(_: &mut QSet_u8);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_u8"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qsetGetUnchecked<'a>(set: &'a QSet_u8, pos: isize) -> &'a u8;
        #[rust_name = "insert_u8"]
        fn qsetInsert(_: &mut QSet_u8, _: &u8);
        #[rust_name = "len_u8"]
        fn qsetLen(_: &QSet_u8) -> isize;
        #[rust_name = "reserve_u8"]
        fn qsetReserve(_: &mut QSet_u8, size: isize);
    }
}

pub(crate) fn reserve(v: &mut ffi::QSet_u8, size: isize) {
    ffi::reserve_u8(v, size);
}

pub(crate) fn clone(s: &ffi::QSet_u8) -> ffi::QSet_u8 {
    ffi::qset_clone_u8(s)
}

pub(crate) fn default() -> ffi::QSet_u8 {
    ffi::qset_default_u8()
}

pub(crate) fn drop(s: &mut ffi::QSet_u8) {
    ffi::qset_drop_u8(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_u8, pos: isize) -> &u8 {
    ffi::get_unchecked_u8(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_u8, value: &u8) {
    ffi::insert_u8(s, value);
}

pub(crate) fn len(s: &ffi::QSet_u8) -> isize {
    ffi::len_u8(s)
}
