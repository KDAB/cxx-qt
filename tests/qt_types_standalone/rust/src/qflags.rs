// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{MouseButton, MouseButtons};

#[cxx::bridge]
mod qmargins_cxx {
    unsafe extern "C++" {
        include!(<QtCore/QtCore>);
        #[namespace = "Qt"]
        type MouseButtons = cxx_qt_lib::MouseButtons;
    }

    extern "Rust" {
        fn construct_qflags() -> MouseButtons;
        fn read_qflags(f: &MouseButtons) -> bool;
        fn clone_qflags(f: &MouseButtons) -> MouseButtons;
        fn test_is_empty(f: &MouseButtons) -> bool;
        fn add_flags(f1: MouseButtons, f2: MouseButtons) -> MouseButtons;
    }
}

fn construct_qflags() -> MouseButtons {
    MouseButton::ForwardButton | MouseButton::LeftButton
}

fn read_qflags(f: &MouseButtons) -> bool {
    f.to_int() == MouseButton::ForwardButton.repr | MouseButton::LeftButton.repr
}

fn clone_qmargins(f: &MouseButtons) -> QMargins {
    f.clone()
}

fn test_is_empty(f: &MouseButtons) -> bool {
    f.is_empty()
}

fn add_flags(f1: MouseButtons, f2: MouseButtons) -> MouseButtons {
    f1 | f2
}
