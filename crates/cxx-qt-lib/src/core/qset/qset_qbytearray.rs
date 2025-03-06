// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;

        include!("cxx-qt-lib/qset_QByteArray.h");
        type QSet_QByteArray = crate::QSet<QByteArray>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_QByteArray);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_QByteArray, _: &QByteArray) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_QByteArray, _: &QByteArray) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QByteArray"]
        fn construct(_: &QSet_QByteArray) -> QSet_QByteArray;
        #[rust_name = "qset_default_QByteArray"]
        fn construct() -> QSet_QByteArray;
        #[rust_name = "qset_drop_QByteArray"]
        fn drop(_: &mut QSet_QByteArray);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_QByteArray"]
        unsafe fn qsetGetUnchecked(set: &QSet_QByteArray, pos: isize) -> &QByteArray;
        #[rust_name = "insert_QByteArray"]
        fn qsetInsert(_: &mut QSet_QByteArray, _: &QByteArray);
        #[rust_name = "len_QByteArray"]
        fn qsetLen(_: &QSet_QByteArray) -> isize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_QByteArray) -> ffi::QSet_QByteArray {
    ffi::qset_clone_QByteArray(s)
}

pub(crate) fn default() -> ffi::QSet_QByteArray {
    ffi::qset_default_QByteArray()
}

pub(crate) fn drop(s: &mut ffi::QSet_QByteArray) {
    ffi::qset_drop_QByteArray(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_QByteArray, pos: isize) -> &ffi::QByteArray {
    ffi::get_unchecked_QByteArray(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QByteArray, value: &ffi::QByteArray) {
    ffi::insert_QByteArray(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QByteArray) -> isize {
    ffi::len_QByteArray(s)
}
