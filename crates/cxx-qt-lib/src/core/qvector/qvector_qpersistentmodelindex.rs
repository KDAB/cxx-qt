// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpersistentmodelindex.h");
        type QPersistentModelIndex = crate::QPersistentModelIndex;

        include!("cxx-qt-lib/core/qvector/qvector_QPersistentModelIndex.h");
        type QVector_QPersistentModelIndex = crate::QVector<QPersistentModelIndex>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QPersistentModelIndex);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QPersistentModelIndex, _: &QPersistentModelIndex) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QPersistentModelIndex"]
        fn construct(_: &QVector_QPersistentModelIndex) -> QVector_QPersistentModelIndex;
        #[rust_name = "qvector_default_QPersistentModelIndex"]
        fn construct() -> QVector_QPersistentModelIndex;
        #[rust_name = "qvector_drop_QPersistentModelIndex"]
        fn drop(_: &mut QVector_QPersistentModelIndex);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QPersistentModelIndex"]
        fn qvectorReserve(_: &mut QVector_QPersistentModelIndex, size: isize);
        #[rust_name = "append_QPersistentModelIndex"]
        fn qvectorAppend(_: &mut QVector_QPersistentModelIndex, _: &QPersistentModelIndex);
        #[rust_name = "get_unchecked_QPersistentModelIndex"]
        unsafe fn qvectorGetUnchecked(
            set: &QVector_QPersistentModelIndex,
            pos: isize,
        ) -> &QPersistentModelIndex;
        #[rust_name = "index_of_QPersistentModelIndex"]
        fn qvectorIndexOf(_: &QVector_QPersistentModelIndex, _: &QPersistentModelIndex) -> isize;
        #[rust_name = "insert_QPersistentModelIndex"]
        fn qvectorInsert(
            _: &mut QVector_QPersistentModelIndex,
            _: isize,
            _: &QPersistentModelIndex,
        );
        #[rust_name = "remove_QPersistentModelIndex"]
        fn qvectorRemove(_: &mut QVector_QPersistentModelIndex, _: isize);
        #[rust_name = "len_QPersistentModelIndex"]
        fn qvectorLen(_: &QVector_QPersistentModelIndex) -> isize;
    }
}

pub(crate) fn append(
    v: &mut ffi::QVector_QPersistentModelIndex,
    value: &ffi::QPersistentModelIndex,
) {
    ffi::append_QPersistentModelIndex(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QPersistentModelIndex) -> ffi::QVector_QPersistentModelIndex {
    ffi::qvector_clone_QPersistentModelIndex(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QPersistentModelIndex, size: isize) {
    ffi::reserve_QPersistentModelIndex(v, size);
}

pub(crate) fn default() -> ffi::QVector_QPersistentModelIndex {
    ffi::qvector_default_QPersistentModelIndex()
}

pub(crate) fn drop(s: &mut ffi::QVector_QPersistentModelIndex) {
    ffi::qvector_drop_QPersistentModelIndex(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QVector_QPersistentModelIndex,
    pos: isize,
) -> &ffi::QPersistentModelIndex {
    ffi::get_unchecked_QPersistentModelIndex(s, pos)
}

pub(crate) fn index_of(
    v: &ffi::QVector_QPersistentModelIndex,
    value: &ffi::QPersistentModelIndex,
) -> isize {
    ffi::index_of_QPersistentModelIndex(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QVector_QPersistentModelIndex,
    pos: isize,
    value: &ffi::QPersistentModelIndex,
) {
    ffi::insert_QPersistentModelIndex(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QPersistentModelIndex) -> isize {
    ffi::len_QPersistentModelIndex(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QPersistentModelIndex, pos: isize) {
    ffi::remove_QPersistentModelIndex(s, pos);
}
