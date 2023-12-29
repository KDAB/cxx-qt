// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type LayoutDirection = crate::LayoutDirection;
        type BGMode = crate::BGMode;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qpainter.h");
        type QPainter = super::QPainter;
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = crate::QRectF;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;
        include!("cxx-qt-lib/qline.h");
        type QLine = crate::QLine;
        // include!("cxx-qt-lib/qcolor.h");
        // type QColor = crate::QColor;
        include!("cxx-qt-lib/qimage.h");
        type QImage = crate::QImage;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Returns the current background mode.
        #[rust_name = "background_mode"]
        fn backgroundMode(self: &QPainter) -> BGMode;

        /// Returns the currently set brush origin.
        #[rust_name = "brush_origin"]
        fn brushOrigin(self: &QPainter) -> QPoint;

        /// Returns the bounding rectangle of the current clip if there is a clip;
        /// otherwise returns an empty rectangle. Note that the clip region is given in logical coordinates.
        #[rust_name = "clip_bounding_rect"]
        fn clipBoundingRect(self: &QPainter) -> QRectF;

        /// Draws the arc defined by the rectangle beginning at (x, y) with the specified width and height,
        /// and the given startAngle and spanAngle.
        #[rust_name = "draw_arc"]
        fn drawArc(self: &mut QPainter, x: i32, y: i32, width: i32, height: i32, startAngle: i32, spanAngle: i32);

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

        /// Returns true if clipping has been set; otherwise returns false.
        #[rust_name = "has_clipping"]
        fn hasClipping(self: &QPainter) -> bool;

        /// Returns true if begin() has been called and end() has not yet been called; otherwise returns false.
        #[rust_name = "is_active"]
        fn isActive(self: &QPainter) -> bool;

        /// Returns the layout direction used by the painter when drawing text.
        #[rust_name = "layout_direction"]
        fn layoutDirection(self: &QPainter) -> LayoutDirection;

        /// Returns the opacity of the painter. The default value is 1.
        fn opacity(self: &QPainter) -> f64;

        /// Saves the current painter state (pushes the state onto a stack).
        /// A save() must be followed by a corresponding restore(); the end() function unwinds the stack.
        fn save(self: &mut QPainter);

        /// Sets the background mode of the painter to the given mode
        #[rust_name = "set_background_mode"]
        fn setBackgroundMode(self: &mut QPainter, mode: BGMode);

        /// Enables clipping if enable is true, or disables clipping if enable is false.
        #[rust_name = "set_clipping"]
        fn setClipping(self: &mut QPainter, enable: bool);

        /// Sets the layout direction used by the painter when drawing text, to the specified direction.
        #[rust_name = "set_layout_direction"]
        fn setLayoutDirection(self: &mut QPainter, direction: LayoutDirection);

        /// Sets the opacity of the painter to opacity. The value should be in the range 0.0 to 1.0,
        /// where 0.0 is fully transparent and 1.0 is fully opaque.
        #[rust_name = "set_opacity"]
        fn setOpacity(self: &mut QPainter, opacity: f64);

        /// Restores the current painter state (pops a saved state off the stack).
        fn restore(self: &mut QPainter);

        /// Rotates the coordinate system clockwise. The given angle parameter is in degrees.
        fn rotate(self: &mut QPainter, angle: f64);

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
