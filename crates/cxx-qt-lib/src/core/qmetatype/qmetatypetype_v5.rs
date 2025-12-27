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
        QRegExp = 27,
        QEasingCurve = 29,
        QUuid = 30,
        QVariant = 41,
        QModelIndex = 42,
        QRegularExpression = 44,
        QJsonValue = 45,
        QJsonObject = 46,
        QJsonArray = 47,
        QJsonDocument = 48,
        QObjectStar = 39,
        SChar = 40,
        Void = 43,
        QVariantMap = 8,
        QVariantList = 9,
        QVariantHash = 28,

        // Gui types
        QFont = 64,
        QPixmap = 65,
        QBrush = 66,
        QColor = 67,
        QPalette = 68,
        QIcon = 69,
        QImage = 70,
        QPolygon = 71,
        QRegion = 72,
        QBitmap = 73,
        QCursor = 74,
        QKeySequence = 75,
        QPen = 76,
        QTextLength = 77,
        QTextFormat = 78,
        QMatrix = 79,
        QTransform = 80,
        QMatrix4x4 = 81,
        QVector2D = 82,
        QVector3D = 83,
        QVector4D = 84,
        QQuaternion = 85,
        QPolygonF = 86,
        QColorSpace = 87,

        // Widget types
        QSizePolicy = 121,
        User = 1024,
    }

    extern "C++" {
        include!("cxx-qt-lib/qmetatype.h");
        type QMetaTypeType;
    }
}

pub use ffi::QMetaTypeType;
