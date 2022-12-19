// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGlobal>

// Qt 5 has a different QList<T>
#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))
#include <cstdint>

#include <QtCore/QList>

#include <QtCore/QDate>
#include <QtCore/QDateTime>
#include <QtCore/QPoint>
#include <QtCore/QPointF>
#include <QtCore/QRect>
#include <QtCore/QRectF>
#include <QtCore/QSize>
#include <QtCore/QSizeF>
#include <QtCore/QString>
#include <QtCore/QTime>
#include <QtCore/QUrl>
#include <QtCore/QVariant>
#include <QtGui/QColor>

#include "rust/cxx.h"

// This has static asserts in the cpp file to ensure this is valid.
template<typename T>
struct rust::IsRelocatable<QList<T>> : std::true_type
{
};

using Qt5List_bool = QList<bool>;
using Qt5List_f32 = QList<float>;
using Qt5List_f64 = QList<double>;
using Qt5List_i8 = QList<::std::int8_t>;
using Qt5List_i16 = QList<::std::int16_t>;
using Qt5List_i32 = QList<::std::int32_t>;
using Qt5List_i64 = QList<::std::int64_t>;
using Qt5List_QColor = QList<::QColor>;
using Qt5List_QDate = QList<::QDate>;
using Qt5List_QDateTime = QList<::QDateTime>;
using Qt5List_QPoint = QList<::QPoint>;
using Qt5List_QPointF = QList<::QPointF>;
using Qt5List_QRect = QList<::QRect>;
using Qt5List_QRectF = QList<::QRectF>;
using Qt5List_QSize = QList<::QSize>;
using Qt5List_QSizeF = QList<::QSizeF>;
using Qt5List_QString = QList<::QString>;
using Qt5List_QTime = QList<::QTime>;
using Qt5List_QUrl = QList<::QUrl>;
using Qt5List_QVariant = QList<::QVariant>;
using Qt5List_u8 = QList<::std::uint8_t>;
using Qt5List_u16 = QList<::std::uint16_t>;
using Qt5List_u32 = QList<::std::uint32_t>;
using Qt5List_u64 = QList<::std::uint64_t>;

#endif
