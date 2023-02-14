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
        type QVector3D = super::QVector3D;
        include!("cxx-qt-lib/qvector4d.h");
        type QVector4D = crate::QVector4D;

        /// Returns true if the x, y, and z coordinates are set to 0.0, otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QVector3D) -> bool;

        /// Returns the length of the vector from the origin.
        fn length(self: &QVector3D) -> f32;

        /// Returns the squared length of the vector from the origin.
        /// This is equivalent to the dot product of the vector with itself.
        #[rust_name = "length_squared"]
        fn lengthSquared(self: &QVector3D) -> f32;

        /// Normalizes the currect vector in place. Nothing happens if this vector is a null vector
        /// or the length of the vector is very close to 1.
        fn normalize(self: &mut QVector3D);

        /// Returns the normalized unit vector form of this vector.
        ///
        /// If this vector is null, then a null vector is returned.
        /// If the length of the vector is very close to 1, then the vector will be returned as-is.
        /// Otherwise the normalized form of the vector of length 1 will be returned.
        fn normalized(self: &QVector3D) -> QVector3D;

        /// Sets the x coordinate of this point to the given finite x coordinate.
        #[rust_name = "set_x"]
        fn setX(self: &mut QVector3D, x: f32);
        /// Sets the y coordinate of this point to the given finite y coordinate.
        #[rust_name = "set_y"]
        fn setY(self: &mut QVector3D, y: f32);
        /// Sets the z coordinate of this point to the given finite z coordinate.
        #[rust_name = "set_z"]
        fn setZ(self: &mut QVector3D, z: f32);

        // From trait is more idiomatic to Rust and implemented in QPoint and QPointF
        #[doc(hidden)]
        #[rust_name = "to_point"]
        fn toPoint(self: &QVector3D) -> QPoint;
        #[doc(hidden)]
        #[rust_name = "to_pointf"]
        fn toPointF(self: &QVector3D) -> QPointF;

        /// Returns the x coordinate of this point.
        fn x(self: &QVector3D) -> f32;
        /// Returns the y coordinate of this point.
        fn y(self: &QVector3D) -> f32;
        /// Returns the z coordinate of this point.
        fn z(self: &QVector3D) -> f32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qvector3d_init_qvector4d"]
        fn construct(vector: QVector4D) -> QVector3D;
        #[doc(hidden)]
        #[rust_name = "qvector3d_init_qvector2d"]
        fn construct(vector: QVector2D) -> QVector3D;

        #[doc(hidden)]
        #[rust_name = "qvector3d_init_qpointf"]
        fn construct(point: QPointF) -> QVector3D;
        #[doc(hidden)]
        #[rust_name = "qvector3d_init_qpoint"]
        fn construct(point: QPoint) -> QVector3D;

        #[doc(hidden)]
        #[rust_name = "qvector3d_init"]
        fn construct(x: f32, y: f32, z: f32) -> QVector3D;

        #[doc(hidden)]
        #[rust_name = "qvector3d_init_default"]
        fn construct() -> QVector3D;

        // Note that Qt 5 takes const-ref and Qt 6 takes by-value
        //
        // We want by-value, as that is Rust-idiomatic, so for Qt 5 we create a proxy
        #[doc(hidden)]
        #[rust_name = "qvector3d_distance_to_line"]
        fn qvector3DDistanceToLine(
            vector: &QVector3D,
            point: QVector3D,
            direction: QVector3D,
        ) -> f32;
        #[doc(hidden)]
        #[rust_name = "qvector3d_distance_to_plane"]
        fn qvector3DDistanceToPlane(vector: &QVector3D, point: QVector3D, normal: QVector3D)
            -> f32;
        #[doc(hidden)]
        #[rust_name = "qvector3d_distance_to_point"]
        fn qvector3DDistanceToPoint(vector: &QVector3D, point: QVector3D) -> f32;
        #[doc(hidden)]
        #[rust_name = "qvector3d_to_qstring"]
        fn toQString(value: &QVector3D) -> QString;
    }
}

/// The QVector3D class represents a vector or vertex in 3D space.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct QVector3D {
    v: [f32; 3],
}

impl QVector3D {
    /// Constructs a vector with coordinates (xpos, ypos, zpos). All parameters must be finite.
    pub fn new(xpos: f32, ypos: f32, zpos: f32) -> Self {
        ffi::qvector3d_init(xpos, ypos, zpos)
    }

    /// Returns the distance that this vertex is from a line defined by point and the unit vector direction.
    ///
    /// If direction is a null vector, then it does not define a line.
    /// In that case, the distance from point to this vertex is returned.
    pub fn distance_to_line(&self, point: QVector3D, direction: QVector3D) -> f32 {
        ffi::qvector3d_distance_to_line(self, point, direction)
    }

    /// Returns the distance from this vertex to a plane defined by the vertex plane and a normal unit vector. The normal parameter is assumed to have been normalized to a unit vector.
    ///
    /// The return value will be negative if the vertex is below the plane, or zero if it is on the plane.
    pub fn distance_to_plane(&self, point: QVector3D, normal: QVector3D) -> f32 {
        ffi::qvector3d_distance_to_plane(self, point, normal)
    }

    /// Returns the distance from this vertex to a point defined by the vertex point.
    pub fn distance_to_point(&self, point: QVector3D) -> f32 {
        ffi::qvector3d_distance_to_point(self, point)
    }
}

impl Default for QVector3D {
    /// Constructs a null vector, i.e. with coordinates (0, 0, 0).
    fn default() -> Self {
        ffi::qvector3d_init_default()
    }
}

impl std::fmt::Display for QVector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", ffi::qvector3d_to_qstring(self))
    }
}

impl From<crate::QVector4D> for QVector3D {
    /// Constructs a 3D vector from the specified 4D vector.
    /// The w coordinate is dropped.
    fn from(value: crate::QVector4D) -> Self {
        ffi::qvector3d_init_qvector4d(value)
    }
}

impl From<crate::QVector2D> for QVector3D {
    /// Constructs a 3D vector from the specified 2D vector.
    /// The z coordinate is set to zero.
    fn from(value: crate::QVector2D) -> Self {
        ffi::qvector3d_init_qvector2d(value)
    }
}

impl From<crate::QPointF> for QVector3D {
    /// Constructs a vector with x and y coordinates from a 2D point, and a z coordinate of 0.
    fn from(value: crate::QPointF) -> Self {
        ffi::qvector3d_init_qpointf(value)
    }
}

impl From<QVector3D> for crate::QPointF {
    /// Returns the QPointF form of this 3D vector. The z coordinate is dropped.
    fn from(value: QVector3D) -> Self {
        value.to_pointf()
    }
}

impl From<crate::QPoint> for QVector3D {
    /// Constructs a vector with x and y coordinates from a 2D point, and a z coordinate of 0.
    fn from(value: crate::QPoint) -> Self {
        ffi::qvector3d_init_qpoint(value)
    }
}

impl From<QVector3D> for crate::QPoint {
    /// Returns the QPoint form of this 3D vector. The z coordinate is dropped.
    fn from(value: QVector3D) -> Self {
        value.to_point()
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QVector3D is trivial.
unsafe impl ExternType for QVector3D {
    type Id = type_id!("QVector3D");
    type Kind = cxx::kind::Trivial;
}

#[cfg(feature = "serde")]
use serde::ser::SerializeTuple;

#[cfg(feature = "serde")]
struct QVector3DVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for QVector3DVisitor {
    type Value = QVector3D;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("QVector3D")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        if let Some(size_hint) = seq.size_hint() {
            if size_hint != 3 {
                return Err(serde::de::Error::invalid_length(
                    size_hint,
                    &"a tuple of three numbers",
                ));
            }
        }

        let x = if let Some(value) = seq.next_element::<f32>()? {
            value
        } else {
            return Err(serde::de::Error::invalid_length(
                0,
                &"a tuple of three numbers",
            ));
        };
        let y = if let Some(value) = seq.next_element::<f32>()? {
            value
        } else {
            return Err(serde::de::Error::invalid_length(
                1,
                &"a tuple of three numbers",
            ));
        };
        let z = if let Some(value) = seq.next_element::<f32>()? {
            value
        } else {
            return Err(serde::de::Error::invalid_length(
                2,
                &"a tuple of three numbers",
            ));
        };

        Ok(QVector3D::new(x, y, z))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QVector3D {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(3, QVector3DVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QVector3D {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_tuple(3)?;
        seq.serialize_element(&self.x())?;
        seq.serialize_element(&self.y())?;
        seq.serialize_element(&self.z())?;
        seq.end()
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serde_deserialize() {
        let test_data: QVector3D = serde_json::from_str(r#"[1.0,2.0,3.0]"#).unwrap();
        assert_eq!(test_data, QVector3D::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_serde_serialize() {
        let test_data = QVector3D::new(1.0, 2.0, 3.0);
        let data_string = serde_json::to_string(&test_data).unwrap();
        assert_eq!(data_string, r#"[1.0,2.0,3.0]"#);
    }
}
