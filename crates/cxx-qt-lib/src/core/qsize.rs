// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qsize.h");
        include!("cxx-qt-lib/qstring.h");

        type QSize = super::QSize;
        type QString = crate::QString;

        /// Returns the height.
        fn height(self: &QSize) -> i32;
        /// Returns the width.
        fn width(self: &QSize) -> i32;

        /// Sets the height to the given height.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QSize, h: i32);
        /// Sets the width to the given width.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QSize, w: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qsize_init_default"]
        fn construct() -> QSize;
        #[doc(hidden)]
        #[rust_name = "qsize_init"]
        fn construct(w: i32, h: i32) -> QSize;
        #[doc(hidden)]
        #[rust_name = "qsize_to_qstring"]
        fn toQString(value: &QSize) -> QString;
    }
}

/// The QSize struct defines the size of a two-dimensional object using integer point precision.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct QSize {
    w: i32,
    h: i32,
}

impl QSize {
    /// Constructs a size with the given width and height.
    pub fn new(width: i32, height: i32) -> Self {
        ffi::qsize_init(width, height)
    }
}

impl Default for QSize {
    /// Constructs a size with an invalid width and height
    fn default() -> Self {
        ffi::qsize_init_default()
    }
}

impl fmt::Display for QSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qsize_to_qstring(self))
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QSize is trivial.
unsafe impl ExternType for QSize {
    type Id = type_id!("QSize");
    type Kind = cxx::kind::Trivial;
}
