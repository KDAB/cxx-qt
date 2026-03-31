// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Darshan Phaldesai <dev.darshanphaldesai@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge]
mod ffi {

    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QQmlImageProviderBaseImageType {
        Invalid = 0,
        Image,
        Pixmap,
        Texture,
        ImageResponse,
    }

    extern "C++Qt" {
        include!("cxx-qt-lib/qqmlimageproviderbase.h");

        #[qobject]
        type QQmlImageProviderBase;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        type QQmlImageProviderBaseImageType;
    }
}

pub use ffi::QQmlImageProviderBase;
pub use ffi::QQmlImageProviderBaseImageType;
