// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qsizef.h");
        include!("cxx-qt-lib/qstring.h");

        type QSizeF = super::QSizeF;
        type QString = crate::QString;

        /// Returns the height.
        fn height(self: &QSizeF) -> f64;
        /// Returns the width.
        fn width(self: &QSizeF) -> f64;

        /// Sets the height to the given height.
        #[rust_name = "set_height"]
        fn setHeight(self: &mut QSizeF, h: f64);
        /// Sets the width to the given width.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QSizeF, w: f64);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qsizef_init_default"]
        fn construct() -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qsizef_init"]
        fn construct(w: f64, h: f64) -> QSizeF;
        #[doc(hidden)]
        #[rust_name = "qsizef_to_qstring"]
        fn toQString(value: &QSizeF) -> QString;
    }
}

/// The QSizeF class defines the size of a two-dimensional object using floating point precision.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct QSizeF {
    w: f64,
    h: f64,
}

impl QSizeF {
    /// Constructs a size with the given width and height.
    pub fn new(w: f64, h: f64) -> Self {
        ffi::qsizef_init(w, h)
    }
}

impl Default for QSizeF {
    /// Constructs an invalid size.
    fn default() -> Self {
        ffi::qsizef_init_default()
    }
}

impl fmt::Display for QSizeF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qsizef_to_qstring(self))
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QSizeF is trivial.
unsafe impl ExternType for QSizeF {
    type Id = type_id!("QSizeF");
    type Kind = cxx::kind::Trivial;
}
