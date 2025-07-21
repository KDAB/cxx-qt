// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qquickstyle.h");
        /// The `QQuickStyle` class allows configuring the application style.
        ///
        /// Qt Documentation: [QQuickStyle](https://doc.qt.io/qt/qquickstyle.html#details)
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
    /// Returns the name of the application style.
    ///
    /// **Note:** The application style can be specified by passing a `-style` command line argument. Therefore this function may not return a fully resolved value if called before constructing a [`QGuiApplication`](crate::QGuiApplication).
    pub fn name() -> QString {
        ffi::qquickstyle_name()
    }

    /// Sets the application fallback style to `style`.
    ///
    /// **Note:** The fallback style must be the name of one of the built-in Qt Quick Controls styles, e.g. "Material".
    ///
    /// **Note:** The style must be configured before loading QML that imports Qt Quick Controls. It is not possible to change the style after the QML types have been registered.
    ///
    /// The fallback style can be also specified by setting the `QT_QUICK_CONTROLS_FALLBACK_STYLE` [environment variable](https://doc.qt.io/qt/qtquickcontrols-environment.html).
    pub fn set_fallback_style(style: &QString) {
        ffi::qquickstyle_set_fallback_style(style)
    }

    /// Sets the application style to `style`.
    ///
    /// Note: The style must be configured before loading QML that imports Qt Quick Controls. It is not possible to change the style after the QML types have been registered.
    pub fn set_style(style: &QString) {
        ffi::qquickstyle_set_style(style)
    }
}
