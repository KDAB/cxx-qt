// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fmt;

use cxx::{type_id, ExternType};

use crate::{QPoint, QPointF, QVector3D, QVector4D};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = crate::QPointF;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qvector2d.h");
        type QVector2D = super::QVector2D;
        include!("cxx-qt-lib/qvector3d.h");
        type QVector3D = crate::QVector3D;
        include!("cxx-qt-lib/qvector4d.h");
        type QVector4D = crate::QVector4D;

        /// Returns `true` if the x and y coordinates are set to 0.0, otherwise returns `false`.
        #[rust_name = "is_null"]
        fn isNull(self: &QVector2D) -> bool;

        /// Returns the length of the vector from the origin.
        fn length(self: &QVector2D) -> f32;

        /// Returns the squared length of the vector from the origin.
        /// This is equivalent to the dot product of the vector with itself.
        #[rust_name = "length_squared"]
        fn lengthSquared(self: &QVector2D) -> f32;

        /// Normalizes the current vector in place. Nothing happens
        /// if this vector is a null vector or the length of the vector is very close to 1.
        fn normalize(self: &mut QVector2D);

        /// Returns the normalized unit vector form of this vector.
        ///
        /// If this vector is null, then a null vector is returned.
        /// If the length of the vector is very close to 1, then the vector will be returned as-is.
        /// Otherwise the normalized form of the vector of length 1 will be returned.
        fn normalized(self: &QVector2D) -> QVector2D;

        /// Sets the x coordinate of this point to the given finite `x` coordinate.
        #[rust_name = "set_x"]
        fn setX(self: &mut QVector2D, x: f32);
        /// Sets the y coordinate of this point to the given finite `y` coordinate.
        #[rust_name = "set_y"]
        fn setY(self: &mut QVector2D, y: f32);

        // From trait is more idiomatic to Rust and implemented in QPoint and QPointF
        #[doc(hidden)]
        #[rust_name = "to_point"]
        fn toPoint(self: &QVector2D) -> QPoint;
        #[doc(hidden)]
        #[rust_name = "to_pointf"]
        fn toPointF(self: &QVector2D) -> QPointF;

        /// Returns the x coordinate of this point.
        fn x(self: &QVector2D) -> f32;
        /// Returns the y coordinate of this point.
        fn y(self: &QVector2D) -> f32;

    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qvector2d_init_qvector4d"]
        fn construct(vector: QVector4D) -> QVector2D;
        #[doc(hidden)]
        #[rust_name = "qvector2d_init_qvector3d"]
        fn construct(vector: QVector3D) -> QVector2D;

        #[doc(hidden)]
        #[rust_name = "qvector2d_init_qpointf"]
        fn construct(point: QPointF) -> QVector2D;
        #[doc(hidden)]
        #[rust_name = "qvector2d_init_qpoint"]
        fn construct(point: QPoint) -> QVector2D;

        #[doc(hidden)]
        #[rust_name = "qvector2d_init"]
        fn construct(x: f32, y: f32) -> QVector2D;

        #[doc(hidden)]
        #[rust_name = "qvector2d_init_default"]
        fn construct() -> QVector2D;

        // Note that Qt 5 takes const-ref and Qt 6 takes by-value
        //
        // We want by-value, as that is Rust-idiomatic, so for Qt 5 we create a proxy
        #[doc(hidden)]
        #[rust_name = "qvector2d_distance_to_line"]
        fn qvector2DDistanceToLine(
            vector: &QVector2D,
            point: QVector2D,
            direction: QVector2D,
        ) -> f32;
        #[doc(hidden)]
        #[rust_name = "qvector2d_distance_to_point"]
        fn qvector2DDistanceToPoint(vector: &QVector2D, point: QVector2D) -> f32;
        #[doc(hidden)]
        #[rust_name = "qvector2d_to_debug_qstring"]
        fn toDebugQString(value: &QVector2D) -> QString;
        #[doc(hidden)]
        #[rust_name = "qvector2d_plus"]
        fn operatorPlus(a: &QVector2D, b: &QVector2D) -> QVector2D;
        #[doc(hidden)]
        #[rust_name = "qvector2d_minus"]
        fn operatorMinus(a: &QVector2D, b: &QVector2D) -> QVector2D;
        #[doc(hidden)]
        #[rust_name = "qvector2d_mul"]
        fn operatorMul(a: f32, b: &QVector2D) -> QVector2D;
        #[doc(hidden)]
        #[rust_name = "qvector2d_div"]
        fn operatorDiv(a: f32, b: &QVector2D) -> QVector2D;
    }
}

/// The QVector2D class represents a vector or vertex in 2D space.
///
/// Qt Documentation: [QVector2D](https://doc.qt.io/qt/qvector2d.html#details)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QVector2D {
    v: [f32; 2],
}

impl QVector2D {
    /// Constructs a vector with coordinates (`xpos`, `ypos`). Both coordinates must be finite.
    pub fn new(xpos: f32, ypos: f32) -> Self {
        ffi::qvector2d_init(xpos, ypos)
    }

    /// Returns the distance that this vertex is from a line defined by `point` and the unit vector `direction`.
    ///
    /// If `direction` is a null vector, then it does not define a line.
    /// In that case, the distance from `point` to this vertex is returned.
    pub fn distance_to_line(&self, point: QVector2D, direction: QVector2D) -> f32 {
        ffi::qvector2d_distance_to_line(self, point, direction)
    }

    /// Returns the distance from this vertex to a point defined by the vertex `point`.
    pub fn distance_to_point(&self, point: QVector2D) -> f32 {
        ffi::qvector2d_distance_to_point(self, point)
    }
}

impl Default for QVector2D {
    /// Constructs a null vector, i.e. with coordinates (0, 0).
    fn default() -> Self {
        ffi::qvector2d_init_default()
    }
}

impl fmt::Display for QVector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qvector2d_to_debug_qstring(self).fmt(f)
    }
}

impl std::ops::Add for QVector2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ffi::qvector2d_plus(&self, &other)
    }
}

impl std::ops::Sub for QVector2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ffi::qvector2d_minus(&self, &other)
    }
}

impl std::ops::Mul<f32> for QVector2D {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        ffi::qvector2d_mul(rhs, &self)
    }
}

impl std::ops::Div<f32> for QVector2D {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        ffi::qvector2d_div(rhs, &self)
    }
}

impl From<QVector4D> for QVector2D {
    /// Constructs a vector with x and y coordinates from a 3D vector.
    /// The z and w coordinates of vector are dropped.
    fn from(value: QVector4D) -> Self {
        ffi::qvector2d_init_qvector4d(value)
    }
}

impl From<QVector3D> for QVector2D {
    /// Constructs a vector with x and y coordinates from a 3D vector.
    /// The z coordinate of vector is dropped.
    fn from(value: QVector3D) -> Self {
        ffi::qvector2d_init_qvector3d(value)
    }
}

impl From<QPointF> for QVector2D {
    /// Constructs a vector with x and y coordinates from a 2D point.
    fn from(value: QPointF) -> Self {
        ffi::qvector2d_init_qpointf(value)
    }
}

impl From<QVector2D> for QPointF {
    /// Returns the `QPointF` form of this 2D vector.
    /// Each coordinate is rounded to the nearest integer.
    fn from(value: QVector2D) -> Self {
        value.to_pointf()
    }
}

impl From<QPoint> for QVector2D {
    /// Constructs a vector with x and y coordinates from a 2D point.
    fn from(value: QPoint) -> Self {
        ffi::qvector2d_init_qpoint(value)
    }
}

impl From<QVector2D> for QPoint {
    /// Returns the `QPoint` form of this 2D vector.
    fn from(value: QVector2D) -> Self {
        value.to_point()
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QVector2D is trivial.
unsafe impl ExternType for QVector2D {
    type Id = type_id!("QVector2D");
    type Kind = cxx::kind::Trivial;
}
