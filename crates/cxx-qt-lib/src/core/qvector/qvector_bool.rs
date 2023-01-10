// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector.h");
        type QVector_bool = crate::QVector<bool>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_bool);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_bool, _: &bool) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_bool"]
        fn construct(_: &QVector_bool) -> QVector_bool;
        #[rust_name = "qvector_default_bool"]
        fn construct() -> QVector_bool;
        #[rust_name = "qvector_drop_bool"]
        fn drop(_: &mut QVector_bool);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_bool"]
        fn qvectorReserve(_: &mut QVector_bool, size: isize);
        #[rust_name = "append_bool"]
        fn qvectorAppend(_: &mut QVector_bool, _: &bool);
        #[rust_name = "get_unchecked_bool"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_bool, pos: isize) -> &'a bool;
        #[rust_name = "index_of_bool"]
        fn qvectorIndexOf(_: &QVector_bool, _: &bool) -> isize;
        #[rust_name = "insert_bool"]
        fn qvectorInsert(_: &mut QVector_bool, _: isize, _: &bool);
        #[rust_name = "len_bool"]
        fn qvectorLen(_: &QVector_bool) -> isize;
        #[rust_name = "remove_bool"]
        fn qvectorRemove(_: &mut QVector_bool, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_bool, value: &bool) {
    ffi::append_bool(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_bool) -> ffi::QVector_bool {
    ffi::qvector_clone_bool(v)
}

pub(crate) fn reserve(v: &mut ffi::QVector_bool, size: isize) {
    ffi::reserve_bool(v, size);
}

pub(crate) fn default() -> ffi::QVector_bool {
    ffi::qvector_default_bool()
}

pub(crate) fn drop(v: &mut ffi::QVector_bool) {
    ffi::qvector_drop_bool(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_bool, pos: isize) -> &bool {
    ffi::get_unchecked_bool(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_bool, value: &bool) -> isize {
    ffi::index_of_bool(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_bool, pos: isize, value: &bool) {
    ffi::insert_bool(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_bool) -> isize {
    ffi::len_bool(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_bool, pos: isize) {
    ffi::remove_bool(s, pos);
}
