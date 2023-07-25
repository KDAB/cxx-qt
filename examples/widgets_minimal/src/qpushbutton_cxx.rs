// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge(cxx_file_stem = "qpushbutton_cxx")]
mod ffi {

    unsafe extern "C++Qt" {
        include!(<QtWidgets/QPushButton>);
        type QPushButton;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        #[qsignal]
        #[allow(dead_code)]
        fn clicked(self: Pin<&mut QPushButton>, checked: bool);

        #[rust_name = "set_text"]
        fn setText(self: Pin<&mut QPushButton>, text: &QString);

        fn show(self: Pin<&mut QPushButton>);
    }

    unsafe extern "C++Qt" {
        include!("qpushbutton.h");

        #[doc(hidden)]
        #[cxx_name = "qpushbuttonNew"]
        fn qpushbutton_new() -> UniquePtr<QPushButton>;
    }
}

impl ffi::QPushButton {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qpushbutton_new()
    }
}

pub use ffi::QPushButton;
