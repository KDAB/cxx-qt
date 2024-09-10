// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/qquickstyle.h");
        #[qobject]
        type QQuickStyle;

        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qquickstyle_name"]
        fn qquickstyleName() -> QString;

        #[doc(hidden)]
        #[rust_name = "qquickstyle_set_fallback_style"]
        fn qquickstyleSetFallbackStyle(style: &QString);

        #[doc(hidden)]
        #[rust_name = "qquickstyle_set_style"]
        fn qquickstyleSetStyle(style: &QString);
    }
}

use crate::QString;
pub use ffi::QQuickStyle;

impl QQuickStyle {
    pub fn name() -> QString {
        ffi::qquickstyle_name()
    }

    pub fn set_fallback_style(style: &QString) {
        ffi::qquickstyle_set_fallback_style(style)
    }

    pub fn set_style(style: &QString) {
        ffi::qquickstyle_set_style(style)
    }
}
