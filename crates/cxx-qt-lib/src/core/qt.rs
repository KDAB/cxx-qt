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

    /// This enum describes the types of connection that can be used with signals.
    ///
    /// Note that UniqueConnection is not supported.
    #[repr(i32)]
    enum ConnectionType {
        /// If the receiver lives in the thread that emits the signal, Qt::DirectConnection is used.
        /// Otherwise, Qt::QueuedConnection is used. The connection type is determined when the signal is emitted.
        AutoConnection,
        /// The slot is invoked immediately when the signal is emitted.
        /// The slot is executed in the signalling thread.
        DirectConnection,
        /// The slot is invoked when control returns to the event loop of the receiver's thread.
        /// The slot is executed in the receiver's thread.
        QueuedConnection,
        /// Same as Qt::QueuedConnection, except that the signalling thread blocks until the slot returns.
        /// This connection must not be used if the receiver lives in the signalling thread, or else the application will deadlock.
        BlockingQueuedConnection,
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

    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type AspectRatioMode;
        type CaseSensitivity;
        type ConnectionType;
        type DateFormat;
        type SplitBehaviorFlags;
        type TimeSpec;
        type TransformationMode;
        type FillRule;
        type LayoutDirection;
        type BGMode;
        type ClipOperation;
    }
}

pub use ffi::{
    AspectRatioMode, BGMode, CaseSensitivity, ClipOperation, ConnectionType, DateFormat, FillRule,
    LayoutDirection, SplitBehaviorFlags, TimeSpec, TransformationMode,
};
