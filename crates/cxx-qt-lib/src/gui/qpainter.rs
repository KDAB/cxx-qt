// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpainter.h");
        type QPainter = super::QPainter;
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;
        include!("cxx-qt-lib/qline.h");
        type QLine = crate::QLine;
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;
        include!("cxx-qt-lib/qimage.h");
        type QImage = crate::QImage;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Draws the ellipse defined by the given rectangle.
        #[rust_name = "draw_ellipse"]
        fn drawEllipse(self: &mut QPainter, rect: &QRect);

        /// Draws the given image into the given rectangle.
        #[rust_name = "draw_image"]
        fn drawImage(self: &mut QPainter, rectangle: &QRect, image: &QImage);

        /// Draws a line defined by line.
        #[rust_name = "draw_line"]
        fn drawLine(self: &mut QPainter, line: &QLine);

        /// Draws a single point at the given position using the current pen's color.
        #[rust_name = "draw_point"]
        fn drawPoint(self: &mut QPainter, point: &QPoint);

        /// Draws the given text with the currently defined text direction, beginning at the given position.
        #[rust_name = "draw_text"]
        fn drawText(self: &mut QPainter, point: &QPoint, text: &QString);

        /// Fills the given rectangle with the color specified.
        // #[rust_name = "fill_rect"]
        // fn fillRect(self: &mut QPainter, rectangle: &QRect, color: &QColor);

        /// Saves the current painter state (pushes the state onto a stack).
        /// A save() must be followed by a corresponding restore(); the end() function unwinds the stack.
        fn save(self: &mut QPainter);

        /// Enables clipping if enable is true, or disables clipping if enable is false.
        #[rust_name = "set_clipping"]
        fn setClipping(self: &mut QPainter, enable: bool);

        /// Restores the current painter state (pops a saved state off the stack).
        fn restore(self: &mut QPainter);

        /// Translates the coordinate system by the given offset.
        fn translate(self: &mut QPainter, offset: &QPoint);

        /// Returns the window rectangle.
        fn window(self: &QPainter) -> QRect;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpainter_init_default"]
        fn construct() -> QPainter;
    }
}

#[repr(C)]
pub struct QPainter {
    _cspec: MaybeUninit<i32>,
}

impl Default for QPainter {
    /// Constructs a painter.
    fn default() -> Self {
        ffi::qpainter_init_default()
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QPainter {
    type Id = type_id!("QPainter");
    type Kind = cxx::kind::Trivial;
}
