// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;

        include!("cxx-qt-lib/qt5list.h");
        type Qt5List_QRect = crate::Qt5List<QRect>;
    }

    unsafe extern "C++" {
        /// # Safety
        ///
        /// Calling this method with an out-of-bounds index is undefined behavior
        /// even if the resulting reference is not used.
        #[rust_name = "cxx_get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn at<'a>(self: &'a Qt5List_QRect, pos: i32) -> &'a QRect;
        #[rust_name = "cxx_append"]
        fn append(self: &mut Qt5List_QRect, _: &QRect);
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut Qt5List_QRect);
        #[rust_name = "cxx_contains"]
        fn contains(self: &Qt5List_QRect, _: &QRect) -> bool;
        #[rust_name = "cxx_index_of"]
        fn indexOf(self: &Qt5List_QRect, _: &QRect, from: i32) -> i32;
        #[rust_name = "cxx_insert"]
        fn insert(self: &mut Qt5List_QRect, _: i32, _: &QRect);
        #[rust_name = "cxx_len"]
        fn length(self: &Qt5List_QRect) -> i32;
        #[rust_name = "cxx_remove"]
        fn removeAt(self: &mut Qt5List_QRect, _: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qt5list_clone_QRect"]
        fn construct(_: &Qt5List_QRect) -> Qt5List_QRect;
        #[rust_name = "qt5list_default_QRect"]
        fn construct() -> Qt5List_QRect;
        #[rust_name = "qt5list_drop_QRect"]
        fn drop(_: &mut Qt5List_QRect);
    }
}

pub(crate) fn clone(s: &ffi::Qt5List_QRect) -> ffi::Qt5List_QRect {
    ffi::qt5list_clone_QRect(s)
}

pub(crate) fn default() -> ffi::Qt5List_QRect {
    ffi::qt5list_default_QRect()
}

pub(crate) fn drop(s: &mut ffi::Qt5List_QRect) {
    ffi::qt5list_drop_QRect(s);
}
