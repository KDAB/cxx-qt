// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qset.h");
        type QSet_QString = crate::QSet<QString>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_QString);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_QString, _: &QString) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_QString, _: &QString) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "clone_QString"]
        fn construct(_: &QSet_QString) -> QSet_QString;
        #[rust_name = "default_QString"]
        fn construct() -> QSet_QString;
        #[rust_name = "drop_QString"]
        fn drop(_: &mut QSet_QString);
        #[rust_name = "get_unchecked_QString"]
        unsafe fn qsetGetUnchecked(set: &QSet_QString, pos: usize) -> &QString;
        #[rust_name = "insert_QString"]
        fn qsetInsert(_: &mut QSet_QString, _: &QString);
        #[rust_name = "len_QString"]
        fn qsetLen(_: &QSet_QString) -> usize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_QString) -> ffi::QSet_QString {
    ffi::clone_QString(s)
}

pub(crate) fn default() -> ffi::QSet_QString {
    ffi::default_QString()
}

pub(crate) fn drop(s: &mut ffi::QSet_QString) {
    ffi::drop_QString(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_QString, pos: usize) -> &ffi::QString {
    ffi::get_unchecked_QString(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QString, value: &ffi::QString) {
    ffi::insert_QString(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QString) -> usize {
    ffi::len_QString(s)
}
