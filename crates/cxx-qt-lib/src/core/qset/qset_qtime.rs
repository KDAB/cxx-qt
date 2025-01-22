// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qtime.h");
        type QTime = crate::QTime;

        include!("cxx-qt-lib/qset.h");
        type QSet_QTime = crate::QSet<QTime>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_QTime);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_QTime, _: &QTime) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_QTime, _: &QTime) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QTime"]
        fn construct(_: &QSet_QTime) -> QSet_QTime;
        #[rust_name = "qset_default_QTime"]
        fn construct() -> QSet_QTime;
        #[rust_name = "qset_drop_QTime"]
        fn drop(_: &mut QSet_QTime);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_QTime"]
        unsafe fn qsetGetUnchecked(set: &QSet_QTime, pos: isize) -> &QTime;
        #[rust_name = "insert_QTime"]
        fn qsetInsert(_: &mut QSet_QTime, _: &QTime);
        #[rust_name = "len_QTime"]
        fn qsetLen(_: &QSet_QTime) -> isize;
        #[rust_name = "reserve_QTime"]
        fn qsetReserve(_: &mut QSet_QTime, size: isize);
    }
}

pub(crate) fn clone(s: &ffi::QSet_QTime) -> ffi::QSet_QTime {
    ffi::qset_clone_QTime(s)
}

pub(crate) fn default() -> ffi::QSet_QTime {
    ffi::qset_default_QTime()
}

pub(crate) fn drop(s: &mut ffi::QSet_QTime) {
    ffi::qset_drop_QTime(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_QTime, pos: isize) -> &ffi::QTime {
    ffi::get_unchecked_QTime(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QTime, value: &ffi::QTime) {
    ffi::insert_QTime(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QTime) -> isize {
    ffi::len_QTime(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_QTime, size: isize) {
    ffi::reserve_QTime(s, size);
}
