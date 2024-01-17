// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type LayoutDirection = crate::LayoutDirection;
        type BGMode = crate::BGMode;
        type ClipOperation = crate::ClipOperation;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qpainter.h");
        type QPainter;
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;
        include!("cxx-qt-lib/qrectf.h");
        type QRectF = crate::QRectF;
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
        include!("cxx-qt-lib/qpainterpath.h");
        type QPainterPath = crate::QPainterPath;
        include!("cxx-qt-lib/qfont.h");
        type QFont = crate::QFont;

        /// Returns the current background mode.
        #[rust_name = "background_mode"]
        fn backgroundMode(self: &QPainter) -> BGMode;

        /// Returns the currently set brush origin.
        #[rust_name = "brush_origin"]
        fn brushOrigin(self: &QPainter) -> QPoint;

        /// Returns the bounding rectangle of the current clip if there is a clip;
        /// otherwise returns an empty rectangle. Note that the clip region is given in logical coordinates.
        #[rust_name = "clip_bounding_rect_or_empty"]
        fn clipBoundingRect(self: &QPainter) -> QRectF;

        /// Returns the current clip path in logical coordinates.
        #[rust_name = "clip_path"]
        fn clipPath(self: &QPainter) -> QPainterPath;

        /// Draws the arc defined by the rectangle beginning at (x, y) with the specified width and height,
        /// and the given startAngle and spanAngle.
        #[rust_name = "draw_arc"]
        fn drawArc(
            self: Pin<&mut QPainter>,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            startAngle: i32,
            spanAngle: i32,
        );

        /// Draws the chord defined by the given rectangle, startAngle and spanAngle.
        #[rust_name = "draw_chord"]
        fn drawChord(
            self: Pin<Pin<&mut QPainter>>,
            rectangle: &QRect,
            startAngle: i32,
            spanAngle: i32,
        );

        /// Draws the ellipse defined by the given rectangle.
        #[rust_name = "draw_ellipse"]
        fn drawEllipse(self: Pin<&mut QPainter>, rect: &QRect);

        /// Draws the given image into the given rectangle.
        #[rust_name = "draw_image"]
        fn drawImage(self: Pin<&mut QPainter>, rectangle: &QRect, image: &QImage);

        /// Draws a line defined by line.
        #[rust_name = "draw_line"]
        fn drawLine(self: Pin<&mut QPainter>, line: &QLine);

        /// Draws the given painter path using the current pen for outline and the current brush for filling.
        #[rust_name = "draw_path"]
        fn drawPath(self: Pin<&mut QPainter>, path: &QPainterPath);

        /// Draws a pie defined by the given rectangle, startAngle and spanAngle.
        #[rust_name = "draw_pie"]
        fn drawPie(self: Pin<&mut QPainter>, rectangle: &QRectF, startAngle: i32, spanAngle: i32);

        /// Draws a single point at the given position using the current pen's color.
        #[rust_name = "draw_point"]
        fn drawPoint(self: Pin<&mut QPainter>, point: &QPoint);

        /// Draws the given text with the currently defined text direction, beginning at the given position.
        #[rust_name = "draw_text"]
        fn drawText(self: Pin<&mut QPainter>, point: &QPoint, text: &QString);

        /// Erases the area inside the given rectangle.
        #[rust_name = "erase_rect"]
        fn eraseRect(self: Pin<&mut QPainter>, rectangle: &QRectF);

        /// Fills the given rectangle with the color specified.
        #[rust_name = "fill_rect"]
        fn fillRect(self: Pin<&mut QPainter>, rectangle: &QRectF, color: &QColor);

        /// Returns the currently set font used for drawing text.
        fn font(self: &QPainter) -> &QFont;

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
        fn save(self: Pin<&mut QPainter>);

        /// Sets the background mode of the painter to the given mode
        #[rust_name = "set_background_mode"]
        fn setBackgroundMode(self: Pin<&mut QPainter>, mode: BGMode);

        /// Enables clipping if enable is true, or disables clipping if enable is false.
        #[rust_name = "set_clipping"]
        fn setClipping(self: Pin<&mut QPainter>, enable: bool);

        /// Enables clipping, and sets the clip path for the painter to the given path, with the clip operation.
        #[rust_name = "set_clip_path"]
        fn setClipPath(self: Pin<&mut QPainter>, path: &QPainterPath, operation: ClipOperation);

        /// Enables clipping, and sets the clip region to the given rectangle using the given clip operation.
        ///
        /// Note that the clip rectangle is specified in logical (painter) coordinates.
        #[rust_name = "set_clip_rect"]
        fn setClipRect(self: Pin<&mut QPainter>, rectangle: &QRect, operation: ClipOperation);

        /// Sets the painter's font to the given font.
        #[rust_name = "set_font"]
        fn setFont(self: Pin<&mut QPainter>, font: &QFont);

        /// Sets the layout direction used by the painter when drawing text, to the specified direction.
        #[rust_name = "set_layout_direction"]
        fn setLayoutDirection(self: Pin<&mut QPainter>, direction: LayoutDirection);

        /// Sets the painter's pen to have style Qt::SolidLine, width 1 and the specified color.
        #[rust_name = "set_pen"]
        fn setPen(self: Pin<&mut QPainter>, color: &QColor);

        /// Sets the opacity of the painter to opacity. The value should be in the range 0.0 to 1.0,
        /// where 0.0 is fully transparent and 1.0 is fully opaque.
        #[rust_name = "set_opacity"]
        fn setOpacity(self: Pin<&mut QPainter>, opacity: f64);

        /// Sets the painter's viewport rectangle to the given rectangle, and enables view transformations.
        #[rust_name = "set_viewport"]
        fn setViewport(self: Pin<&mut QPainter>, rectangle: &QRect);

        /// Sets the painter's window to the given rectangle, and enables view transformations.
        #[rust_name = "set_window"]
        fn setWindow(self: Pin<&mut QPainter>, rectangle: &QRect);

        /// Restores the current painter state (pops a saved state off the stack).
        fn restore(self: Pin<&mut QPainter>);

        /// Rotates the coordinate system clockwise. The given angle parameter is in degrees.
        fn rotate(self: Pin<&mut QPainter>, angle: f64);

        /// Translates the coordinate system by the given offset.
        fn translate(self: Pin<&mut QPainter>, offset: &QPoint);

        /// Returns the window rectangle.
        fn window(self: &QPainter) -> QRect;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qpainter_init_default"]
        fn make_unique() -> UniquePtr<QPainter>;
    }
}

pub use ffi::QPainter;

impl QPainter {
    /// Create a QPainter
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qpainter_init_default()
    }

    /// Returns the bounding rectangle of the current clip if there is a clip;
    /// otherwise returns `None`. Note that the clip region is given in logical coordinates.
    pub fn clip_bounding_rect(&self) -> Option<ffi::QRectF> {
        let result = self.clip_bounding_rect_or_empty();
        if result.is_valid() {
            Some(result)
        } else {
            None
        }
    }
}
