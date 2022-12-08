// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
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

        #[rust_name = "clone_bool"]
        fn construct(_: &QSet_bool) -> QSet_bool;
        #[rust_name = "default_bool"]
        fn construct() -> QSet_bool;
        #[rust_name = "drop_bool"]
        fn drop(_: &mut QSet_bool);
        #[rust_name = "get_unchecked_bool"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qsetGetUnchecked<'a>(set: &'a QSet_bool, pos: usize) -> &'a bool;
        #[rust_name = "insert_bool"]
        fn qsetInsert(_: &mut QSet_bool, _: &bool);
        #[rust_name = "len_bool"]
        fn qsetLen(_: &QSet_bool) -> usize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_bool) -> ffi::QSet_bool {
    ffi::clone_bool(s)
}

pub(crate) fn default() -> ffi::QSet_bool {
    ffi::default_bool()
}

pub(crate) fn drop(s: &mut ffi::QSet_bool) {
    ffi::drop_bool(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_bool, pos: usize) -> &bool {
    ffi::get_unchecked_bool(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_bool, value: &bool) {
    ffi::insert_bool(s, value);
}

pub(crate) fn len(s: &ffi::QSet_bool) -> usize {
    ffi::len_bool(s)
}
