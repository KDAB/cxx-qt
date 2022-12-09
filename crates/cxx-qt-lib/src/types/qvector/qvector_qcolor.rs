// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;

        include!("cxx-qt-lib/qvector.h");
        type QVector_QColor = crate::QVector<QColor>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QColor);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QColor, _: &QColor) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QColor"]
        fn construct(_: &QVector_QColor) -> QVector_QColor;
        #[rust_name = "qvector_default_QColor"]
        fn construct() -> QVector_QColor;
        #[rust_name = "qvector_drop_QColor"]
        fn drop(_: &mut QVector_QColor);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "append_QColor"]
        fn qvectorAppend(_: &mut QVector_QColor, _: &QColor);
        #[rust_name = "get_unchecked_QColor"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QColor, pos: isize) -> &QColor;
        #[rust_name = "index_of_QColor"]
        fn qvectorIndexOf(_: &QVector_QColor, _: &QColor) -> isize;
        #[rust_name = "insert_QColor"]
        fn qvectorInsert(_: &mut QVector_QColor, _: isize, _: &QColor);
        #[rust_name = "remove_QColor"]
        fn qvectorRemove(_: &mut QVector_QColor, _: isize);
        #[rust_name = "len_QColor"]
        fn qvectorLen(_: &QVector_QColor) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QColor, value: &ffi::QColor) {
    ffi::append_QColor(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QColor) -> ffi::QVector_QColor {
    ffi::qvector_clone_QColor(s)
}

pub(crate) fn default() -> ffi::QVector_QColor {
    ffi::qvector_default_QColor()
}

pub(crate) fn drop(s: &mut ffi::QVector_QColor) {
    ffi::qvector_drop_QColor(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QColor, pos: isize) -> &ffi::QColor {
    ffi::get_unchecked_QColor(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QColor, value: &ffi::QColor) -> isize {
    ffi::index_of_QColor(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QColor, pos: isize, value: &ffi::QColor) {
    ffi::insert_QColor(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QColor) -> isize {
    ffi::len_QColor(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QColor, pos: isize) {
    ffi::remove_QColor(s, pos);
}
