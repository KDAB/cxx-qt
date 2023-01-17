// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpersistentmodelindex.h");
        type QPersistentModelIndex = crate::QPersistentModelIndex;

        include!("cxx-qt-lib/qset.h");
        type QSet_QPersistentModelIndex = crate::QSet<QPersistentModelIndex>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_QPersistentModelIndex);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_QPersistentModelIndex, _: &QPersistentModelIndex) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_QPersistentModelIndex, _: &QPersistentModelIndex) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QPersistentModelIndex"]
        fn construct(_: &QSet_QPersistentModelIndex) -> QSet_QPersistentModelIndex;
        #[rust_name = "qset_default_QPersistentModelIndex"]
        fn construct() -> QSet_QPersistentModelIndex;
        #[rust_name = "qset_drop_QPersistentModelIndex"]
        fn drop(_: &mut QSet_QPersistentModelIndex);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_QPersistentModelIndex"]
        unsafe fn qsetGetUnchecked(
            set: &QSet_QPersistentModelIndex,
            pos: isize,
        ) -> &QPersistentModelIndex;
        #[rust_name = "insert_QPersistentModelIndex"]
        fn qsetInsert(_: &mut QSet_QPersistentModelIndex, _: &QPersistentModelIndex);
        #[rust_name = "len_QPersistentModelIndex"]
        fn qsetLen(_: &QSet_QPersistentModelIndex) -> isize;
    }
}

pub(crate) fn clone(s: &ffi::QSet_QPersistentModelIndex) -> ffi::QSet_QPersistentModelIndex {
    ffi::qset_clone_QPersistentModelIndex(s)
}

pub(crate) fn default() -> ffi::QSet_QPersistentModelIndex {
    ffi::qset_default_QPersistentModelIndex()
}

pub(crate) fn drop(s: &mut ffi::QSet_QPersistentModelIndex) {
    ffi::qset_drop_QPersistentModelIndex(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QSet_QPersistentModelIndex,
    pos: isize,
) -> &ffi::QPersistentModelIndex {
    ffi::get_unchecked_QPersistentModelIndex(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QPersistentModelIndex, value: &ffi::QPersistentModelIndex) {
    ffi::insert_QPersistentModelIndex(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QPersistentModelIndex) -> isize {
    ffi::len_QPersistentModelIndex(s)
}
