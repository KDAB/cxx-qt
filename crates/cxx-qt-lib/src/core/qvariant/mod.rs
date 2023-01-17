// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cfg(feature = "qt_gui")]
use crate::QColor;
use crate::{
    QDate, QDateTime, QPersistentModelIndex, QPoint, QPointF, QRect, QRectF, QSize, QSizeF,
    QString, QTime, QUrl,
};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = super::QVariant;

        /// Convert this variant to type QMetaType::UnknownType and free up any resources used.
        fn clear(self: &mut QVariant);
        /// Returns true if this is a null variant, false otherwise.
        #[rust_name = "is_null"]
        fn isNull(self: &QVariant) -> bool;
        /// Returns true if the storage type of this variant is not QMetaType::UnknownType; otherwise returns false.
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
    }
}

/// The QVariant class acts like a union for the most common Qt data types.
#[repr(C)]
pub struct QVariant {
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QVariant has one member, which contains three uints (but they are optimised to a size of 8) and a union
    /// Qt6 QVariant has one member, which contains three pointers and a union (pointer largest)
    _data: MaybeUninit<f64>,

    // Compiler optimisations reduce the size of the uint to a ushort
    #[cfg(qt_version_major = "5")]
    _type: MaybeUninit<u16>,
    #[cfg(qt_version_major = "5")]
    _is_shared: MaybeUninit<u16>,
    #[cfg(qt_version_major = "5")]
    _is_null: MaybeUninit<u16>,

    #[cfg(qt_version_major = "6")]
    _is_shared: MaybeUninit<usize>,
    #[cfg(qt_version_major = "6")]
    _is_null: MaybeUninit<usize>,
    #[cfg(qt_version_major = "6")]
    _packed_type: MaybeUninit<usize>,
}

impl Clone for QVariant {
    /// Constructs a copy of the variant, p, passed as the argument to this constructor.
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
    /// Destroys the QVariant and the contained object.
    fn drop(&mut self) {
        ffi::qvariant_drop(self)
    }
}

impl<T> From<&T> for QVariant
where
    T: QVariantValue,
{
    /// Constructs a QVariant from a value of T
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
    /// Returns the stored value converted to the template type T
    /// if QVariant::canConvert is true otherwise returns None
    pub fn value<T: QVariantValue>(&self) -> Option<T> {
        if T::can_convert(self) {
            Some(T::value_or_default(self))
        } else {
            None
        }
    }

    /// Returns the stored value converted to the template type T
    ///
    /// If the value cannot be converted, a default-constructed value will be returned.
    ///
    /// Note that this calls the `QVariant::value` method, without performance loss.
    /// Whereas `value` first calls `QVariant::canConvert`.
    pub fn value_or_default<T: QVariantValue>(&self) -> T {
        T::value_or_default(self)
    }
}

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
#[cfg(feature = "qt_gui")]
impl_qvariant_value!(QColor, qvariant_qcolor);
impl_qvariant_value!(QDate, qvariant_qdate);
impl_qvariant_value!(QDateTime, qvariant_qdatetime);
impl_qvariant_value!(QPersistentModelIndex, qvariant_qpersistentmodelindex);
impl_qvariant_value!(QPoint, qvariant_qpoint);
impl_qvariant_value!(QPointF, qvariant_qpointf);
impl_qvariant_value!(QRect, qvariant_qrect);
impl_qvariant_value!(QRectF, qvariant_qrectf);
impl_qvariant_value!(QSize, qvariant_qsize);
impl_qvariant_value!(QSizeF, qvariant_qsizef);
impl_qvariant_value!(QString, qvariant_qstring);
impl_qvariant_value!(QTime, qvariant_qtime);
impl_qvariant_value!(QUrl, qvariant_qurl);
impl_qvariant_value!(u8, qvariant_u8);
impl_qvariant_value!(u16, qvariant_u16);
impl_qvariant_value!(u32, qvariant_u32);
impl_qvariant_value!(u64, qvariant_u64);

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QVariant {
    type Id = type_id!("QVariant");
    type Kind = cxx::kind::Trivial;
}
