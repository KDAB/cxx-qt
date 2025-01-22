// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_i8 = crate::QSet<i8>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_i8);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_i8, _: &i8) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_i8, _: &i8) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_i8"]
        fn construct(_: &QSet_i8) -> QSet_i8;
        #[rust_name = "qset_default_i8"]
        fn construct() -> QSet_i8;
        #[rust_name = "qset_drop_i8"]
        fn drop(_: &mut QSet_i8);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_i8"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qsetGetUnchecked<'a>(set: &'a QSet_i8, pos: isize) -> &'a i8;
        #[rust_name = "insert_i8"]
        fn qsetInsert(_: &mut QSet_i8, _: &i8);
        #[rust_name = "len_i8"]
        fn qsetLen(_: &QSet_i8) -> isize;
        #[rust_name = "reserve_i8"]
        fn qsetReserve(_: &mut QSet_i8, size: isize);
    }
}

pub(crate) fn clone(s: &ffi::QSet_i8) -> ffi::QSet_i8 {
    ffi::qset_clone_i8(s)
}

pub(crate) fn default() -> ffi::QSet_i8 {
    ffi::qset_default_i8()
}

pub(crate) fn drop(s: &mut ffi::QSet_i8) {
    ffi::qset_drop_i8(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_i8, pos: isize) -> &i8 {
    ffi::get_unchecked_i8(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_i8, value: &i8) {
    ffi::insert_i8(s, value);
}

pub(crate) fn len(s: &ffi::QSet_i8) -> isize {
    ffi::len_i8(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_i8, size: isize) {
    ffi::reserve_i8(s, size);
}
