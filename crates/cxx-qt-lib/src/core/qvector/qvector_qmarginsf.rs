// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmarginsf.h");
        type QMarginsF = crate::QMarginsF;

        include!("cxx-qt-lib/qvector.h");
        type QVector_QMarginsF = crate::QVector<QMarginsF>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QMarginsF);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QMarginsF, _: &QMarginsF) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QMarginsF"]
        fn construct(_: &QVector_QMarginsF) -> QVector_QMarginsF;
        #[rust_name = "qvector_default_QMarginsF"]
        fn construct() -> QVector_QMarginsF;
        #[rust_name = "qvector_drop_QMarginsF"]
        fn drop(_: &mut QVector_QMarginsF);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QMarginsF"]
        fn qvectorReserve(_: &mut QVector_QMarginsF, size: isize);
        #[rust_name = "append_QMarginsF"]
        fn qvectorAppend(_: &mut QVector_QMarginsF, _: &QMarginsF);
        #[rust_name = "get_unchecked_QMarginsF"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QMarginsF, pos: isize) -> &QMarginsF;
        #[rust_name = "index_of_QMarginsF"]
        fn qvectorIndexOf(_: &QVector_QMarginsF, _: &QMarginsF) -> isize;
        #[rust_name = "insert_QMarginsF"]
        fn qvectorInsert(_: &mut QVector_QMarginsF, _: isize, _: &QMarginsF);
        #[rust_name = "remove_QMarginsF"]
        fn qvectorRemove(_: &mut QVector_QMarginsF, _: isize);
        #[rust_name = "len_QMarginsF"]
        fn qvectorLen(_: &QVector_QMarginsF) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QMarginsF, value: &ffi::QMarginsF) {
    ffi::append_QMarginsF(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QMarginsF) -> ffi::QVector_QMarginsF {
    ffi::qvector_clone_QMarginsF(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QMarginsF, size: isize) {
    ffi::reserve_QMarginsF(v, size);
}

pub(crate) fn default() -> ffi::QVector_QMarginsF {
    ffi::qvector_default_QMarginsF()
}

pub(crate) fn drop(s: &mut ffi::QVector_QMarginsF) {
    ffi::qvector_drop_QMarginsF(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QMarginsF, pos: isize) -> &ffi::QMarginsF {
    ffi::get_unchecked_QMarginsF(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QMarginsF, value: &ffi::QMarginsF) -> isize {
    ffi::index_of_QMarginsF(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QMarginsF, pos: isize, value: &ffi::QMarginsF) {
    ffi::insert_QMarginsF(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QMarginsF) -> isize {
    ffi::len_QMarginsF(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QMarginsF, pos: isize) {
    ffi::remove_QMarginsF(s, pos);
}
