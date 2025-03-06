// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/quuid.h");
        type QUuid = crate::QUuid;

        include!("cxx-qt-lib/qset_QUuid.h");
        type QSet_QUuid = crate::QSet<QUuid>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_QUuid);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_QUuid, _: &QUuid) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_QUuid, _: &QUuid) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QUuid"]
        fn construct(_: &QSet_QUuid) -> QSet_QUuid;
        #[rust_name = "qset_default_QUuid"]
        fn construct() -> QSet_QUuid;
        #[rust_name = "qset_drop_QUuid"]
        fn drop(_: &mut QSet_QUuid);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_QUuid"]
        unsafe fn qsetGetUnchecked(set: &QSet_QUuid, pos: isize) -> &QUuid;
        #[rust_name = "insert_QUuid"]
        fn qsetInsert(_: &mut QSet_QUuid, _: &QUuid);
        #[rust_name = "len_QUuid"]
        fn qsetLen(_: &QSet_QUuid) -> isize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_QUuid) -> ffi::QSet_QUuid {
    ffi::qset_clone_QUuid(s)
}

pub(crate) fn default() -> ffi::QSet_QUuid {
    ffi::qset_default_QUuid()
}

pub(crate) fn drop(s: &mut ffi::QSet_QUuid) {
    ffi::qset_drop_QUuid(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_QUuid, pos: isize) -> &ffi::QUuid {
    ffi::get_unchecked_QUuid(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QUuid, value: &ffi::QUuid) {
    ffi::insert_QUuid(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QUuid) -> isize {
    ffi::len_QUuid(s)
}
