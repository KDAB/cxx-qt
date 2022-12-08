// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QVariant>

#include <QColor>
#include <QDate>
#include <QDateTime>
#include <QPoint>
#include <QPointF>
#include <QRect>
#include <QRectF>
#include <QSize>
#include <QSizeF>
#include <QString>
#include <QTime>
#include <QUrl>

#include "rust/cxx.h"

template<>
struct rust::IsRelocatable<QVariant> : std::true_type
{
};

namespace rust {
namespace cxxqtlib1 {

namespace types {

enum class QVariantType : uint8_t
{
  Unsupported = 0,
  Bool = 1,
  F32 = 2,
  F64 = 3,
  I8 = 4,
  I16 = 5,
  I32 = 6,
  QColor = 7,
  QDate = 8,
  QDateTime = 9,
  QPoint = 10,
  QPointF = 11,
  QRect = 12,
  QRectF = 13,
  QSize = 14,
  QSizeF = 15,
  QString = 16,
  QTime = 17,
  QUrl = 18,
  U8 = 19,
  U16 = 20,
  U32 = 21,
};

} // namespace types

template<typename T>
QVariant
qvariantInitFromT(T value)
{
  return QVariant::fromValue(value);
}

types::QVariantType
qvariantType(const QVariant& variant);

template<typename T>
T
qvariantToT(const QVariant& variant)
{
  Q_ASSERT(variant.canConvert<T>());
  return variant.value<T>();
}

}
}
