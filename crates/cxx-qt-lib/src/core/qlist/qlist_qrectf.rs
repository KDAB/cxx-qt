// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = crate::QRectF;

        include!("cxx-qt-lib/qlist.h");
        type QList_QRectF = crate::QList<QRectF>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QRectF);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QRectF, _: &QRectF) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QRectF"]
        fn construct(_: &QList_QRectF) -> QList_QRectF;
        #[rust_name = "qlist_default_QRectF"]
        fn construct() -> QList_QRectF;
        #[rust_name = "qlist_drop_QRectF"]
        fn drop(_: &mut QList_QRectF);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QRectF"]
        fn qlistReserve(_: &mut QList_QRectF, size: isize);
        #[rust_name = "append_QRectF"]
        fn qlistAppend(_: &mut QList_QRectF, _: &QRectF);
        #[rust_name = "get_unchecked_QRectF"]
        unsafe fn qlistGetUnchecked(set: &QList_QRectF, pos: isize) -> &QRectF;
        #[rust_name = "index_of_QRectF"]
        fn qlistIndexOf(_: &QList_QRectF, _: &QRectF) -> isize;
        #[rust_name = "insert_QRectF"]
        fn qlistInsert(_: &mut QList_QRectF, _: isize, _: &QRectF);
        #[rust_name = "remove_QRectF"]
        fn qlistRemove(_: &mut QList_QRectF, _: isize);
        #[rust_name = "len_QRectF"]
        fn qlistLen(_: &QList_QRectF) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QRectF, size: isize) {
    ffi::reserve_QRectF(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QRectF, value: &ffi::QRectF) {
    ffi::append_QRectF(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QRectF) -> ffi::QList_QRectF {
    ffi::qlist_clone_QRectF(s)
}

pub(crate) fn default() -> ffi::QList_QRectF {
    ffi::qlist_default_QRectF()
}

pub(crate) fn drop(s: &mut ffi::QList_QRectF) {
    ffi::qlist_drop_QRectF(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QRectF, pos: isize) -> &ffi::QRectF {
    ffi::get_unchecked_QRectF(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QRectF, value: &ffi::QRectF) -> isize {
    ffi::index_of_QRectF(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QRectF, pos: isize, value: &ffi::QRectF) {
    ffi::insert_QRectF(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QRectF) -> isize {
    ffi::len_QRectF(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QRectF, pos: isize) {
    ffi::remove_QRectF(s, pos);
}
