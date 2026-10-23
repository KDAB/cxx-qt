// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Wehrwolfmann <256216494+wehrwolfmann@users.noreply.github.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Checks that an `#[allow(...)]` in a bridge reaches the generated code.
//!
//! The pedantic `unnecessary_box_returns` lint is enabled here, which clippy
//! reports on the function CXX generates to create the Rust struct of a QObject.
//! A signal with this many parameters makes clippy report `too_many_arguments`
//! on the generated signal and signal handler functions, and a signal that is
//! never emitted is reported as `dead_code`. If the attributes are not passed
//! on, `cargo clippy -- -D warnings` fails on this file.

#![warn(clippy::unnecessary_box_returns)]

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "RustQt" {
        #[qobject]
        #[allow(clippy::unnecessary_box_returns)]
        type LintsObject = super::LintsObjectRust;

        #[qsignal]
        #[allow(clippy::too_many_arguments, dead_code)]
        fn many_arguments(
            self: Pin<&mut Self>,
            first: i32,
            second: i32,
            third: i32,
            fourth: i32,
            fifth: i32,
            sixth: i32,
            seventh: i32,
            eighth: i32,
        );
    }
}

#[derive(Default)]
pub struct LintsObjectRust;
