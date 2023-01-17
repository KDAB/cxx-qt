// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cstdint>

#include <QtCore/QVariant>

#include <QtCore/QDate>
#include <QtCore/QDateTime>
#include <QtCore/QPersistentModelIndex>
#include <QtCore/QPoint>
#include <QtCore/QPointF>
#include <QtCore/QRect>
#include <QtCore/QRectF>
#include <QtCore/QSize>
#include <QtCore/QSizeF>
#include <QtCore/QString>
#include <QtCore/QTime>
#include <QtCore/QUrl>

#ifdef CXX_QT_GUI_FEATURE
#include <QtGui/QColor>
#endif

#include "rust/cxx.h"

template<>
struct rust::IsRelocatable<QVariant> : ::std::true_type
{
};

namespace rust {
namespace cxxqtlib1 {
namespace qvariant {

template<typename T>
QVariant
qvariantConstruct(const T& value) noexcept
{
  return QVariant::fromValue<T>(value);
}

template<typename T>
T
qvariantValueOrDefault(const QVariant& variant) noexcept
{
  return variant.value<T>();
}

// Need to use a macro here as we can't template because the types
// are always QVariant and bool. So then CXX can't decide which to use.
#define CXX_QT_QVARIANT_CAN_CONVERT(name)                                      \
  bool qvariantCanConvert##name(const QVariant& variant);

CXX_QT_QVARIANT_CAN_CONVERT(Bool)
CXX_QT_QVARIANT_CAN_CONVERT(F32)
CXX_QT_QVARIANT_CAN_CONVERT(F64)
CXX_QT_QVARIANT_CAN_CONVERT(I8)
CXX_QT_QVARIANT_CAN_CONVERT(I16)
CXX_QT_QVARIANT_CAN_CONVERT(I32)
CXX_QT_QVARIANT_CAN_CONVERT(I64)
#ifdef CXX_QT_GUI_FEATURE
CXX_QT_QVARIANT_CAN_CONVERT(QColor)
#endif
CXX_QT_QVARIANT_CAN_CONVERT(QDate)
CXX_QT_QVARIANT_CAN_CONVERT(QDateTime)
CXX_QT_QVARIANT_CAN_CONVERT(QPersistentModelIndex)
CXX_QT_QVARIANT_CAN_CONVERT(QPoint)
CXX_QT_QVARIANT_CAN_CONVERT(QPointF)
CXX_QT_QVARIANT_CAN_CONVERT(QRect)
CXX_QT_QVARIANT_CAN_CONVERT(QRectF)
CXX_QT_QVARIANT_CAN_CONVERT(QSize)
CXX_QT_QVARIANT_CAN_CONVERT(QSizeF)
CXX_QT_QVARIANT_CAN_CONVERT(QString)
CXX_QT_QVARIANT_CAN_CONVERT(QTime)
CXX_QT_QVARIANT_CAN_CONVERT(QUrl)
CXX_QT_QVARIANT_CAN_CONVERT(U8)
CXX_QT_QVARIANT_CAN_CONVERT(U16)
CXX_QT_QVARIANT_CAN_CONVERT(U32)
CXX_QT_QVARIANT_CAN_CONVERT(U64)

}
}
}
