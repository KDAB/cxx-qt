// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset_bool.h");
        type QSet_bool = crate::QSet<bool>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_bool);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_bool, _: &bool) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_bool, _: &bool) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_bool"]
        fn construct(_: &QSet_bool) -> QSet_bool;
        #[rust_name = "qset_default_bool"]
        fn construct() -> QSet_bool;
        #[rust_name = "qset_drop_bool"]
        fn drop(_: &mut QSet_bool);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_bool"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qsetGetUnchecked<'a>(set: &'a QSet_bool, pos: isize) -> &'a bool;
        #[rust_name = "insert_bool"]
        fn qsetInsert(_: &mut QSet_bool, _: &bool);
        #[rust_name = "len_bool"]
        fn qsetLen(_: &QSet_bool) -> isize;
        #[rust_name = "reserve_bool"]
        fn qsetReserve(_: &mut QSet_bool, size: isize);
    }
}

pub(crate) fn clone(s: &ffi::QSet_bool) -> ffi::QSet_bool {
    ffi::qset_clone_bool(s)
}

pub(crate) fn default() -> ffi::QSet_bool {
    ffi::qset_default_bool()
}

pub(crate) fn drop(s: &mut ffi::QSet_bool) {
    ffi::qset_drop_bool(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_bool, pos: isize) -> &bool {
    ffi::get_unchecked_bool(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_bool, value: &bool) {
    ffi::insert_bool(s, value);
}

pub(crate) fn len(s: &ffi::QSet_bool) -> isize {
    ffi::len_bool(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_bool, size: isize) {
    ffi::reserve_bool(s, size);
}
