// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector.h");
        type QVector_f64 = crate::QVector<f64>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_f64);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_f64, _: &f64) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_f64"]
        fn construct(_: &QVector_f64) -> QVector_f64;
        #[rust_name = "qvector_default_f64"]
        fn construct() -> QVector_f64;
        #[rust_name = "qvector_drop_f64"]
        fn drop(_: &mut QVector_f64);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_f64"]
        fn qvectorReserve(_: &mut QVector_f64, size: isize);
        #[rust_name = "append_f64"]
        fn qvectorAppend(_: &mut QVector_f64, _: &f64);
        #[rust_name = "get_unchecked_f64"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_f64, pos: isize) -> &'a f64;
        #[rust_name = "index_of_f64"]
        fn qvectorIndexOf(_: &QVector_f64, _: &f64) -> isize;
        #[rust_name = "insert_f64"]
        fn qvectorInsert(_: &mut QVector_f64, _: isize, _: &f64);
        #[rust_name = "len_f64"]
        fn qvectorLen(_: &QVector_f64) -> isize;
        #[rust_name = "remove_f64"]
        fn qvectorRemove(_: &mut QVector_f64, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_f64, value: &f64) {
    ffi::append_f64(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_f64) -> ffi::QVector_f64 {
    ffi::qvector_clone_f64(v)
}

pub(crate) fn reserve(v: &mut ffi::QVector_f64, size: isize) {
    ffi::reserve_f64(v, size);
}

pub(crate) fn default() -> ffi::QVector_f64 {
    ffi::qvector_default_f64()
}

pub(crate) fn drop(v: &mut ffi::QVector_f64) {
    ffi::qvector_drop_f64(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_f64, pos: isize) -> &f64 {
    ffi::get_unchecked_f64(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_f64, value: &f64) -> isize {
    ffi::index_of_f64(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_f64, pos: isize, value: &f64) {
    ffi::insert_f64(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_f64) -> isize {
    ffi::len_f64(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_f64, pos: isize) {
    ffi::remove_f64(s, pos);
}
