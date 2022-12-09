// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector.h");
        type QVector_u8 = crate::QVector<u8>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_u8);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_u8, _: &u8) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_u8"]
        fn construct(_: &QVector_u8) -> QVector_u8;
        #[rust_name = "qvector_default_u8"]
        fn construct() -> QVector_u8;
        #[rust_name = "qvector_drop_u8"]
        fn drop(_: &mut QVector_u8);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "append_u8"]
        fn qvectorAppend(_: &mut QVector_u8, _: &u8);
        #[rust_name = "get_unchecked_u8"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_u8, pos: isize) -> &'a u8;
        #[rust_name = "index_of_u8"]
        fn qvectorIndexOf(_: &QVector_u8, _: &u8) -> isize;
        #[rust_name = "insert_u8"]
        fn qvectorInsert(_: &mut QVector_u8, _: isize, _: &u8);
        #[rust_name = "len_u8"]
        fn qvectorLen(_: &QVector_u8) -> isize;
        #[rust_name = "remove_u8"]
        fn qvectorRemove(_: &mut QVector_u8, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_u8, value: &u8) {
    ffi::append_u8(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_u8) -> ffi::QVector_u8 {
    ffi::qvector_clone_u8(v)
}

pub(crate) fn default() -> ffi::QVector_u8 {
    ffi::qvector_default_u8()
}

pub(crate) fn drop(v: &mut ffi::QVector_u8) {
    ffi::qvector_drop_u8(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_u8, pos: isize) -> &u8 {
    ffi::get_unchecked_u8(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_u8, value: &u8) -> isize {
    ffi::index_of_u8(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_u8, pos: isize, value: &u8) {
    ffi::insert_u8(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_u8) -> isize {
    ffi::len_u8(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_u8, pos: isize) {
    ffi::remove_u8(s, pos);
}
