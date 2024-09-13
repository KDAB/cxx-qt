// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qline.h");
        type QLine = crate::QLine;

        include!("cxx-qt-lib/qvector.h");
        type QVector_QLine = crate::QVector<QLine>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_QLine);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_QLine, _: &QLine) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_QLine"]
        fn construct(_: &QVector_QLine) -> QVector_QLine;
        #[rust_name = "qvector_default_QLine"]
        fn construct() -> QVector_QLine;
        #[rust_name = "qvector_drop_QLine"]
        fn drop(_: &mut QVector_QLine);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QLine"]
        fn qvectorReserve(_: &mut QVector_QLine, size: isize);
        #[rust_name = "append_QLine"]
        fn qvectorAppend(_: &mut QVector_QLine, _: &QLine);
        #[rust_name = "get_unchecked_QLine"]
        unsafe fn qvectorGetUnchecked(set: &QVector_QLine, pos: isize) -> &QLine;
        #[rust_name = "index_of_QLine"]
        fn qvectorIndexOf(_: &QVector_QLine, _: &QLine) -> isize;
        #[rust_name = "insert_QLine"]
        fn qvectorInsert(_: &mut QVector_QLine, _: isize, _: &QLine);
        #[rust_name = "remove_QLine"]
        fn qvectorRemove(_: &mut QVector_QLine, _: isize);
        #[rust_name = "len_QLine"]
        fn qvectorLen(_: &QVector_QLine) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_QLine, value: &ffi::QLine) {
    ffi::append_QLine(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_QLine) -> ffi::QVector_QLine {
    ffi::qvector_clone_QLine(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_QLine, size: isize) {
    ffi::reserve_QLine(v, size);
}

pub(crate) fn default() -> ffi::QVector_QLine {
    ffi::qvector_default_QLine()
}

pub(crate) fn drop(s: &mut ffi::QVector_QLine) {
    ffi::qvector_drop_QLine(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_QLine, pos: isize) -> &ffi::QLine {
    ffi::get_unchecked_QLine(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_QLine, value: &ffi::QLine) -> isize {
    ffi::index_of_QLine(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_QLine, pos: isize, value: &ffi::QLine) {
    ffi::insert_QLine(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_QLine) -> isize {
    ffi::len_QLine(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_QLine, pos: isize) {
    ffi::remove_QLine(s, pos);
}
