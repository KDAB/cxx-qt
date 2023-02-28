// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

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
        type QVector2D = crate::QVector2D;
        include!("cxx-qt-lib/qvector3d.h");
        type QVector3D = crate::QVector3D;
        include!("cxx-qt-lib/qvector4d.h");
        type QVector4D = super::QVector4D;

        /// Returns true if the x, y, z, and w coordinates are set to 0.0, otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QVector4D) -> bool;

        /// Returns the length of the vector from the origin.
        fn length(self: &QVector4D) -> f32;

        /// Returns the squared length of the vector from the origin.
        /// This is equivalent to the dot product of the vector with itself.
        #[rust_name = "length_squared"]
        fn lengthSquared(self: &QVector4D) -> f32;

        /// Normalizes the current vector in place. Nothing happens
        /// if this vector is a null vector or the length of the vector is very close to 1.
        fn normalize(self: &mut QVector4D);

        /// Returns the normalized unit vector form of this vector.
        ///
        /// If this vector is null, then a null vector is returned.
        /// If the length of the vector is very close to 1, then the vector will be returned as-is.
        /// Otherwise the normalized form of the vector of length 1 will be returned.
        fn normalized(self: &QVector4D) -> QVector4D;

        /// Sets the w coordinate of this point to the given finite w coordinate.
        #[rust_name = "set_w"]
        fn setW(self: &mut QVector4D, w: f32);
        /// Sets the x coordinate of this point to the given finite x coordinate.
        #[rust_name = "set_x"]
        fn setX(self: &mut QVector4D, x: f32);
        /// Sets the y coordinate of this point to the given finite y coordinate.
        #[rust_name = "set_y"]
        fn setY(self: &mut QVector4D, y: f32);
        /// Sets the z coordinate of this point to the given finite z coordinate.
        #[rust_name = "set_z"]
        fn setZ(self: &mut QVector4D, z: f32);

        // From trait is more idiomatic to Rust and implemented in QPoint and QPointF
        #[doc(hidden)]
        #[rust_name = "to_point"]
        fn toPoint(self: &QVector4D) -> QPoint;
        #[doc(hidden)]
        #[rust_name = "to_pointf"]
        fn toPointF(self: &QVector4D) -> QPointF;

        /// Returns the 2D vector form of this 4D vector,
        /// dividing the x and y coordinates by the w coordinate and dropping the z coordinate.
        /// Returns a null vector if w is zero.
        #[rust_name = "to_vector_2d_affine"]
        fn toVector2DAffine(self: &QVector4D) -> QVector2D;
        /// Returns the 3D vector form of this 4D vector,
        /// dividing the x, y, and z coordinates by the w coordinate.
        /// Returns a null vector if w is zero.
        #[rust_name = "to_vector_3d_affine"]
        fn toVector3DAffine(self: &QVector4D) -> QVector3D;

        /// Returns the w coordinate of this point.
        fn w(self: &QVector4D) -> f32;
        /// Returns the x coordinate of this point.
        fn x(self: &QVector4D) -> f32;
        /// Returns the y coordinate of this point.
        fn y(self: &QVector4D) -> f32;
        /// Returns the z coordinate of this point.
        fn z(self: &QVector4D) -> f32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qvector4d_init_qvector3d"]
        fn construct(vector: QVector3D) -> QVector4D;
        #[doc(hidden)]
        #[rust_name = "qvector4d_init_qvector2d"]
        fn construct(vector: QVector2D) -> QVector4D;

        #[doc(hidden)]
        #[rust_name = "qvector4d_init_qpointf"]
        fn construct(point: QPointF) -> QVector4D;
        #[doc(hidden)]
        #[rust_name = "qvector4d_init_qpoint"]
        fn construct(point: QPoint) -> QVector4D;

        #[doc(hidden)]
        #[rust_name = "qvector4d_init"]
        fn construct(x: f32, y: f32, z: f32, w: f32) -> QVector4D;

        #[doc(hidden)]
        #[rust_name = "qvector4d_init_default"]
        fn construct() -> QVector4D;

        #[doc(hidden)]
        #[rust_name = "qvector4d_to_qstring"]
        fn toQString(value: &QVector4D) -> QString;
        #[doc(hidden)]
        #[rust_name = "qvector4d_plus"]
        fn operatorPlus(a: &QVector4D, b: &QVector4D) -> QVector4D;
        #[doc(hidden)]
        #[rust_name = "qvector4d_minus"]
        fn operatorMinus(a: &QVector4D, b: &QVector4D) -> QVector4D;
        #[doc(hidden)]
        #[rust_name = "qvector4d_mul"]
        fn operatorMul(a: f32, b: &QVector4D) -> QVector4D;
        #[doc(hidden)]
        #[rust_name = "qvector4d_div"]
        fn operatorDiv(a: f32, b: &QVector4D) -> QVector4D;
    }
}

/// The QVector4D class represents a vector or vertex in 4D space.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QVector4D {
    v: [f32; 4],
}

impl QVector4D {
    /// Constructs a vector with coordinates (xpos, ypos, zpos, wpos).
    /// All parameters must be finite.
    pub fn new(xpos: f32, ypos: f32, zpos: f32, wpos: f32) -> Self {
        ffi::qvector4d_init(xpos, ypos, zpos, wpos)
    }
}

impl Default for QVector4D {
    /// Constructs a null vector, i.e. with coordinates (0, 0, 0, 0).
    fn default() -> Self {
        ffi::qvector4d_init_default()
    }
}

impl std::fmt::Display for QVector4D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", ffi::qvector4d_to_qstring(self))
    }
}

impl std::ops::Add for QVector4D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ffi::qvector4d_plus(&self, &other)
    }
}

impl std::ops::Sub for QVector4D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ffi::qvector4d_minus(&self, &other)
    }
}

impl std::ops::Mul<f32> for QVector4D {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        ffi::qvector4d_mul(rhs, &self)
    }
}

impl std::ops::Div<f32> for QVector4D {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        ffi::qvector4d_div(rhs, &self)
    }
}

impl From<crate::QVector3D> for QVector4D {
    /// Constructs a 4D vector from the specified 3D vector.
    /// The w coordinate is set to zero.
    fn from(value: crate::QVector3D) -> Self {
        ffi::qvector4d_init_qvector3d(value)
    }
}

impl From<crate::QVector2D> for QVector4D {
    /// Constructs a 4D vector from the specified 2D vector.
    /// The z and w coordinates are set to zero.
    fn from(value: crate::QVector2D) -> Self {
        ffi::qvector4d_init_qvector2d(value)
    }
}

impl From<crate::QPointF> for QVector4D {
    /// Constructs a vector with x and y coordinates from a 2D point, and z and w coordinates of 0.
    fn from(value: crate::QPointF) -> Self {
        ffi::qvector4d_init_qpointf(value)
    }
}

impl From<QVector4D> for crate::QPointF {
    /// Returns the QPointF form of this 4D vector. The z and w coordinates are dropped.
    fn from(value: QVector4D) -> Self {
        value.to_pointf()
    }
}

impl From<crate::QPoint> for QVector4D {
    /// Constructs a vector with x and y coordinates from a 2D point, and z and w coordinates of 0.
    fn from(value: crate::QPoint) -> Self {
        ffi::qvector4d_init_qpoint(value)
    }
}

impl From<QVector4D> for crate::QPoint {
    /// Returns the QPoint form of this 4D vector. The z and w coordinates are dropped.
    /// The x and y coordinates are rounded to nearest integers.
    fn from(value: QVector4D) -> Self {
        value.to_point()
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QVector4D is trivial.
unsafe impl ExternType for QVector4D {
    type Id = type_id!("QVector4D");
    type Kind = cxx::kind::Trivial;
}
