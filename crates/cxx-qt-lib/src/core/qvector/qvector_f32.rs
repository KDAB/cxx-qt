// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector.h");
        type QVector_f32 = crate::QVector<f32>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_f32);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_f32, _: &f32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_f32"]
        fn construct(_: &QVector_f32) -> QVector_f32;
        #[rust_name = "qvector_default_f32"]
        fn construct() -> QVector_f32;
        #[rust_name = "qvector_drop_f32"]
        fn drop(_: &mut QVector_f32);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_f32"]
        fn qvectorReserve(_: &mut QVector_f32, size: isize);
        #[rust_name = "append_f32"]
        fn qvectorAppend(_: &mut QVector_f32, _: &f32);
        #[rust_name = "get_unchecked_f32"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_f32, pos: isize) -> &'a f32;
        #[rust_name = "index_of_f32"]
        fn qvectorIndexOf(_: &QVector_f32, _: &f32) -> isize;
        #[rust_name = "insert_f32"]
        fn qvectorInsert(_: &mut QVector_f32, _: isize, _: &f32);
        #[rust_name = "len_f32"]
        fn qvectorLen(_: &QVector_f32) -> isize;
        #[rust_name = "remove_f32"]
        fn qvectorRemove(_: &mut QVector_f32, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_f32, value: &f32) {
    ffi::append_f32(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_f32) -> ffi::QVector_f32 {
    ffi::qvector_clone_f32(v)
}

pub(crate) fn reserve(v: &mut ffi::QVector_f32, size: isize) {
    ffi::reserve_f32(v, size);
}

pub(crate) fn default() -> ffi::QVector_f32 {
    ffi::qvector_default_f32()
}

pub(crate) fn drop(v: &mut ffi::QVector_f32) {
    ffi::qvector_drop_f32(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_f32, pos: isize) -> &f32 {
    ffi::get_unchecked_f32(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_f32, value: &f32) -> isize {
    ffi::index_of_f32(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_f32, pos: isize, value: &f32) {
    ffi::insert_f32(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_f32) -> isize {
    ffi::len_f32(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_f32, pos: isize) {
    ffi::remove_f32(s, pos);
}
