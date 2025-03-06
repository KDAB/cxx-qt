// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qline.h");
        type QLine = crate::QLine;

        include!("cxx-qt-lib/qlist_QLine.h");
        type QList_QLine = crate::QList<QLine>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QLine);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QLine, _: &QLine) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QLine"]
        fn construct(_: &QList_QLine) -> QList_QLine;
        #[rust_name = "qlist_default_QLine"]
        fn construct() -> QList_QLine;
        #[rust_name = "qlist_drop_QLine"]
        fn drop(_: &mut QList_QLine);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QLine"]
        fn qlistReserve(_: &mut QList_QLine, size: isize);
        #[rust_name = "append_QLine"]
        fn qlistAppend(_: &mut QList_QLine, _: &QLine);
        #[rust_name = "get_unchecked_QLine"]
        unsafe fn qlistGetUnchecked(set: &QList_QLine, pos: isize) -> &QLine;
        #[rust_name = "index_of_QLine"]
        fn qlistIndexOf(_: &QList_QLine, _: &QLine) -> isize;
        #[rust_name = "insert_QLine"]
        fn qlistInsert(_: &mut QList_QLine, _: isize, _: &QLine);
        #[rust_name = "remove_QLine"]
        fn qlistRemove(_: &mut QList_QLine, _: isize);
        #[rust_name = "len_QLine"]
        fn qlistLen(_: &QList_QLine) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QLine, size: isize) {
    ffi::reserve_QLine(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QLine, value: &ffi::QLine) {
    ffi::append_QLine(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QLine) -> ffi::QList_QLine {
    ffi::qlist_clone_QLine(s)
}

pub(crate) fn default() -> ffi::QList_QLine {
    ffi::qlist_default_QLine()
}

pub(crate) fn drop(s: &mut ffi::QList_QLine) {
    ffi::qlist_drop_QLine(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QLine, pos: isize) -> &ffi::QLine {
    ffi::get_unchecked_QLine(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QLine, value: &ffi::QLine) -> isize {
    ffi::index_of_QLine(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QLine, pos: isize, value: &ffi::QLine) {
    ffi::insert_QLine(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QLine) -> isize {
    ffi::len_QLine(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QLine, pos: isize) {
    ffi::remove_QLine(s, pos);
}
