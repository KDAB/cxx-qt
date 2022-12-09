// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;

        include!("cxx-qt-lib/qvector.h");
        type QVector_QVariant = crate::QVector<QVariant>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QVariant);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QVariant, _: &QVariant) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QVariant"]
        fn construct(_: &QVector_QVariant) -> QVector_QVariant;
        #[rust_name = "qvector_default_QVariant"]
        fn construct() -> QVector_QVariant;
        #[rust_name = "qvector_drop_QVariant"]
        fn drop(_: &mut QVector_QVariant);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "append_QVariant"]
        fn qvectorAppend(_: &mut QVector_QVariant, _: &QVariant);
        #[rust_name = "get_unchecked_QVariant"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QVariant, pos: isize) -> &QVariant;
        #[rust_name = "index_of_QVariant"]
        fn qvectorIndexOf(_: &QVector_QVariant, _: &QVariant) -> isize;
        #[rust_name = "insert_QVariant"]
        fn qvectorInsert(_: &mut QVector_QVariant, _: isize, _: &QVariant);
        #[rust_name = "remove_QVariant"]
        fn qvectorRemove(_: &mut QVector_QVariant, _: isize);
        #[rust_name = "len_QVariant"]
        fn qvectorLen(_: &QVector_QVariant) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QVariant, value: &ffi::QVariant) {
    ffi::append_QVariant(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QVariant) -> ffi::QVector_QVariant {
    ffi::qvector_clone_QVariant(s)
}

pub(crate) fn default() -> ffi::QVector_QVariant {
    ffi::qvector_default_QVariant()
}

pub(crate) fn drop(s: &mut ffi::QVector_QVariant) {
    ffi::qvector_drop_QVariant(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QVariant, pos: isize) -> &ffi::QVariant {
    ffi::get_unchecked_QVariant(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QVariant, value: &ffi::QVariant) -> isize {
    ffi::index_of_QVariant(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QVariant, pos: isize, value: &ffi::QVariant) {
    ffi::insert_QVariant(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QVariant) -> isize {
    ffi::len_QVariant(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QVariant, pos: isize) {
    ffi::remove_QVariant(s, pos);
}
