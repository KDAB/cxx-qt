// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
mod ffi {
    /// Types supported by `QMetaType`. This enum list is non-exhaustive, as additional types may
    /// be registered during runtime.
    ///
    /// User-registered types should be greater than or equal to [`QMetaTypeType::User`].
    #[namespace = "rust::cxxqtlib1"]
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

    #[namespace = "rust::cxxqtlib1"]
    extern "C++" {
        include!("cxx-qt-lib/qmetatype.h");
        type QMetaTypeType;
    }
}

pub use ffi::QMetaTypeType;
