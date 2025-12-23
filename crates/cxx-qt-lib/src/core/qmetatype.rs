// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use std::ffi::CStr;
use std::fmt;
use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    /// Types supported by [`QMetaType`]. This enum list is non-exhaustive, as additional types may
    /// be registered during runtime.
    ///
    /// User-registered types should be greater than or equal to [`QMetaTypeType::User`].
    #[repr(i32)]
    #[derive(Debug)]
    enum QMetaTypeType {
        /// This is an invalid type id. It is returned for types that are not registered.
        UnknownType = 0,
        Bool = 1,
        Int = 2,
        UInt = 3,
        LongLong = 4,
        ULongLong = 5,
        Double = 6,
        Long = 32,
        Short = 33,
        Char = 34,
        ULong = 35,
        UShort = 36,
        UChar = 37,
        Float = 38,
        VoidStar = 31,
        QChar = 7,
        QString = 10,
        QStringList = 11,
        QByteArray = 12,
        QBitArray = 13,
        QDate = 14,
        QTime = 15,
        QDateTime = 16,
        QUrl = 17,
        QLocale = 18,
        QRect = 19,
        QRectF = 20,
        QSize = 21,
        QSizeF = 22,
        QLine = 23,
        QLineF = 24,
        QPoint = 25,
        QPointF = 26,
        QEasingCurve = 29,
        QUuid = 30,
        QVariant = 41,
        QModelIndex = 42,
        QPersistentModelIndex = 50,
        QRegularExpression = 44,
        QJsonValue = 45,
        QJsonObject = 46,
        QJsonArray = 47,
        QJsonDocument = 48,
        QByteArrayList = 49,
        QObjectStar = 39,
        SChar = 40,
        Void = 43,
        Nullptr = 51,
        QVariantMap = 8,
        QVariantList = 9,
        QVariantHash = 28,
        QCborSimpleType = 52,
        QCborValue = 53,
        QCborArray = 54,
        QCborMap = 55,
        Char16 = 56,
        Char32 = 57,

        // Gui types
        QFont = 0x1000,
        QPixmap = 0x1001,
        QBrush = 0x1002,
        QColor = 0x1003,
        QPalette = 0x1004,
        QIcon = 0x1005,
        QImage = 0x1006,
        QPolygon = 0x1007,
        QRegion = 0x1008,
        QBitmap = 0x1009,
        QCursor = 0x100a,
        QKeySequence = 0x100b,
        QPen = 0x100c,
        QTextLength = 0x100d,
        QTextFormat = 0x100e,
        QTransform = 0x1010,
        QMatrix4x4 = 0x1011,
        QVector2D = 0x1012,
        QVector3D = 0x1013,
        QVector4D = 0x1014,
        QQuaternion = 0x1015,
        QPolygonF = 0x1016,
        QColorSpace = 0x1017,

        // Widget types
        QSizePolicy = 0x2000,
        User = 65536,
    }

    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
    }

    extern "C++" {
        include!("cxx-qt-lib/qmetatype.h");
        type QMetaTypeType;
    }

    unsafe extern "C++" {
        type QMetaType = super::QMetaType;

        #[doc(hidden)]
        #[rust_name = "name_ptr"]
        fn name(&self) -> *const c_char;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[rust_name = "qmetatype_id"]
        fn qmetatypeId(meta_type: &QMetaType) -> i32;
        #[rust_name = "qmetatype_is_registered"]
        fn qmetatypeIsRegistered(meta_type: &QMetaType) -> bool;
        #[rust_name = "qmetatype_is_valid"]
        fn qmetatypeIsValid(meta_type: &QMetaType) -> bool;
        #[rust_name = "qmetatype_from_name"]
        fn qmetatypeFromName(type_name: &[u8]) -> QMetaType;
        #[rust_name = "qmetatype_can_convert"]
        fn qmetatypeCanConvert(from_type: QMetaType, to_type: QMetaType) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qmetatype_drop"]
        fn drop(proxy: &mut QMetaType);

        #[rust_name = "qmetatype_init_default"]
        fn construct() -> QMetaType;
        #[rust_name = "qmetatype_init_from_type"]
        fn construct(type_id: i32) -> QMetaType;
        #[rust_name = "qmetatype_clone"]
        fn construct(other: &QMetaType) -> QMetaType;

        #[rust_name = "qmetatype_eq"]
        fn operatorEq(a: &QMetaType, b: &QMetaType) -> bool;

        #[cfg(cxxqt_qt_version_at_least_6_5)]
        #[rust_name = "qmetatype_to_debug_qstring"]
        fn toDebugQString(value: &QMetaType) -> QString;
    }
}

pub use ffi::QMetaTypeType;

impl From<i32> for QMetaTypeType {
    fn from(value: i32) -> Self {
        Self { repr: value }
    }
}
impl From<QMetaTypeType> for i32 {
    fn from(value: QMetaTypeType) -> Self {
        value.repr
    }
}

/// The `QMetaType` class manages named types in the meta-object system.
///
/// Qt Documentation: [QMetaType](https://doc.qt.io/qt-6/qmetatype.html#details)
#[repr(C)]
pub struct QMetaType {
    _space: MaybeUninit<usize>,
}

impl Clone for QMetaType {
    fn clone(&self) -> Self {
        ffi::qmetatype_clone(self)
    }
}

impl Default for QMetaType {
    /// Constructs a default, invalid, `QMetaType` object.
    #[cfg(cxxqt_qt_version_major = "6")]
    fn default() -> Self {
        ffi::qmetatype_init_default()
    }
}

impl Drop for QMetaType {
    fn drop(&mut self) {
        ffi::qmetatype_drop(self);
    }
}

impl PartialEq for QMetaType {
    fn eq(&self, other: &Self) -> bool {
        ffi::qmetatype_eq(self, other)
    }
}

impl Eq for QMetaType {}

impl fmt::Debug for QMetaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(cxxqt_qt_version_at_least_6_5)]
        return ffi::qmetatype_to_debug_qstring(self).fmt(f);
        #[cfg(not(cxxqt_qt_version_at_least_6_5))]
        return write!(f, "QMetaType({})", self.id().repr);
    }
}

impl fmt::Display for QMetaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = self.name() {
            name.to_string_lossy().fmt(f)
        } else {
            write!(f, "unregistered type")
        }
    }
}

impl From<QMetaTypeType> for QMetaType {
    /// Constructs a `QMetaType` object that contains all information about metatype `type_id`.
    fn from(type_id: QMetaTypeType) -> Self {
        ffi::qmetatype_init_from_type(type_id.repr)
    }
}

impl QMetaType {
    /// Returns `true` if `QMetaType` can convert from `from_type` to `to_type`.
    pub fn can_convert(from_type: QMetaType, to_type: QMetaType) -> bool {
        ffi::qmetatype_can_convert(from_type, to_type)
    }

    /// Constructs a `QMetaType` object that contains all information about metatype `type_id`.
    pub fn new(type_id: QMetaTypeType) -> Self {
        Self::from(type_id)
    }

    /// Constructs a `QMetaType` object that contains all information about a type.
    pub fn from_type<T>() -> Self
    where
        T: QMetaTyped,
    {
        Self::from(T::meta_type())
    }

    /// Returns a `QMetaType` matching `type_name`. Returns `None` if the `type_name` is not known to `QMetaType`.
    pub fn from_name<S>(type_name: S) -> Option<Self>
    where
        S: AsRef<[u8]>,
    {
        let meta_type = ffi::qmetatype_from_name(type_name.as_ref());
        if meta_type.is_valid() {
            Some(meta_type)
        } else {
            None
        }
    }

    /// Returns id type held by this `QMetaType` instance.
    pub fn id(&self) -> QMetaTypeType {
        QMetaTypeType {
            repr: ffi::qmetatype_id(self),
        }
    }

    /// Returns `true` if this `QMetaType` object has been registered with the Qt global metatype registry. Registration allows the type to be found by its name (using [`QMetaType::from_name`]) or by its ID (using [`QMetaType::new`]).
    pub fn is_registered(&self) -> bool {
        ffi::qmetatype_is_registered(self)
    }

    /// Returns `true` if this QMetaType object contains valid information about a type, `false` otherwise.
    pub fn is_valid(&self) -> bool {
        ffi::qmetatype_is_valid(self)
    }

    /// Returns the type name associated with this `QMetaType`, or `None` if no matching type was found.
    pub fn name(&self) -> Option<&CStr> {
        let name_ptr = self.name_ptr();
        if name_ptr.is_null() {
            return None;
        }
        // SAFETY: `name_ptr` is valid and non-null, and will remain so for this object's lifetime.
        Some(unsafe { CStr::from_ptr(name_ptr) })
    }
}

impl QMetaTypeType {
    /// Returns the type ID associated with a given type.
    ///
    /// # Example
    ///
    /// ```
    /// # use cxx_qt_lib::QMetaTypeType;
    /// # fn main() {
    /// # cxx_qt::init_crate!(cxx_qt_lib);
    /// assert_eq!(QMetaTypeType::of::<bool>(), QMetaTypeType::Bool);
    /// # }
    /// ```
    pub fn of<T>() -> Self
    where
        T: QMetaTyped,
    {
        T::meta_type()
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QMetaType {
    type Id = type_id!("QMetaType");
    type Kind = cxx::kind::Trivial;
}

/// Types that are registeed with Qt's meta-type system.
pub trait QMetaTyped {
    fn meta_type() -> crate::QMetaTypeType;
}

macro_rules! impl_typed {
    ($($t:ty => $i:ident),* $(,)?) => {
        $(impl QMetaTyped for $t {
            fn meta_type() -> QMetaTypeType {
                QMetaTypeType::$i
            }
        })*
    };
}

impl_typed! {
    () => Void,
    bool => Bool,
    i32 => Int,
    u32 => UInt,
    f64 => Double,
    crate::QString => QString,
    crate::QByteArray => QByteArray,
    i64 => LongLong,
    i16 => Short,
    i8 => SChar,
    u8 => UChar,
    f32 => Float,
    crate::QList<crate::QByteArray> => QByteArrayList,
    crate::QDate => QDate,
    crate::QLine => QLine,
    crate::QLineF => QLineF,
    crate::QModelIndex => QModelIndex,
    crate::QPersistentModelIndex => QPersistentModelIndex,
    crate::QPoint => QPoint,
    crate::QPointF => QPointF,
    crate::QRect => QRect,
    crate::QSize => QSize,
    crate::QSizeF => QSizeF,
    crate::QStringList => QStringList,
    crate::QList<crate::QString> => QStringList,
    crate::QTime => QTime,
    crate::QUrl => QUrl,
    crate::QUuid => QUuid,
    crate::QVariant => QVariant,
    crate::QHash<crate::QHashPair_QString_QVariant> => QVariantHash,
    crate::QList<crate::QVariant> => QVariantList,
    crate::QMap<crate::QMapPair_QString_QVariant> => QVariantMap,
}

#[cfg(not(target_os = "emscripten"))]
impl_typed! {
    crate::QDateTime => QDateTime,
}

#[cfg(feature = "qt_gui")]
impl_typed! {
    crate::QColor => QColor,
    crate::QFont => QFont,
    crate::QGenericMatrix<4, 4> => QMatrix4x4,
    crate::QImage => QImage,
    crate::QPen => QPen,
    crate::QPolygon => QPolygon,
    crate::QPolygonF => QPolygonF,
    crate::QQuaternion => QQuaternion,
    crate::QRegion => QRegion,
    crate::QVector2D => QVector2D,
    crate::QVector3D => QVector3D,
    crate::QVector4D => QVector4D,
}
