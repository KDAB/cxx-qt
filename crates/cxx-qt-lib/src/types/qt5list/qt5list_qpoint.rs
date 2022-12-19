// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;

        include!("cxx-qt-lib/qt5list.h");
        type Qt5List_QPoint = crate::Qt5List<QPoint>;
    }

    unsafe extern "C++" {
        /// # Safety
        ///
        /// Calling this method with an out-of-bounds index is undefined behavior
        /// even if the resulting reference is not used.
        #[rust_name = "cxx_get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn at<'a>(self: &'a Qt5List_QPoint, pos: i32) -> &'a QPoint;
        #[rust_name = "cxx_append"]
        fn append(self: &mut Qt5List_QPoint, _: &QPoint);
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut Qt5List_QPoint);
        #[rust_name = "cxx_contains"]
        fn contains(self: &Qt5List_QPoint, _: &QPoint) -> bool;
        #[rust_name = "cxx_index_of"]
        fn indexOf(self: &Qt5List_QPoint, _: &QPoint, from: i32) -> i32;
        #[rust_name = "cxx_insert"]
        fn insert(self: &mut Qt5List_QPoint, _: i32, _: &QPoint);
        #[rust_name = "cxx_len"]
        fn length(self: &Qt5List_QPoint) -> i32;
        #[rust_name = "cxx_remove"]
        fn removeAt(self: &mut Qt5List_QPoint, _: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qt5list_clone_QPoint"]
        fn construct(_: &Qt5List_QPoint) -> Qt5List_QPoint;
        #[rust_name = "qt5list_default_QPoint"]
        fn construct() -> Qt5List_QPoint;
        #[rust_name = "qt5list_drop_QPoint"]
        fn drop(_: &mut Qt5List_QPoint);
    }
}

pub(crate) fn clone(s: &ffi::Qt5List_QPoint) -> ffi::Qt5List_QPoint {
    ffi::qt5list_clone_QPoint(s)
}

pub(crate) fn default() -> ffi::Qt5List_QPoint {
    ffi::qt5list_default_QPoint()
}

pub(crate) fn drop(s: &mut ffi::Qt5List_QPoint) {
    ffi::qt5list_drop_QPoint(s);
}
