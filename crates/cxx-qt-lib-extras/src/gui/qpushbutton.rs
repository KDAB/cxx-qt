// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++Qt" {
        include!("cxx-qt-lib-extras/qpushbutton.h");

        #[qobject]
        type QPushButton;

        // TODO: we should use upcasting methods here and implement QAbstractButton and QWidget
        // so that we don't need to duplicate all of the methods

        /// This signal is emitted when the button is activated (i.e., pressed down then released while the mouse cursor is inside the button)
        #[qsignal]
        fn clicked(self: Pin<&mut QPushButton>, checked: bool);

        /// Set the text shown on the button
        #[rust_name = "set_text"]
        fn setText(self: Pin<&mut QPushButton>, text: &QString);

        /// Shows the widget and its child widgets.
        fn show(self: Pin<&mut QPushButton>);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpushbutton_init_default"]
        fn make_unique() -> UniquePtr<QPushButton>;
    }
}

impl ffi::QPushButton {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qpushbutton_init_default()
    }
}

pub use ffi::QPushButton;
