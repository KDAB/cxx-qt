// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpixmap.h");
        type QPixmap;
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;

        fn fill(self: Pin<&mut QPixmap>, color: &QColor);

        fn setDevicePixelRatio(self: Pin<&mut QPixmap>, ratio: f64);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[rust_name = "qpixmap_new"]
        fn qpixmapNew(width: i32, height: i32) -> UniquePtr<QPixmap>;
    }
}

pub use ffi::{QPixmap};

impl QPixmap {
    pub fn new(width: i32, height: i32) -> cxx::UniquePtr<Self> {
        ffi::qpixmap_new(width, height)
    }
}