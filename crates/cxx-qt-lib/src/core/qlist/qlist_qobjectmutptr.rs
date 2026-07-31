// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qobjectmutptr.h");
        type QObjectMutPtr = crate::QObjectMutPtr;

        include!("cxx-qt-lib/core/qlist/qlist_QObjectMutPtr.h");
        type QList_QObjectMutPtr = crate::QList<QObjectMutPtr>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_QObjectMutPtr);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QObjectMutPtr, _: &QObjectMutPtr) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QObjectMutPtr"]
        fn construct(_: &QList_QObjectMutPtr) -> QList_QObjectMutPtr;
        #[rust_name = "qlist_default_QObjectMutPtr"]
        fn construct() -> QList_QObjectMutPtr;
        #[rust_name = "qlist_drop_QObjectMutPtr"]
        fn drop(_: &mut QList_QObjectMutPtr);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QObjectMutPtr"]
        fn qlistReserve(_: &mut QList_QObjectMutPtr, size: isize);
        #[rust_name = "append_QObjectMutPtr"]
        fn qlistAppend(_: &mut QList_QObjectMutPtr, _: &QObjectMutPtr);
        #[rust_name = "get_unchecked_QObjectMutPtr"]
        unsafe fn qlistGetUnchecked(set: &QList_QObjectMutPtr, pos: isize) -> &QObjectMutPtr;
        #[rust_name = "index_of_QObjectMutPtr"]
        fn qlistIndexOf(_: &QList_QObjectMutPtr, _: &QObjectMutPtr) -> isize;
        #[rust_name = "insert_QObjectMutPtr"]
        fn qlistInsert(_: &mut QList_QObjectMutPtr, _: isize, _: &QObjectMutPtr);
        #[rust_name = "remove_QObjectMutPtr"]
        fn qlistRemove(_: &mut QList_QObjectMutPtr, _: isize);
        #[rust_name = "len_QObjectMutPtr"]
        fn qlistLen(_: &QList_QObjectMutPtr) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_QObjectMutPtr, size: isize) {
    ffi::reserve_QObjectMutPtr(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QObjectMutPtr, value: &ffi::QObjectMutPtr) {
    ffi::append_QObjectMutPtr(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QObjectMutPtr) -> ffi::QList_QObjectMutPtr {
    ffi::qlist_clone_QObjectMutPtr(s)
}

pub(crate) fn default() -> ffi::QList_QObjectMutPtr {
    ffi::qlist_default_QObjectMutPtr()
}

pub(crate) fn drop(s: &mut ffi::QList_QObjectMutPtr) {
    ffi::qlist_drop_QObjectMutPtr(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QObjectMutPtr,
    pos: isize,
) -> &ffi::QObjectMutPtr {
    ffi::get_unchecked_QObjectMutPtr(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QObjectMutPtr, value: &ffi::QObjectMutPtr) -> isize {
    ffi::index_of_QObjectMutPtr(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QObjectMutPtr, pos: isize, value: &ffi::QObjectMutPtr) {
    ffi::insert_QObjectMutPtr(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QObjectMutPtr) -> isize {
    ffi::len_QObjectMutPtr(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QObjectMutPtr, pos: isize) {
    ffi::remove_QObjectMutPtr(s, pos);
}
