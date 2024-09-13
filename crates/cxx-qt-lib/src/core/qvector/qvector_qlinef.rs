// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlinef.h");
        type QLineF = crate::QLineF;

        include!("cxx-qt-lib/qvector.h");
        type QVector_QLineF = crate::QVector<QLineF>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QLineF);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QLineF, _: &QLineF) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QLineF"]
        fn construct(_: &QVector_QLineF) -> QVector_QLineF;
        #[rust_name = "qvector_default_QLineF"]
        fn construct() -> QVector_QLineF;
        #[rust_name = "qvector_drop_QLineF"]
        fn drop(_: &mut QVector_QLineF);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QLineF"]
        fn qvectorReserve(_: &mut QVector_QLineF, size: isize);
        #[rust_name = "append_QLineF"]
        fn qvectorAppend(_: &mut QVector_QLineF, _: &QLineF);
        #[rust_name = "get_unchecked_QLineF"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QLineF, pos: isize) -> &QLineF;
        #[rust_name = "index_of_QLineF"]
        fn qvectorIndexOf(_: &QVector_QLineF, _: &QLineF) -> isize;
        #[rust_name = "insert_QLineF"]
        fn qvectorInsert(_: &mut QVector_QLineF, _: isize, _: &QLineF);
        #[rust_name = "remove_QLineF"]
        fn qvectorRemove(_: &mut QVector_QLineF, _: isize);
        #[rust_name = "len_QLineF"]
        fn qvectorLen(_: &QVector_QLineF) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QLineF, value: &ffi::QLineF) {
    ffi::append_QLineF(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QLineF) -> ffi::QVector_QLineF {
    ffi::qvector_clone_QLineF(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QLineF, size: isize) {
    ffi::reserve_QLineF(v, size);
}

pub(crate) fn default() -> ffi::QVector_QLineF {
    ffi::qvector_default_QLineF()
}

pub(crate) fn drop(s: &mut ffi::QVector_QLineF) {
    ffi::qvector_drop_QLineF(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QLineF, pos: isize) -> &ffi::QLineF {
    ffi::get_unchecked_QLineF(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QLineF, value: &ffi::QLineF) -> isize {
    ffi::index_of_QLineF(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QLineF, pos: isize, value: &ffi::QLineF) {
    ffi::insert_QLineF(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QLineF) -> isize {
    ffi::len_QLineF(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QLineF, pos: isize) {
    ffi::remove_QLineF(s, pos);
}
