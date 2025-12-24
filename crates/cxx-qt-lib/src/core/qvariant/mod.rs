// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = super::QVariant;

        /// Convert this variant to type `QMetaType::UnknownType` and free up any resources used.
        fn clear(self: &mut QVariant);
        /// Returns `true` if this is a null variant, `false` otherwise.
        ///
        /// In Qt 6, a value is considered null if it contains no initialized value or a null pointer.
        /// In Qt 5, a value is additionally considered null if the variant contains an object of a builtin type with an `is_null` method that returned `true` for that object.
        #[rust_name = "is_null"]
        fn isNull(self: &QVariant) -> bool;
        /// Returns `true` if the storage type of this variant is not `QMetaType::UnknownType`; otherwise returns `false`.
        #[rust_name = "is_valid"]
        fn isValid(self: &QVariant) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qvariant_drop"]
        fn drop(variant: &mut QVariant);
        #[doc(hidden)]
        #[rust_name = "qvariant_default"]
        fn construct() -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_clone"]
        fn construct(variant: &QVariant) -> QVariant;
        #[doc(hidden)]
        #[rust_name = "qvariant_eq"]
        fn operatorEq(a: &QVariant, b: &QVariant) -> bool;
        #[doc(hidden)]
        #[rust_name = "qvariant_to_debug_qstring"]
        fn toDebugQString(variant: &QVariant) -> QString;
    }
}

/// The `QVariant` class acts like a union for the most common Qt data types.
///
/// Qt Documentation: [QVariant]("https://doc.qt.io/qt/qvariant.html#details")
#[repr(C)]
pub struct QVariant {
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 `QVariant` has one member, which contains three `uint`s (but they are optimised to a size of 8) and a union
    /// Qt6 `QVariant` has one member, which contains three pointers and a union (pointer largest)
    _data: MaybeUninit<f64>,

    // Compiler optimisations reduce the size of the `uint` to a `ushort`
    #[cfg(cxxqt_qt_version_major = "5")]
    _type: MaybeUninit<u16>,
    #[cfg(cxxqt_qt_version_major = "5")]
    _is_shared: MaybeUninit<u16>,
    #[cfg(cxxqt_qt_version_major = "5")]
    _is_null: MaybeUninit<u16>,

    #[cfg(cxxqt_qt_version_major = "6")]
    _is_shared: MaybeUninit<usize>,
    #[cfg(cxxqt_qt_version_major = "6")]
    _is_null: MaybeUninit<usize>,
    #[cfg(cxxqt_qt_version_major = "6")]
    _packed_type: MaybeUninit<usize>,
}

impl Clone for QVariant {
    /// Constructs a copy of the variant passed as the argument to `self`'s constructor.
    fn clone(&self) -> Self {
        ffi::qvariant_clone(self)
    }
}

impl Default for QVariant {
    /// Constructs an invalid variant.
    fn default() -> Self {
        ffi::qvariant_default()
    }
}

impl Drop for QVariant {
    /// Destroys the `QVariant` and the contained object.
    fn drop(&mut self) {
        ffi::qvariant_drop(self)
    }
}

impl<T> From<&T> for QVariant
where
    T: QVariantValue,
{
    /// Constructs a `QVariant` from a value of `T`.
    fn from(value: &T) -> Self {
        T::construct(value)
    }
}

// Note we can't use impl Into or TryInto for QVariant here as it conflicts
//
// note: conflicting implementation in crate `core`:
// - impl<T, U> TryInto<U> for T
//   where U: TryFrom<T>;
impl QVariant {
    /// Returns the stored value converted to the template type `T`, or `None` if the type cannot be converted to `T`.
    ///
    /// Note that this first calls [`can_convert`](QVariantValue::can_convert).
    pub fn value<T: QVariantValue>(&self) -> Option<T> {
        if T::can_convert(self) {
            Some(T::value_or_default(self))
        } else {
            None
        }
    }

    /// Returns the stored value converted to the template type `T`, or a default-constructed value if the type cannot be converted to `T`.
    ///
    /// For most value types, a default-constructed value simply means that a value is created using the default constructor (e.g. an empty string for [`QString`](crate::QString)). Primitive types like `i32` and `f64` are initialized to 0.
    ///
    /// Note that this calls Qt's `QVariant::value` method, without performance loss.
    /// Whereas `value` first calls [`can_convert`](QVariantValue::can_convert).
    pub fn value_or_default<T: QVariantValue>(&self) -> T {
        T::value_or_default(self)
    }
}

impl std::cmp::PartialEq for QVariant {
    /// Returns `true` if `self` and `other` are equal, otherwise returns `false`.
    ///
    /// `QVariant` uses the equality operator of the type contained to check for equality.
    ///
    /// Variants of different types will always compare as not equal with a few exceptions:
    ///
    /// - If both types are numeric types (integers and floatins point numbers) Qt will compare those types using standard C++ type promotion rules.
    /// - If one type is numeric and the other one a [`QString`](crate::QString), Qt will try to convert the `QString` to a matching numeric type and if successful compare those.
    /// - If both variants contain pointers to `QObject` derived types, `QVariant` will check whether the types are related and point to the same object.
    ///
    /// The result of the function is not affected by the result of [`is_null`](QVariant::is_null), which means that two values can be equal even if one of them is null and another is not.
    fn eq(&self, other: &Self) -> bool {
        ffi::qvariant_eq(self, other)
    }
}

impl fmt::Debug for QVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qvariant_to_debug_qstring(self).fmt(f)
    }
}

/// Trait implementation for a value in a [`QVariant`].
pub trait QVariantValue {
    fn can_convert(variant: &QVariant) -> bool;
    fn construct(value: &Self) -> QVariant;
    fn value_or_default(variant: &QVariant) -> Self;
}

macro_rules! impl_qvariant_value {
    ( $typeName:ty, $module:ident ) => {
        mod $module;

        impl QVariantValue for $typeName {
            fn can_convert(variant: &QVariant) -> bool {
                $module::can_convert(variant)
            }

            fn construct(value: &Self) -> QVariant {
                $module::construct(value)
            }

            fn value_or_default(variant: &QVariant) -> Self {
                $module::value_or_default(variant)
            }
        }
    };
}

impl_qvariant_value!(bool, qvariant_bool);
impl_qvariant_value!(f32, qvariant_f32);
impl_qvariant_value!(f64, qvariant_f64);
impl_qvariant_value!(i8, qvariant_i8);
impl_qvariant_value!(i16, qvariant_i16);
impl_qvariant_value!(i32, qvariant_i32);
impl_qvariant_value!(i64, qvariant_i64);
impl_qvariant_value!(crate::QByteArray, qvariant_qbytearray);
impl_qvariant_value!(crate::QDate, qvariant_qdate);
#[cfg(not(target_os = "emscripten"))]
impl_qvariant_value!(crate::QDateTime, qvariant_qdatetime);
impl_qvariant_value!(crate::QLine, qvariant_qline);
impl_qvariant_value!(crate::QLineF, qvariant_qlinef);
impl_qvariant_value!(crate::QModelIndex, qvariant_qmodelindex);
impl_qvariant_value!(crate::QPersistentModelIndex, qvariant_qpersistentmodelindex);
impl_qvariant_value!(crate::QPoint, qvariant_qpoint);
impl_qvariant_value!(crate::QPointF, qvariant_qpointf);
impl_qvariant_value!(crate::QRect, qvariant_qrect);
impl_qvariant_value!(crate::QRectF, qvariant_qrectf);
impl_qvariant_value!(crate::QSize, qvariant_qsize);
impl_qvariant_value!(crate::QSizeF, qvariant_qsizef);
impl_qvariant_value!(crate::QString, qvariant_qstring);
impl_qvariant_value!(crate::QStringList, qvariant_qstringlist);
impl_qvariant_value!(crate::QTime, qvariant_qtime);
impl_qvariant_value!(crate::QUrl, qvariant_qurl);
impl_qvariant_value!(crate::QUuid, qvariant_quuid);
impl_qvariant_value!(
    crate::QHash<crate::QHashPair_QString_QVariant>,
    qvariant_qvarianthash
);
impl_qvariant_value!(crate::QList<QVariant>, qvariant_qvariantlist);
impl_qvariant_value!(
    crate::QMap<crate::QMapPair_QString_QVariant>,
    qvariant_qvariantmap
);
impl_qvariant_value!(u8, qvariant_u8);
impl_qvariant_value!(u16, qvariant_u16);
impl_qvariant_value!(u32, qvariant_u32);
impl_qvariant_value!(u64, qvariant_u64);

#[cfg(feature = "qt_gui")]
impl_qvariant_value!(crate::QColor, qvariant_qcolor);
#[cfg(feature = "qt_gui")]
impl_qvariant_value!(crate::QFont, qvariant_qfont);
#[cfg(feature = "qt_gui")]
impl_qvariant_value!(crate::QImage, qvariant_qimage);
#[cfg(feature = "qt_gui")]
impl_qvariant_value!(crate::QPen, qvariant_qpen);
#[cfg(feature = "qt_gui")]
impl_qvariant_value!(crate::QPolygon, qvariant_qpolygon);
#[cfg(feature = "qt_gui")]
impl_qvariant_value!(crate::QPolygonF, qvariant_qpolygonf);
#[cfg(feature = "qt_gui")]
impl_qvariant_value!(crate::QQuaternion, qvariant_qquaternion);
#[cfg(feature = "qt_gui")]
impl_qvariant_value!(crate::QRegion, qvariant_qregion);
#[cfg(feature = "qt_gui")]
impl_qvariant_value!(crate::QVector2D, qvariant_qvector2d);
#[cfg(feature = "qt_gui")]
impl_qvariant_value!(crate::QVector3D, qvariant_qvector3d);
#[cfg(feature = "qt_gui")]
impl_qvariant_value!(crate::QVector4D, qvariant_qvector4d);

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QVariant {
    type Id = type_id!("QVariant");
    type Kind = cxx::kind::Trivial;
}
