// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{unsafe_impl_qflag, QFlags};

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
        /// The default Qt format, which includes the day and month name, the day number in the month, and the year in full. The day and month names will be short names in English (C locale). This effectively uses, for a date, format `ddd MMM d yyyy`, for a time `HH:mm:ss` and combines these as `ddd MMM d HH:mm:ss yyyy` for a date-time, with an optional zone-offset suffix, where relevant. When reading from a string, a fractional part is also recognized on the seconds of a time part, as `HH:mm:ss.zzz`, and some minor variants on the format may be recognized, for compatibility with earlier versions of Qt and with changes to the format planned for the future. In particular, the zone-offset suffix presently uses GMT\[±`tzoff`\] with a `tzoff` in `HH[[:]mm]` format (two-digit hour and optional two-digit minutes, with optional colon separator); this shall change to use UTC in place of GMT in a future release of Qt, so the planned UTC format is recognized.
        TextDate = 0,
        /// ISO 8601 extended format: uses `yyyy-MM-dd` for dates, `HH:mm:ss.zzz` for times or `yyyy-MM-ddTHH:mm:ss.zzz` (e.g. `2017-07-24T15:46:29.739`) for combined dates and times, optionally with a time-zone suffix (`Z` for UTC otherwise an offset as `±HH:mm`) where appropriate. When parsed, a single space, `' '`, may be used in place of the `'T'` separator between date and time; no other spacing characters are permitted. This format also accepts `HH:mm` and plain `HH` formats for the time part, either of which may include a fractional part, `HH:mm.zzz` or `HH.zzz`, applied to the last field present (hour or minute).
        ISODateWithMs = 9,
        /// ISO 8601 extended format, as for `ISODateWithMs`, but omitting the milliseconds (`.zzz`) part when converting to a string. There is no difference when reading from a string: if a fractional part is present on the last time field, either format will accept it.
        ISODate = 1,
        /// RFC 2822, RFC 850 and RFC 1036 format: when converting dates to string form, format `dd MMM yyyy` is used, for times the format is `HH:mm:ss`. For combined date and time, these are combined as `dd MMM yyyy HH:mm:ss ±tzoff` (omitting the optional leading day of the week from the first format recognized). When reading from a string either `[ddd,] dd MMM yyyy [HH:mm[:ss]][ ±tzoff]` or `ddd MMM dd[ HH:mm:ss] yyyy[ ±tzoff]` will be recognized for combined dates and times, where `tzoff` is a timezone offset in `HHmm` format. Arbitrary spacing may appear before or after the text and any non-empty spacing may replace the spaces in this format. For dates and times separately, the same formats are matched and the unwanted parts are ignored. In particular, note that a time is not recognized without an accompanying date.
        RFC2822Date = 8,
    }

    /// This enum specifies how [`QString::split`](crate::QString::split) functions should behave with respect to empty strings.
    #[repr(i32)]
    enum SplitBehaviorFlags {
        /// If a field is empty, keep it in the result.
        KeepEmptyParts,
        /// If a field is empty, don't include it in the result.
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

    /// This enum type defines the pen styles that can be drawn using [`QPainter`](crate::QPainter).
    #[repr(i32)]
    enum PenStyle {
        /// No line at all. For example, [`QPainter::draw_rect_f`](crate::QPainter::draw_rect_f) fills but does not draw any boundary line.
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
        /// A custom pattern defined using [QPainterPathStroker::setDashPattern](https://doc.qt.io/qt/qpainterpathstroker.html#setDashPattern)().
        CustomDashLine,
    }

    /// This enum type defines the pen cap styles supported by Qt, i.e. the line end caps that can be drawn using [`QPainter`](crate::QPainter).
    #[repr(i32)]
    enum PenCapStyle {
        /// A square line end that does not cover the end point of the line.
        FlatCap = 0x00,
        /// A square line end that covers the end point and extends beyond it by half the line width.
        SquareCap = 0x10,
        /// A rounded line end.
        RoundCap = 0x20,
        #[doc(hidden)]
        MPenCapStyle = 0x30,
    }

    /// This enum type defines the pen join styles supported by Qt, i.e. which joins between two connected lines can be drawn using [`QPainter`](crate::QPainter).
    #[repr(i32)]
    enum PenJoinStyle {
        /// The outer edges of the lines are extended to meet at an angle, and this area is filled.
        MiterJoin = 0x00,
        /// The triangular notch between the two lines is filled.
        BevelJoin = 0x40,
        /// A circular arc between the two lines is filled.
        RoundJoin = 0x80,
        /// A miter join corresponding to the definition of a miter join in the SVG 1.2 Tiny specification.
        SvgMiterJoin = 0x100,
        #[doc(hidden)]
        MPenJoinStyle = 0x1c0,
    }

    /// Specifies which method should be used to fill the paths and polygons.
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
        /// Left-to-right layout.
        LeftToRight,
        /// Right-to-left layout.
        RightToLeft,
        /// Automatic layout. Text directionality is determined from the content of the string to be layouted.
        LayoutDirectionAuto,
    }

    /// This enum type specifies the background mode.
    #[repr(i32)]
    enum BGMode {
        TransparentMode,
        OpaqueMode,
    }

    #[repr(i32)]
    enum ClipOperation {
        /// This operation turns clipping off.
        NoClip,
        /// Replaces the current clip path/rect/region with the one supplied in the function call.
        ReplaceClip,
        /// Intersects the current clip path/rect/region with the one supplied in the function call.
        IntersectClip,
    }

    /// This enum is used by [`QPainter::draw_rounded_rect`](crate::QPainter::draw_rounded_rect) and [`QPainterPath::add_rounded_rect`](crate::QPainterPath::add_rounded_rect)
    /// functions to specify the radii of rectangle corners with respect to the dimensions
    /// of the bounding rectangles specified.
    #[repr(i32)]
    enum SizeMode {
        /// Specifies the size using absolute measurements.
        AbsoluteSize,
        /// Specifies the size relative to the bounding rectangle, typically using percentage measurements.
        RelativeSize,
    }

    /// This enum describes the modifier keys.
    #[derive(Debug)]
    #[repr(u32)]
    enum KeyboardModifier {
        /// No modifier key is pressed.
        NoModifier = 0x00000000,
        /// A Shift key on the keyboard is pressed.
        ShiftModifier = 0x02000000,
        /// A Ctrl key on the keyboard is pressed.
        ControlModifier = 0x04000000,
        /// An Alt key on the keyboard is pressed.
        AltModifier = 0x08000000,
        /// A Meta key on the keyboard is pressed.
        MetaModifier = 0x10000000,
        /// A keypad button is pressed.
        KeypadModifier = 0x20000000,
        /// X11 only (unless activated on Windows by a command line argument).
        /// A Mode_switch key on the keyboard is pressed.
        GroupSwitchModifier = 0x40000000,
    }

    /// This enum type describes the different mouse buttons.
    #[derive(Debug)]
    #[repr(u32)]
    enum MouseButton {
        /// The button state does not refer to any button.
        NoButton = 0x00000000,
        /// This value corresponds to a mask of all possible mouse buttons. Use to set the
        /// ['acceptedButtons'](https://doc.qt.io/qt/qml-qtquick-mousearea.html#acceptedButtons-prop) property of a [MouseArea](https://doc.qt.io/qt/qml-qtquick-mousearea.html) to accept ALL mouse buttons.
        AllButtons = 0x07ffffff,
        /// The left button is pressed, or an event refers to the left button. (The left button may
        /// be the right button on left-handed mice.)
        LeftButton = 0x00000001,
        /// The right button.
        RightButton = 0x00000002,
        /// The middle button.
        MiddleButton = 0x00000004,
        /// The 'Back' button. (Typically present on the 'thumb' side of a mouse with extra buttons.
        /// This is NOT the tilt wheel.)
        BackButton = 0x00000008,
        /// The 'Forward' button. (Typically present beside the 'Back' button, and also pressed by
        /// the thumb.)
        ForwardButton = 0x00000010,
        /// The 'Task' button.
        TaskButton = 0x00000020,
        ExtraButton4 = 0x00000040,
        ExtraButton5 = 0x00000080,
        ExtraButton6 = 0x00000100,
        ExtraButton7 = 0x00000200,
        ExtraButton8 = 0x00000400,
        ExtraButton9 = 0x00000800,
        ExtraButton10 = 0x00001000,
        ExtraButton11 = 0x00002000,
        ExtraButton12 = 0x00004000,
        ExtraButton13 = 0x00008000,
        ExtraButton14 = 0x00010000,
        ExtraButton15 = 0x00020000,
        ExtraButton16 = 0x00040000,
        ExtraButton17 = 0x00080000,
        ExtraButton18 = 0x00100000,
        ExtraButton19 = 0x00200000,
        ExtraButton20 = 0x00400000,
        ExtraButton21 = 0x00800000,
        ExtraButton22 = 0x01000000,
        ExtraButton23 = 0x02000000,
        ExtraButton24 = 0x04000000,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type AspectRatioMode;
        type CaseSensitivity;
        type DateFormat;
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
        type MouseButton;
        type KeyboardModifier;
    }
}

pub use ffi::{
    AspectRatioMode, BGMode, CaseSensitivity, ClipOperation, DateFormat, FillRule,
    KeyboardModifier, LayoutDirection, MouseButton, PenCapStyle, PenJoinStyle, PenStyle, SizeMode,
    SplitBehaviorFlags, TimeSpec, TransformationMode,
};

// Reexport ConnectionType from cxx-qt
pub use cxx_qt::ConnectionType;

/// [`QFlags`] of [`MouseButton`].
pub type MouseButtons = QFlags<MouseButton>;
/// [`QFlags`] of [`KeyboardModifier`].
pub type KeyboardModifiers = QFlags<KeyboardModifier>;

unsafe_impl_qflag!(MouseButton, "Qt::MouseButtons", u32);
unsafe_impl_qflag!(KeyboardModifier, "Qt::KeyboardModifiers", u32);
