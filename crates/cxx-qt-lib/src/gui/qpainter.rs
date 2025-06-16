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
        type FillRule = crate::FillRule;
        type SizeMode = crate::SizeMode;
    }

    /// Warning: Only a [QPainter](https://doc.qt.io/qt-6/qpainter.html) operating on a [QImage](https://doc.qt.io/qt-6/qimage.html)
    /// fully supports all composition modes. The RasterOp modes are supported for X11 as described
    /// in [compositionMode](https://doc.qt.io/qt-6/qpainter.html#compositionMode)().
    ///
    /// Defines the modes supported for digital image compositing. Composition modes are used to specify
    /// how the pixels in one image, the source, are merged with the pixel in another image, the destination.
    /// Please note that the bitwise raster operation modes, denoted with a RasterOp prefix,
    /// are only natively supported in the X11 and raster paint engines. This means that the only way to utilize
    /// these modes on the Mac is via a QImage. The RasterOp denoted blend modes are not supported for pens
    /// and brushes with alpha components. Also, turning on the QPainter::Antialiasing render hint will
    /// effectively disable the RasterOp modes.
    /// The most common type is SourceOver (often referred to as just alpha blending) where the source pixel
    /// is blended on top of the destination pixel in such a way that the alpha component of the source
    /// defines the translucency of the pixel.
    /// Several composition modes require an alpha channel in the source or target images to have an effect.
    /// For optimal performance the image format Format_ARGB32_Premultiplied is preferred.
    /// When a composition mode is set it applies to all painting operator, pens, brushes, gradients and pixmap/image drawing.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QPainterCompositionMode {
        /// This is the default mode. The alpha of the source is used to blend the pixel on top of the destination.
        CompositionMode_SourceOver,
        /// The alpha of the destination is used to blend it on top of the source pixels. This mode is the inverse of CompositionMode_SourceOver.
        CompositionMode_DestinationOver,
        /// The pixels in the destination are cleared (set to fully transparent) independent of the source.
        CompositionMode_Clear,
        /// The output is the source pixel. (This means a basic copy operation and is identical to SourceOver when the source pixel is opaque).
        CompositionMode_Source,
        /// The output is the destination pixel. This means that the blending has no effect. This mode is the inverse of CompositionMode_Source.
        CompositionMode_Destination,
        /// The output is the source, where the alpha is reduced by that of the destination.
        CompositionMode_SourceIn,
        /// The output is the destination, where the alpha is reduced by that of the source. This mode is the inverse of CompositionMode_SourceIn.
        CompositionMode_DestinationIn,
        /// The output is the source, where the alpha is reduced by the inverse of destination.
        CompositionMode_SourceOut,
        /// The output is the destination, where the alpha is reduced by the inverse of the source. This mode is the inverse of CompositionMode_SourceOut.
        CompositionMode_DestinationOut,
        /// The source pixel is blended on top of the destination, with the alpha of the source pixel reduced by the alpha of the destination pixel.
        CompositionMode_SourceAtop,
        /// The destination pixel is blended on top of the source, with the alpha of the destination pixel is reduced by the alpha of the destination pixel.
        /// This mode is the inverse of CompositionMode_SourceAtop.
        CompositionMode_DestinationAtop,
        /// The source, whose alpha is reduced with the inverse of the destination alpha, is merged with the destination, whose alpha is reduced by the
        /// inverse of the source alpha. CompositionMode_Xor is not the same as the bitwise Xor.
        CompositionMode_Xor,

        //svg 1.2 blend modes
        /// Both the alpha and color of the source and destination pixels are added together.
        CompositionMode_Plus,
        /// The output is the source color multiplied by the destination. Multiplying a color with white leaves
        /// the color unchanged, while multiplying a color with black produces black.
        CompositionMode_Multiply,
        /// The source and destination colors are inverted and then multiplied. Screening a color with white produces
        /// white, whereas screening a color with black leaves the color unchanged.
        CompositionMode_Screen,
        /// Multiplies or screens the colors depending on the destination color. The destination color is mixed with
        /// the source color to reflect the lightness or darkness of the destination.
        CompositionMode_Overlay,
        /// The darker of the source and destination colors is selected.
        CompositionMode_Darken,
        /// The lighter of the source and destination colors is selected.
        CompositionMode_Lighten,
        /// The destination color is brightened to reflect the source color.
        /// A black source color leaves the destination color unchanged.
        CompositionMode_ColorDodge,
        /// The destination color is darkened to reflect the source color. A white source color leaves the destination color unchanged.
        CompositionMode_ColorBurn,
        /// Multiplies or screens the colors depending on the source color. A light source color will lighten
        /// the destination color, whereas a dark source color will darken the destination color.
        CompositionMode_HardLight,
        /// Darkens or lightens the colors depending on the source color. Similar to CompositionMode_HardLight.
        CompositionMode_SoftLight,
        /// Subtracts the darker of the colors from the lighter. Painting with white inverts the destination
        /// color, whereas painting with black leaves the destination color unchanged.
        CompositionMode_Difference,
        /// Similar to CompositionMode_Difference, but with a lower contrast. Painting with white inverts
        /// the destination color, whereas painting with black leaves the destination color unchanged.
        CompositionMode_Exclusion,

        // ROPs
        /// Does a bitwise OR operation on the source and destination pixels (src OR dst).
        RasterOp_SourceOrDestination,
        /// Does a bitwise AND operation on the source and destination pixels (src AND dst).
        RasterOp_SourceAndDestination,
        /// Does a bitwise XOR operation on the source and destination pixels (src XOR dst).
        RasterOp_SourceXorDestination,
        /// Does a bitwise NOR operation on the source and destination pixels ((NOT src) AND (NOT dst)).
        RasterOp_NotSourceAndNotDestination,
        /// Does a bitwise NAND operation on the source and destination pixels ((NOT src) OR (NOT dst)).
        RasterOp_NotSourceOrNotDestination,
        /// Does a bitwise operation where the source pixels are inverted and then XOR'ed with the destination ((NOT src) XOR dst).
        RasterOp_NotSourceXorDestination,
        /// Does a bitwise operation where the source pixels are inverted (NOT src).
        RasterOp_NotSource,
        ///Does a bitwise operation where the source is inverted and then AND'ed with the destination ((NOT src) AND dst).
        RasterOp_NotSourceAndDestination,
        /// Does a bitwise operation where the source is AND'ed with the inverted destination pixels (src AND (NOT dst)).
        RasterOp_SourceAndNotDestination,
        /// Does a bitwise operation where the source is inverted and then OR'ed with the destination ((NOT src) OR dst).
        RasterOp_NotSourceOrDestination,
        /// The pixels in the destination are cleared (set to 0) independent of the source
        RasterOp_SourceOrNotDestination,
        /// The pixels in the destination are set (set to 1) independent of the source.
        RasterOp_ClearDestination,
        /// Does a bitwise operation where the destination pixels are inverted (NOT dst).
        RasterOp_SetDestination,
        /// Does a bitwise operation where the source is OR'ed with the inverted destination pixels (src OR (NOT dst)).
        RasterOp_NotDestination,
    }

    /// Renderhints are used to specify flags to QPainter that may or may not be respected by any given engine.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QPainterRenderHint {
        /// Indicates that the engine should antialias edges of primitives if possible.
        Antialiasing = 0x01,
        /// Indicates that the engine should antialias text if possible.
        /// To forcibly disable antialiasing for text, do not use this hint.
        /// Instead, set QFont::NoAntialias on your font's style strategy.
        TextAntialiasing = 0x02,
        /// Indicates that the engine should use a smooth pixmap transformation algorithm
        /// (such as bilinear) rather than nearest neighbor.
        SmoothPixmapTransform = 0x04,
        /// Use a lossless image rendering, whenever possible. Currently, this hint is only
        /// used when QPainter is employed to output a PDF file through QPrinter or QPdfWriter,
        /// where drawImage()/drawPixmap() calls will encode images using a lossless compression
        /// algorithm instead of lossy JPEG compression. This value was added in Qt 5.13.
        LosslessImageRendering = 0x40,
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
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;
        include!("cxx-qt-lib/qline.h");
        type QLine = crate::QLine;
        include!("cxx-qt-lib/qlinef.h");
        type QLineF = crate::QLineF;
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;
        include!("cxx-qt-lib/qimage.h");
        type QImage = crate::QImage;
        include!("cxx-qt-lib/qpixmap.h");
        type QPixmap = crate::QPixmap;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qpainterpath.h");
        type QPainterPath = crate::QPainterPath;
        include!("cxx-qt-lib/qfont.h");
        type QFont = crate::QFont;
        include!("cxx-qt-lib/qpen.h");
        type QPen = crate::QPen;
        include!("cxx-qt-lib/qpolygon.h");
        type QPolygon = crate::QPolygon;
        include!("cxx-qt-lib/qregion.h");
        type QRegion = crate::QRegion;
        include!("cxx-qt-lib/qvector.h");
        type QVector_QLine = crate::QVector<QLine>;
        type QVector_QLineF = crate::QVector<QLineF>;

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

        /// Returns the currently set clip region. Note that the clip region is given in logical coordinates.
        #[rust_name = "clip_region"]
        fn clipRegion(self: &QPainter) -> QRegion;

        /// Returns the current composition mode.
        #[rust_name = "composition_mode"]
        fn compositionMode(self: &QPainter) -> QPainterCompositionMode;

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

        /// Draws the convex polygon defined by polygon using the current pen and brush.
        #[rust_name = "draw_convex_polygon"]
        fn drawConvexPolygon(self: Pin<&mut QPainter>, polygon: &QPolygon);

        /// Draws the ellipse defined by the given rectangle.
        #[rust_name = "draw_ellipse"]
        fn drawEllipse(self: Pin<&mut QPainter>, rect: &QRect);

        /// Draws the given image into the given rectangle.
        #[rust_name = "draw_image"]
        fn drawImage(self: Pin<&mut QPainter>, rectangle: &QRect, image: &QImage);

        /// Draws the rectangular portion source of the given pixmap into the given target in the paint device.
        #[rust_name = "draw_pixmap"]
        fn drawPixmap(self: Pin<&mut QPainter>,point: &QPointF, pixmap: &QPixmap);

        /// Draws a line defined by line.
        #[rust_name = "draw_line"]
        fn drawLine(self: Pin<&mut QPainter>, line: &QLine);

        /// Draws a line defined by line.
        #[rust_name = "draw_linef"]
        fn drawLine(self: Pin<&mut QPainter>, line: &QLineF);

        /// Draws the set of lines defined by the list lines using the current pen and brush.
        #[rust_name = "draw_lines"]
        fn drawLines(self: Pin<&mut QPainter>, lines: &QVector_QLine);

        /// Draws the set of lines defined by the list lines using the current pen and brush.
        #[rust_name = "draw_linefs"]
        fn drawLines(self: Pin<&mut QPainter>, lines: &QVector_QLineF);

        /// Draws the given painter path using the current pen for outline and the current brush for filling.
        #[rust_name = "draw_path"]
        fn drawPath(self: Pin<&mut QPainter>, path: &QPainterPath);

        /// Draws a pie defined by the given rectangle, startAngle and spanAngle.
        #[rust_name = "draw_pie"]
        fn drawPie(self: Pin<&mut QPainter>, rectangle: &QRectF, startAngle: i32, spanAngle: i32);

        /// Draws a single point at the given position using the current pen's color.
        #[rust_name = "draw_point"]
        fn drawPoint(self: Pin<&mut QPainter>, point: &QPoint);

        /// Draws the points in the vector points.
        #[rust_name = "draw_points"]
        fn drawPoints(self: Pin<&mut QPainter>, points: &QPolygon);

        /// Draws the polygon defined by the given points using the fill rule fillRule.
        #[rust_name = "draw_polygon"]
        fn drawPolygon(self: Pin<&mut QPainter>, points: &QPolygon, fillRule: FillRule);

        /// Draws the polyline defined by the given points using the current pen.
        #[rust_name = "draw_polyline"]
        fn drawPolyline(self: Pin<&mut QPainter>, points: &QPolygon);

        /// Draws the current rectangle with the current pen and brush.
        #[rust_name = "draw_rect_f"]
        fn drawRect(self: Pin<&mut QPainter>, rectangle: &QRectF);

        /// Draws the given rectangle rect with rounded corners.
        #[rust_name = "draw_rounded_rect"]
        fn drawRoundedRect(
            self: Pin<&mut QPainter>,
            rect: &QRectF,
            xRadius: f64,
            yRadius: f64,
            mode: SizeMode,
        );

        /// Draws the given text with the currently defined text direction, beginning at the given position.
        #[rust_name = "draw_text"]
        fn drawText(self: Pin<&mut QPainter>, point: &QPoint, text: &QString);

        #[rust_name = "draw_textf"]
        fn drawText(self: Pin<&mut QPainter>, point: &QPointF, text: &QString);

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

        /// Returns the painter's current pen.
        fn pen(self: &QPainter) -> &QPen;

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

        /// Sets the clip region to the given region using the specified clip operation. The default
        /// clip operation is to replace the current clip region.
        /// Note that the clip region is given in logical coordinates.
        #[rust_name = "set_clip_region"]
        fn setClipRegion(self: Pin<&mut QPainter>, region: &QRegion, operation: ClipOperation);

        /// Sets the composition mode to the given mode.
        #[rust_name = "set_composition_mode"]
        fn setCompositionMode(self: Pin<&mut QPainter>, mode: QPainterCompositionMode);

        /// Sets the painter's font to the given font.
        #[rust_name = "set_font"]
        fn setFont(self: Pin<&mut QPainter>, font: &QFont);

        /// Sets the layout direction used by the painter when drawing text, to the specified direction.
        #[rust_name = "set_layout_direction"]
        fn setLayoutDirection(self: Pin<&mut QPainter>, direction: LayoutDirection);

        /// Sets the opacity of the painter to opacity. The value should be in the range 0.0 to 1.0,
        /// where 0.0 is fully transparent and 1.0 is fully opaque.
        #[rust_name = "set_opacity"]
        fn setOpacity(self: Pin<&mut QPainter>, opacity: f64);

        /// Sets the painter's pen to be the given pen.
        #[rust_name = "set_pen"]
        fn setPen(self: Pin<&mut QPainter>, pen: &QPen);

        /// Sets the given render hint on the painter if on is true; otherwise clears the render hint.
        #[rust_name = "set_render_hint"]
        fn setRenderHint(self: Pin<&mut QPainter>, hint: QPainterRenderHint, on: bool);

        /// Sets the painter's viewport rectangle to the given rectangle, and enables view transformations.
        #[rust_name = "set_viewport"]
        fn setViewport(self: Pin<&mut QPainter>, rectangle: &QRect);

        /// Sets the painter's window to the given rectangle, and enables view transformations.
        #[rust_name = "set_window"]
        fn setWindow(self: Pin<&mut QPainter>, rectangle: &QRect);

        /// Draws the outline (strokes) the path path with the pen specified by pen
        #[rust_name = "stroke_path"]
        fn strokePath(self: Pin<&mut QPainter>, path: &QPainterPath, pen: &QPen);

        /// Returns true if hint is set; otherwise returns false.
        #[rust_name = "test_render_hint"]
        fn testRenderHint(self: &QPainter, hint: QPainterRenderHint) -> bool;

        /// Restores the current painter state (pops a saved state off the stack).
        fn restore(self: Pin<&mut QPainter>);

        /// Rotates the coordinate system clockwise. The given angle parameter is in degrees.
        fn rotate(self: Pin<&mut QPainter>, angle: f64);

        /// Translates the coordinate system by the given offset.
        fn translate(self: Pin<&mut QPainter>, offset: &QPoint);

        /// Returns true if view transformation is enabled; otherwise returns false.
        #[rust_name = "view_transform_enabled"]
        fn viewTransformEnabled(self: &QPainter) -> bool;

        /// Returns the window rectangle.
        fn window(self: &QPainter) -> QRect;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
        type QPainterCompositionMode;

        type QPainterRenderHint;

        #[doc(hidden)]
        #[rust_name = "qpainter_init_default"]
        fn make_unique() -> UniquePtr<QPainter>;

        #[doc(hidden)]
        #[rust_name = "qpainter_from_qpixmap"]
        unsafe fn qpainterFromQPixmap(pixmap: *mut QPixmap) -> UniquePtr<QPainter>;
    }
}

pub use ffi::{QPainter, QPainterCompositionMode, QPainterRenderHint};

impl QPainter {
    /// Create a QPainter
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qpainter_init_default()
    }

    /// Create a QPainter from a QPixmap
    pub fn new_from_qpixmap(pixmap: &mut crate::gui::QPixmap) -> cxx::UniquePtr<Self> {
        unsafe { ffi::qpainter_from_qpixmap(pixmap as *mut _) }
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
