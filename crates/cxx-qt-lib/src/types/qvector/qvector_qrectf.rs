// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = crate::QRectF;

        include!("cxx-qt-lib/qvector.h");
        type QVector_QRectF = crate::QVector<QRectF>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QRectF);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QRectF, _: &QRectF) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QRectF"]
        fn construct(_: &QVector_QRectF) -> QVector_QRectF;
        #[rust_name = "qvector_default_QRectF"]
        fn construct() -> QVector_QRectF;
        #[rust_name = "qvector_drop_QRectF"]
        fn drop(_: &mut QVector_QRectF);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QRectF"]
        fn qvectorReserve(_: &mut QVector_QRectF, size: isize);
        #[rust_name = "append_QRectF"]
        fn qvectorAppend(_: &mut QVector_QRectF, _: &QRectF);
        #[rust_name = "get_unchecked_QRectF"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QRectF, pos: isize) -> &QRectF;
        #[rust_name = "index_of_QRectF"]
        fn qvectorIndexOf(_: &QVector_QRectF, _: &QRectF) -> isize;
        #[rust_name = "insert_QRectF"]
        fn qvectorInsert(_: &mut QVector_QRectF, _: isize, _: &QRectF);
        #[rust_name = "remove_QRectF"]
        fn qvectorRemove(_: &mut QVector_QRectF, _: isize);
        #[rust_name = "len_QRectF"]
        fn qvectorLen(_: &QVector_QRectF) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QRectF, value: &ffi::QRectF) {
    ffi::append_QRectF(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QRectF) -> ffi::QVector_QRectF {
    ffi::qvector_clone_QRectF(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QRectF, size: isize) {
    ffi::reserve_QRectF(v, size);
}

pub(crate) fn default() -> ffi::QVector_QRectF {
    ffi::qvector_default_QRectF()
}

pub(crate) fn drop(s: &mut ffi::QVector_QRectF) {
    ffi::qvector_drop_QRectF(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QRectF, pos: isize) -> &ffi::QRectF {
    ffi::get_unchecked_QRectF(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QRectF, value: &ffi::QRectF) -> isize {
    ffi::index_of_QRectF(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QRectF, pos: isize, value: &ffi::QRectF) {
    ffi::insert_QRectF(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QRectF) -> isize {
    ffi::len_QRectF(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QRectF, pos: isize) {
    ffi::remove_QRectF(s, pos);
}
