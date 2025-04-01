// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset_i16.h");
        type QSet_i16 = crate::QSet<i16>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_i16);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_i16, _: &i16) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_i16, _: &i16) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_i16"]
        fn construct(_: &QSet_i16) -> QSet_i16;
        #[rust_name = "qset_default_i16"]
        fn construct() -> QSet_i16;
        #[rust_name = "qset_drop_i16"]
        fn drop(_: &mut QSet_i16);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_i16"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qsetGetUnchecked<'a>(set: &'a QSet_i16, pos: isize) -> &'a i16;
        #[rust_name = "insert_i16"]
        fn qsetInsert(_: &mut QSet_i16, _: &i16);
        #[rust_name = "len_i16"]
        fn qsetLen(_: &QSet_i16) -> isize;
        #[rust_name = "reserve_i16"]
        fn qsetReserve(_: &mut QSet_i16, size: isize);
    }
}

pub(crate) fn clone(s: &ffi::QSet_i16) -> ffi::QSet_i16 {
    ffi::qset_clone_i16(s)
}

pub(crate) fn default() -> ffi::QSet_i16 {
    ffi::qset_default_i16()
}

pub(crate) fn drop(s: &mut ffi::QSet_i16) {
    ffi::qset_drop_i16(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_i16, pos: isize) -> &i16 {
    ffi::get_unchecked_i16(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_i16, value: &i16) {
    ffi::insert_i16(s, value);
}

pub(crate) fn len(s: &ffi::QSet_i16) -> isize {
    ffi::len_i16(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_i16, size: isize) {
    ffi::reserve_i16(s, size);
}
