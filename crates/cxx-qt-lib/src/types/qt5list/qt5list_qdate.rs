// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdate.h");
        type QDate = crate::QDate;

        include!("cxx-qt-lib/qt5list.h");
        type Qt5List_QDate = crate::Qt5List<QDate>;
    }

    unsafe extern "C++" {
        /// # Safety
        ///
        /// Calling this method with an out-of-bounds index is undefined behavior
        /// even if the resulting reference is not used.
        #[rust_name = "cxx_get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn at<'a>(self: &'a Qt5List_QDate, pos: i32) -> &'a QDate;
        #[rust_name = "cxx_append"]
        fn append(self: &mut Qt5List_QDate, _: &QDate);
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut Qt5List_QDate);
        #[rust_name = "cxx_contains"]
        fn contains(self: &Qt5List_QDate, _: &QDate) -> bool;
        #[rust_name = "cxx_index_of"]
        fn indexOf(self: &Qt5List_QDate, _: &QDate, from: i32) -> i32;
        #[rust_name = "cxx_insert"]
        fn insert(self: &mut Qt5List_QDate, _: i32, _: &QDate);
        #[rust_name = "cxx_len"]
        fn length(self: &Qt5List_QDate) -> i32;
        #[rust_name = "cxx_remove"]
        fn removeAt(self: &mut Qt5List_QDate, _: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qt5list_clone_QDate"]
        fn construct(_: &Qt5List_QDate) -> Qt5List_QDate;
        #[rust_name = "qt5list_default_QDate"]
        fn construct() -> Qt5List_QDate;
        #[rust_name = "qt5list_drop_QDate"]
        fn drop(_: &mut Qt5List_QDate);
    }
}

pub(crate) fn clone(s: &ffi::Qt5List_QDate) -> ffi::Qt5List_QDate {
    ffi::qt5list_clone_QDate(s)
}

pub(crate) fn default() -> ffi::Qt5List_QDate {
    ffi::qt5list_default_QDate()
}

pub(crate) fn drop(s: &mut ffi::Qt5List_QDate) {
    ffi::qt5list_drop_QDate(s);
}
