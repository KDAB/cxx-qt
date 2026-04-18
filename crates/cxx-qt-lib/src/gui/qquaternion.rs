// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fmt;
use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};

use crate::{QMatrix3x3, QVector3D, QVector4D};

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qgenericmatrix.h");
        type QMatrix3x3 = crate::QMatrix3x3;
        include!("cxx-qt-lib/qvector3d.h");
        type QVector3D = crate::QVector3D;
        include!("cxx-qt-lib/qvector4d.h");
        type QVector4D = crate::QVector4D;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qquaternion.h");
        type QQuaternion = super::QQuaternion;

        /// Returns the conjugate of this quaternion, which is (-x, -y, -z, scalar).
        fn conjugated(&self) -> QQuaternion;

        #[doc(hidden)]
        #[rust_name = "get_axes_raw"]
        unsafe fn getAxes(
            &self,
            x_axis: *mut QVector3D,
            y_axis: *mut QVector3D,
            z_axis: *mut QVector3D,
        );

        #[doc(hidden)]
        #[rust_name = "get_axis_and_angle_raw"]
        unsafe fn getAxisAndAngle(&self, x: *mut f32, y: *mut f32, z: *mut f32, angle: *mut f32);

        #[doc(hidden)]
        #[rust_name = "get_euler_angles_raw"]
        unsafe fn getEulerAngles(&self, pitch: *mut f32, yaw: *mut f32, roll: *mut f32);

        /// Returns the inverse of this quaternion. If this quaternion is null, then a null quaternion is returned.
        fn inverted(&self) -> QQuaternion;

        /// Returns `true` if the x, y, and z components of this quaternion are set to 0.0, and the scalar component is set to 1.0; otherwise returns `false`.
        #[rust_name = "is_identity"]
        fn isIdentity(&self) -> bool;

        /// Returns `true` if the x, y, z, and scalar components of this quaternion are set to 0.0; otherwise returns `false`.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;

        /// Returns the length of the quaternion. This is also called the "norm".
        fn length(&self) -> f32;

        /// Returns the squared length of the quaternion.
        ///
        /// **Note:** Though cheap to compute, this is susceptible to overflow and underflow that [`length`](Self::length) avoids in many cases.
        #[rust_name = "length_squared"]
        fn lengthSquared(&self) -> f32;

        /// Normalizes the current quaternion in place. Nothing happens if this is a null quaternion or the length of the quaternion is very close to 1.
        fn normalize(&mut self);

        /// Returns the normalized unit form of this quaternion.
        ///
        /// If this quaternion is null, then a null quaternion is returned. If the length of the quaternion is very close to 1, then the quaternion will be returned as-is. Otherwise the normalized form of the quaternion of length 1 will be returned.
        fn normalized(&self) -> QQuaternion;

        /// Rotates `vector` with this quaternion to produce a new vector in 3D space.
        #[rust_name = "rotated_vector"]
        fn rotatedVector(&self, vector: &QVector3D) -> QVector3D;

        /// Returns the scalar component of this quaternion.
        fn scalar(&self) -> f32;

        /// Sets the scalar component of this quaternion to `scalar`.
        #[rust_name = "set_scalar"]
        fn setScalar(&mut self, scalar: f32);

        /// Sets the vector component of this quaternion to `vector`.
        #[rust_name = "set_vector"]
        fn setVector(&mut self, vector: &QVector3D);

        /// Sets the x coordinate of this quaternion's vector to the given `x` coordinate.
        #[rust_name = "set_x"]
        fn setX(&mut self, x: f32);

        /// Sets the y coordinate of this quaternion's vector to the given `y` coordinate.
        #[rust_name = "set_y"]
        fn setY(&mut self, y: f32);

        /// Sets the z coordinate of this quaternion's vector to the given `z` coordinate.
        #[rust_name = "set_z"]
        fn setZ(&mut self, z: f32);

        /// Calculates roll, pitch, and yaw Euler angles (in degrees) that corresponds to this quaternion.
        #[rust_name = "to_euler_angles"]
        fn toEulerAngles(&self) -> QVector3D;

        /// Creates a rotation matrix that corresponds to this quaternion.
        ///
        /// **Note:** If this quaternion is not normalized, the resulting rotation matrix will contain scaling information.
        #[rust_name = "to_rotation_matrix"]
        fn toRotationMatrix(&self) -> QMatrix3x3;

        #[doc(hidden)]
        #[rust_name = "to_vector_4d"]
        fn toVector4D(&self) -> QVector4D;

        /// Returns the vector component of this quaternion.
        fn vector(&self) -> QVector3D;

        /// Returns the x coordinate of this quaternion's vector.
        fn x(&self) -> f32;

        /// Returns the y coordinate of this quaternion's vector.
        fn y(&self) -> f32;

        /// Returns the z coordinate of this quaternion's vector.
        fn z(&self) -> f32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qquaternion_dot_product"]
        fn qquaternionDotProduct(q1: &QQuaternion, q2: &QQuaternion) -> f32;

        #[doc(hidden)]
        #[rust_name = "qquaternion_from_axes"]
        fn qquaternionFromAxes(
            x_axis: &QVector3D,
            y_axis: &QVector3D,
            z_axis: &QVector3D,
        ) -> QQuaternion;
        #[doc(hidden)]
        #[rust_name = "qquaternion_from_axis_and_angle"]
        fn qquaternionFromAxisAndAngle(x: f32, y: f32, z: f32, angle: f32) -> QQuaternion;
        #[doc(hidden)]
        #[rust_name = "qquaternion_from_euler_angles"]
        fn qquaternionFromEulerAngles(pitch: f32, yaw: f32, roll: f32) -> QQuaternion;
        #[doc(hidden)]
        #[rust_name = "qquaternion_from_rotation_matrix"]
        fn qquaternionFromRotationMatrix(rot3x3: &QMatrix3x3) -> QQuaternion;

        #[doc(hidden)]
        #[rust_name = "qquaternion_nlerp"]
        fn qquaternionNlerp(q1: &QQuaternion, q2: &QQuaternion, t: f32) -> QQuaternion;
        #[doc(hidden)]
        #[rust_name = "qquaternion_slerp"]
        fn qquaternionSlerp(q1: &QQuaternion, q2: &QQuaternion, t: f32) -> QQuaternion;

        #[doc(hidden)]
        #[rust_name = "qquaternion_rotation_to"]
        fn qquaternionRotationTo(from: &QVector3D, to: &QVector3D) -> QQuaternion;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qquaternion_init_qvector4d"]
        fn construct(vector: &QVector4D) -> QQuaternion;
        #[doc(hidden)]
        #[rust_name = "qquaternion_init_float_qvector3d"]
        fn construct(scalar: f32, vector: &QVector3D) -> QQuaternion;

        #[doc(hidden)]
        #[rust_name = "qquaternion_init_default"]
        fn construct() -> QQuaternion;

        #[doc(hidden)]
        #[rust_name = "qquaternion_to_debug_qstring"]
        fn toDebugQString(value: &QQuaternion) -> QString;
        #[doc(hidden)]
        #[rust_name = "qquaternion_plus"]
        fn operatorPlus(a: &QQuaternion, b: &QQuaternion) -> QQuaternion;
        #[doc(hidden)]
        #[rust_name = "qquaternion_minus"]
        fn operatorMinus(a: &QQuaternion, b: &QQuaternion) -> QQuaternion;
        #[doc(hidden)]
        #[rust_name = "qquaternion_mul"]
        fn operatorMul(a: f32, b: &QQuaternion) -> QQuaternion;
        #[doc(hidden)]
        #[rust_name = "qquaternion_div"]
        fn operatorDiv(a: f32, b: &QQuaternion) -> QQuaternion;
        #[doc(hidden)]
        #[rust_name = "qquaternion_neg"]
        fn operatorNeg(a: &QQuaternion) -> QQuaternion;
    }
}

/// The QQuaternion class represents a quaternion consisting of a vector and scalar.
///
/// Qt Documentation: [QQuaternion](https://doc.qt.io/qt/qquaternion.html#details)
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QQuaternion {
    wp: f32,
    xp: f32,
    yp: f32,
    zp: f32,
}

impl QQuaternion {
    /// Returns the dot product of `q1` and `q2`.
    pub fn dot_product(q1: &QQuaternion, q2: &QQuaternion) -> f32 {
        ffi::qquaternion_dot_product(q1, q2)
    }

    /// Interpolates along the shortest linear path between the rotational positions `q1` and `q2`. The value `t` should be between 0 and 1, indicating the distance to travel between `q1` and `q2`. The result will be [`normalized`](Self::normalized).
    ///
    /// If `t` is less than or equal to 0, then `q1` will be returned. If `t` is greater than or equal to 1, then `q2` will be returned.
    ///
    /// This function is typically faster than [`slerp`](Self::slerp) and will give approximate results to spherical interpolation that are good enough for some applications.
    pub fn nlerp(q1: &Self, q2: &Self, t: f32) -> Self {
        ffi::qquaternion_nlerp(q1, q2, t)
    }

    /// Returns the shortest arc quaternion to rotate from the direction described by the vector `from` to the direction described by the vector `to`.
    pub fn rotation_to(from: &QVector3D, to: &QVector3D) -> Self {
        ffi::qquaternion_rotation_to(from, to)
    }

    /// Interpolates along the shortest linear path between the rotational positions `q1` and `q2`. The value `t` should be between 0 and 1, indicating the distance to travel between `q1` and `q2`. The result will be [`normalized`](Self::normalized).
    ///
    /// If `t` is less than or equal to 0, then `q1` will be returned. If `t` is greater than or equal to 1, then `q2` will be returned.
    pub fn slerp(q1: &Self, q2: &Self, t: f32) -> Self {
        ffi::qquaternion_slerp(q1, q2, t)
    }

    /// Constructs a quaternion vector from the specified `vector` and `scalar`.
    pub fn new(scalar: f32, vector: &QVector3D) -> Self {
        ffi::qquaternion_init_float_qvector3d(scalar, vector)
    }

    /// Constructs the quaternion using 3 axes (`x_axis`, `y_axis`, `z_axis`).
    ///
    /// **Note:** The axes are assumed to be orthonormal.
    pub fn from_axes(x_axis: &QVector3D, y_axis: &QVector3D, z_axis: &QVector3D) -> Self {
        ffi::qquaternion_from_axes(x_axis, y_axis, z_axis)
    }

    /// Creates a normalized quaternion that corresponds to rotating through `angle` degrees about the specified 3D `axis`.
    pub fn from_axis_and_angle(x: f32, y: f32, z: f32, angle: f32) -> Self {
        ffi::qquaternion_from_axis_and_angle(x, y, z, angle)
    }

    /// Creates a quaternion that corresponds to a rotation of `roll` degrees around the z axis, `pitch` degrees around the x axis, and `yaw` degrees around the y axis (in that order).
    pub fn from_euler_angles(pitch: f32, yaw: f32, roll: f32) -> Self {
        ffi::qquaternion_from_euler_angles(pitch, yaw, roll)
    }

    /// Creates a quaternion that corresponds to a rotation matrix `rot3x3`.
    ///
    /// **Note:** If a given rotation matrix is not normalized, the resulting quaternion will contain scaling information.
    pub fn from_rotation_matrix(rot3x3: &QMatrix3x3) -> Self {
        ffi::qquaternion_from_rotation_matrix(rot3x3)
    }

    /// Returns the 3 orthonormal axes (`x_axis`, `y_axis`, `z_axis`) defining the quaternion.
    pub fn get_axes(&self) -> (QVector3D, QVector3D, QVector3D) {
        let mut x = MaybeUninit::uninit();
        let mut y = MaybeUninit::uninit();
        let mut z = MaybeUninit::uninit();
        unsafe {
            // SAFETY: All pointers are valid.
            self.get_axes_raw(x.as_mut_ptr(), y.as_mut_ptr(), z.as_mut_ptr());
            // SAFETY: Qt has initialized all values.
            (x.assume_init(), y.assume_init(), z.assume_init())
        }
    }

    /// Extracts a 3D axis and a rotating angle (in degrees) (`x`, `y`, `z`, `angle`) that corresponds to this quaternion.
    pub fn get_axis_and_angle(&self) -> (f32, f32, f32, f32) {
        let mut x = MaybeUninit::uninit();
        let mut y = MaybeUninit::uninit();
        let mut z = MaybeUninit::uninit();
        let mut angle = MaybeUninit::uninit();
        unsafe {
            // SAFETY: All pointers are valid.
            self.get_axis_and_angle_raw(
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                z.as_mut_ptr(),
                angle.as_mut_ptr(),
            );
            // SAFETY: Qt has initialized all values.
            (
                x.assume_init(),
                y.assume_init(),
                z.assume_init(),
                angle.assume_init(),
            )
        }
    }

    /// Calculates (`pitch`, `yaw`, `roll`) Euler angles (in degrees) that corresponds to this quaternion.
    pub fn get_euler_angles(&self) -> (f32, f32, f32) {
        let mut pitch = MaybeUninit::uninit();
        let mut yaw = MaybeUninit::uninit();
        let mut roll = MaybeUninit::uninit();
        unsafe {
            // SAFETY: All pointers are valid.
            self.get_euler_angles_raw(pitch.as_mut_ptr(), yaw.as_mut_ptr(), roll.as_mut_ptr());
            // SAFETY: Qt has initialized all values.
            (pitch.assume_init(), yaw.assume_init(), roll.assume_init())
        }
    }
}

impl Default for QQuaternion {
    /// Constructs an identity quaternion (1, 0, 0, 0), i.e. with the vector (0, 0, 0) and scalar 1.
    fn default() -> Self {
        ffi::qquaternion_init_default()
    }
}

impl fmt::Display for QQuaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qquaternion_to_debug_qstring(self).fmt(f)
    }
}

impl std::ops::Add for QQuaternion {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ffi::qquaternion_plus(&self, &other)
    }
}

impl std::ops::Sub for QQuaternion {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ffi::qquaternion_minus(&self, &other)
    }
}

impl std::ops::Mul<f32> for QQuaternion {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        ffi::qquaternion_mul(rhs, &self)
    }
}

impl std::ops::Div<f32> for QQuaternion {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        ffi::qquaternion_div(rhs, &self)
    }
}

impl std::ops::Neg for QQuaternion {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ffi::qquaternion_neg(&self)
    }
}

impl From<&QVector4D> for QQuaternion {
    /// Constructs a quaternion from the components of vector.
    fn from(value: &QVector4D) -> Self {
        ffi::qquaternion_init_qvector4d(value)
    }
}
impl From<QVector4D> for QQuaternion {
    /// Constructs a quaternion from the components of vector.
    fn from(value: QVector4D) -> Self {
        Self::from(&value)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QQuaternion is trivial.
unsafe impl ExternType for QQuaternion {
    type Id = type_id!("QQuaternion");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod test {
    use super::*;

    fn round(n: f32) -> i32 {
        n.round() as i32
    }

    #[test]
    fn get_axes() {
        let axis1 = QVector3D::new(1.0, 0.0, 0.0);
        let axis2 = QVector3D::new(0.0, 1.0, 0.0);
        let axis3 = QVector3D::new(0.0, 0.0, 1.0);
        let qq = QQuaternion::from_axes(&axis1, &axis2, &axis3);
        assert_eq!(qq.get_axes(), (axis1, axis2, axis3));
    }

    #[test]
    fn get_axis_and_angle() {
        let qq = QQuaternion::from_axis_and_angle(1.0, 0.0, 0.0, 40.0);
        let (a, b, c, d) = qq.get_axis_and_angle();
        assert_eq!((round(a), round(b), round(c), round(d)), (1, 0, 0, 40));
    }

    #[test]
    fn get_euler_angles() {
        let qq = QQuaternion::from_euler_angles(10.0, 20.0, 30.0);
        let (a, b, c) = qq.get_euler_angles();
        assert_eq!((round(a), round(b), round(c)), (10, 20, 30));
    }

    #[test]
    fn to_rotation_matrix() {
        let qq = QQuaternion::from_axis_and_angle(1.0, 0.0, 0.0, 40.0);
        let matrix = qq.to_rotation_matrix();
        assert_eq!(QQuaternion::from_rotation_matrix(&matrix), qq);
    }
}
