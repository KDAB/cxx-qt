// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmargins.h");
        type QMargins = crate::QMargins;
        include!("cxx-qt-lib/qmarginsf.h");
        type QMarginsF = super::QMarginsF;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns the bottom margin.
        fn bottom(self: &QMarginsF) -> f64;

        /// Returns true if all margins are very close to 0; otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QMarginsF) -> bool;

        /// Returns the left margin.
        fn left(self: &QMarginsF) -> f64;

        /// Returns the right margin.
        fn right(self: &QMarginsF) -> f64;

        /// Sets the bottom margin to abottom (which must be finite).
        #[rust_name = "set_bottom"]
        fn setBottom(self: &mut QMarginsF, bottom: f64);

        /// Sets the left margin to aleft (which must be finite).
        #[rust_name = "set_left"]
        fn setLeft(self: &mut QMarginsF, left: f64);

        /// Sets the right margin to aright (which must be finite).
        #[rust_name = "set_right"]
        fn setRight(self: &mut QMarginsF, right: f64);

        /// Sets the top margin to atop (which must be finite).
        #[rust_name = "set_top"]
        fn setTop(self: &mut QMarginsF, top: f64);

        /// Returns an integer-based copy of this margins object.
        ///
        /// Note that the components in the returned margins will be rounded to the nearest integer.
        #[rust_name = "to_margins"]
        fn toMargins(self: &QMarginsF) -> QMargins;

        /// Returns the top margin.
        fn top(self: &QMarginsF) -> f64;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qmarginsf_default"]
        fn construct() -> QMarginsF;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_from_qmargin"]
        fn construct(margins: &QMargins) -> QMarginsF;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_new"]
        fn construct(left: f64, top: f64, right: f64, bottom: f64) -> QMarginsF;
        #[doc(hidden)]
        #[rust_name = "qmarginsf_to_qstring"]
        fn toQString(value: &QMarginsF) -> QString;
    }
}

/// The QMarginsF class defines the four margins of a rectangle.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QMarginsF {
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
}

impl QMarginsF {
    /// Constructs margins with the given left, top, right, and bottom. All parameters must be finite.
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        ffi::qmarginsf_new(left, top, right, bottom)
    }
}

impl Default for QMarginsF {
    /// Constructs a margins object with all margins set to 0.
    fn default() -> Self {
        ffi::qmarginsf_default()
    }
}

impl fmt::Display for QMarginsF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qmarginsf_to_qstring(self))
    }
}

impl From<&ffi::QMargins> for QMarginsF {
    /// Constructs margins copied from the given margins.
    fn from(margins: &ffi::QMargins) -> Self {
        ffi::qmarginsf_from_qmargin(margins)
    }
}

impl From<&QMarginsF> for ffi::QMargins {
    /// Returns an integer-based copy of this margins object.
    ///
    /// Note that the components in the returned margins will be rounded to the nearest integer.
    fn from(value: &QMarginsF) -> Self {
        value.to_margins()
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QMarginsF is trivial.
unsafe impl ExternType for QMarginsF {
    type Id = type_id!("QMarginsF");
    type Kind = cxx::kind::Trivial;
}
