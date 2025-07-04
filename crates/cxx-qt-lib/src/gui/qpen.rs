// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::fmt;
use std::mem::MaybeUninit;

use crate::{PenStyle, QColor};

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type PenStyle = crate::PenStyle;
        type PenCapStyle = crate::PenCapStyle;
        type PenJoinStyle = crate::PenJoinStyle;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qpen.h");
        type QPen = super::QPen;
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns the pen's cap style.
        #[rust_name = "cap_style"]
        fn capStyle(self: &QPen) -> PenCapStyle;

        /// Returns the color of this pen's brush.
        fn color(self: &QPen) -> QColor;

        /// Returns the dash offset for the pen.
        #[rust_name = "dash_offset"]
        fn dashOffset(self: &QPen) -> f64;

        /// Returns `true` if the pen is cosmetic; otherwise returns `false`.
        ///
        /// Cosmetic pens are used to draw strokes that have a constant width regardless of any transformations applied to the [`QPainter`](crate::QPainter) they are used with. Drawing a shape with a cosmetic pen ensures that its outline will have the same thickness at different scale factors.
        ///
        /// A zero width pen is cosmetic by default.
        #[rust_name = "is_comestic"]
        fn isCosmetic(self: &QPen) -> bool;

        /// Returns `true` if the pen has a solid fill, otherwise `false`.
        #[rust_name = "is_solid"]
        fn isSolid(self: &QPen) -> bool;

        /// Returns the pen's join style.
        #[rust_name = "join_style"]
        fn joinStyle(self: &QPen) -> PenJoinStyle;

        /// Returns the miter limit of the pen. The miter limit is only
        /// relevant when the join style is set to [`PenJoinStyle::MiterJoin`].
        #[rust_name = "miter_limit"]
        fn miterLimit(self: &QPen) -> f64;

        /// Sets the pen's cap style to the given `style`. The default value is [`PenCapStyle::SquareCap`].
        #[rust_name = "set_cap_style"]
        fn setCapStyle(self: &mut QPen, style: PenCapStyle);

        /// Sets the color of this pen's brush to the given `color`.
        #[rust_name = "set_color"]
        fn setColor(self: &mut QPen, color: &QColor);

        /// Sets this pen to cosmetic or non-cosmetic, depending on the value of `cosmetic`.
        #[rust_name = "set_cosmetic"]
        fn setCosmetic(self: &mut QPen, cosmetic: bool);

        /// Sets the dash offset (the starting point on the dash pattern) for this pen to
        /// the `offset` specified. The offset is measured in terms of the units used to
        /// specify the dash pattern.
        #[rust_name = "set_dash_offset"]
        fn setDashOffset(self: &mut QPen, offset: f64);

        /// Sets the pen's join style to the given style. The default value is [`PenJoinStyle::BevelJoin`].
        #[rust_name = "set_join_style"]
        fn setJoinStyle(self: &mut QPen, style: PenJoinStyle);

        /// Sets the pen style to the given `style`.
        #[rust_name = "set_style"]
        fn setStyle(self: &mut QPen, style: PenStyle);

        /// Sets the miter limit of this pen to the given `limit`.
        ///
        /// The miter limit describes how far a miter join can extend from the join point. This is used to reduce artifacts between line joins where the lines are close to parallel.
        ///
        /// This value does only have effect when the pen style is set to [`PenJoinStyle::MiterJoin`]. The value is specified in units of the pen's width, e.g. a miter limit of 5 in width 10 is 50 pixels long. The default miter limit is 2, i.e. twice the pen width in pixels.
        #[rust_name = "set_miter_limit"]
        fn setMiterLimit(self: &mut QPen, limit: f64);

        /// Sets the pen width to the given `width` in pixels with integer precision.
        ///
        /// A line width of zero indicates a cosmetic pen. This means that the pen width is always drawn one pixel wide, independent of the transformation set on the painter.
        ///
        /// Setting a pen width with a negative value is not supported.
        #[rust_name = "set_width"]
        fn setWidth(self: &mut QPen, width: i32);

        /// Returns the pen style.
        fn style(self: &QPen) -> PenStyle;

        /// Returns the pen width with integer precision.
        fn width(self: &QPen) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpen_init_default"]
        fn construct() -> QPen;

        #[doc(hidden)]
        #[rust_name = "qpen_init_from_qcolor"]
        fn construct(color: &QColor) -> QPen;

        #[doc(hidden)]
        #[rust_name = "qpen_init_from_penstyle"]
        fn construct(penstyle: &PenStyle) -> QPen;

        #[doc(hidden)]
        #[rust_name = "qpen_drop"]
        fn drop(pen: &mut QPen);

        #[doc(hidden)]
        #[rust_name = "qpen_clone"]
        fn construct(pen: &QPen) -> QPen;

        #[doc(hidden)]
        #[rust_name = "qpen_eq"]
        fn operatorEq(a: &QPen, b: &QPen) -> bool;

        #[doc(hidden)]
        #[rust_name = "qpen_to_debug_qstring"]
        fn toDebugQString(value: &QPen) -> QString;
    }
}

/// The `QPen` class defines how a [`QPainter`](crate::QPainter) should draw lines and outlines of shapes.
///
/// Qt Documentation: [QPen](https://doc.qt.io/qt/qpen.html#details)
#[repr(C)]
pub struct QPen {
    _cspec: MaybeUninit<usize>,
}

impl Default for QPen {
    /// Constructs a default black solid line pen with 1 width.
    fn default() -> Self {
        ffi::qpen_init_default()
    }
}

impl Drop for QPen {
    fn drop(&mut self) {
        ffi::qpen_drop(self);
    }
}

impl Clone for QPen {
    fn clone(&self) -> Self {
        ffi::qpen_clone(self)
    }
}

impl PartialEq for QPen {
    fn eq(&self, other: &Self) -> bool {
        ffi::qpen_eq(self, other)
    }
}

impl Eq for QPen {}

impl From<&QColor> for QPen {
    /// Constructs a solid line pen with 1 width and the given `color`.
    fn from(color: &QColor) -> Self {
        ffi::qpen_init_from_qcolor(color)
    }
}

impl From<&PenStyle> for QPen {
    /// Constructs a black pen with 1 width and the given `style`.
    fn from(style: &PenStyle) -> Self {
        ffi::qpen_init_from_penstyle(style)
    }
}

impl fmt::Display for QPen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qpen_to_debug_qstring(self).fmt(f)
    }
}

impl fmt::Debug for QPen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qpen_to_debug_qstring(self).fmt(f)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QPen {
    type Id = type_id!("QPen");
    type Kind = cxx::kind::Trivial;
}
