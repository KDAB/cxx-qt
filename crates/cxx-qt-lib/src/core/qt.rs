// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge(namespace = "Qt")]
mod ffi {
    /// This enum type defines what happens to the aspect ratio when scaling an rectangle.
    #[repr(i32)]
    enum AspectRatioMode {
        /// The size is scaled freely. The aspect ratio is not preserved.
        IgnoreAspectRatio,
        /// The size is scaled to a rectangle as large as possible inside a given rectangle, preserving the aspect ratio.
        KeepAspectRatio,
        /// The size is scaled to a rectangle as small as possible outside a given rectangle, preserving the aspect ratio.
        KeepAspectRatioByExpanding,
    }

    #[repr(i32)]
    enum CaseSensitivity {
        CaseInsensitive,
        CaseSensitive,
    }

    #[repr(i32)]
    enum DateFormat {
        TextDate = 0,
        ISODateWithMs = 9,
        ISODate = 1,
        RFC2822Date = 8,
    }

    #[repr(i32)]
    enum SplitBehaviorFlags {
        KeepEmptyParts,
        SkipEmptyParts,
    }

    #[repr(i32)]
    enum TimeSpec {
        /// Local time, controlled by a system time-zone setting.
        LocalTime,
        /// Coordinated Universal Time.
        UTC,
        /// An offset in seconds from Coordinated Universal Time.
        OffsetFromUTC,
        /// A named time zone.
        TimeZone,
    }

    /// This enum type defines whether image transformations (e.g., scaling) should be smooth or not.
    #[repr(i32)]
    enum TransformationMode {
        /// The transformation is performed quickly, with no smoothing.
        FastTransformation,
        /// The resulting image is transformed using bilinear filtering.
        SmoothTransformation,
    }

    /// This enum type defines the pen styles that can be drawn using QPainter.
    #[repr(i32)]
    enum PenStyle {
        /// no line at all. For example, QPainter::drawRect() fills but does not draw any boundary line.
        NoPen,
        /// A plain line.
        SolidLine,
        /// Dashes separated by a few pixels.
        DashLine,
        /// Dots separated by a few pixels.
        DotLine,
        /// Alternate dots and dashes.
        DashDotLine,
        /// One dash, two dots, one dash, two dots.
        DashDotDotLine,
        /// A custom pattern defined using QPainterPathStroker::setDashPattern().
        CustomDashLine,
    }

    /// This enum type defines the line endcap style
    #[repr(i32)]
    enum PenCapStyle {
        FlatCap = 0x00,
        SquareCap = 0x10,
        RoundCap = 0x20,
        MPenCapStyle = 0x30,
    }

    /// This enum type defines the line join style.
    #[repr(i32)]
    enum PenJoinStyle {
        MiterJoin = 0x00,
        BevelJoin = 0x40,
        RoundJoin = 0x80,
        SvgMiterJoin = 0x100,
        MPenJoinStyle = 0x1c0,
    }

    #[repr(i32)]
    enum FillRule {
        /// Specifies that the region is filled using the odd even fill rule.
        /// With this rule, we determine whether a point is inside the shape by using
        /// the following method. Draw a horizontal line from the point to a location
        /// outside the shape, and count the number of intersections. If the number of
        /// intersections is an odd number, the point is inside the shape. This mode is the default.
        OddEvenFill,
        /// Specifies that the region is filled using the non zero winding rule.
        /// With this rule, we determine whether a point is inside the shape by using the following method.
        /// Draw a horizontal line from the point to a location outside the shape. Determine whether
        /// the direction of the line at each intersection point is up or down. The winding number is determined
        /// by summing the direction of each intersection. If the number is non zero, the point is inside the shape.
        /// This fill mode can also in most cases be considered as the intersection of closed shapes.
        WindingFill,
    }

    /// This enum type specifies the direction of Qt's layouts and text handling.
    #[repr(i32)]
    enum LayoutDirection {
        LeftToRight,
        RightToLeft,
        LayoutDirectionAuto,
    }

    /// This enum type specifies the background mode
    #[repr(i32)]
    enum BGMode {
        TransparentMode,
        OpaqueMode,
    }

    #[repr(i32)]
    enum ClipOperation {
        NoClip,
        ReplaceClip,
        IntersectClip,
    }

    /// This enum is used by QPainter::drawRoundedRect() and QPainterPath::addRoundedRect()
    /// functions to specify the radii of rectangle corners with respect to the dimensions
    /// of the bounding rectangles specified.
    #[repr(i32)]
    enum SizeMode {
        /// Specifies the size using absolute measurements.
        AbsoluteSize,
        /// Specifies the size relative to the bounding rectangle, typically using percentage measurements.
        RelativeSize,
    }

    #[repr(i32)]
    enum DayOfWeek {
        Monday = 1,
        Tuesday = 2,
        Wednesday = 3,
        Thursday = 4,
        Friday = 5,
        Saturday = 6,
        Sunday = 7,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type AspectRatioMode;
        type CaseSensitivity;
        type DateFormat;
        type DayOfWeek;
        type SplitBehaviorFlags;
        type TimeSpec;
        type TransformationMode;
        type PenStyle;
        type PenCapStyle;
        type PenJoinStyle;
        type FillRule;
        type LayoutDirection;
        type BGMode;
        type ClipOperation;
        type SizeMode;
    }
}

pub use ffi::{
    AspectRatioMode, BGMode, CaseSensitivity, ClipOperation, DateFormat, DayOfWeek, FillRule,
    LayoutDirection, PenCapStyle, PenJoinStyle, PenStyle, SizeMode, SplitBehaviorFlags, TimeSpec,
    TransformationMode,
};

// Reexport ConnectionType from cxx-qt
pub use cxx_qt::ConnectionType;
