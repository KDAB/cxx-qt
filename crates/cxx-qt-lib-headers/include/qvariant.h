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

QVariant
qvariantInitFromBool(bool b);
QVariant
qvariantInitFromF32(float f32);
QVariant
qvariantInitFromF64(double f64);
QVariant
qvariantInitFromI8(qint8 i8);
QVariant
qvariantInitFromI16(qint16 i16);
QVariant
qvariantInitFromI32(qint32 i32);
QVariant
qvariantInitFromQColor(const QColor& color);
QVariant
qvariantInitFromQDate(const QDate& date);
QVariant
qvariantInitFromQDateTime(const QDateTime& dateTime);
QVariant
qvariantInitFromQPoint(const QPoint& point);
QVariant
qvariantInitFromQPointF(const QPointF& pointf);
QVariant
qvariantInitFromQRect(const QRect& rect);
QVariant
qvariantInitFromQRectF(const QRectF& rectf);
QVariant
qvariantInitFromQSize(const QSize& size);
QVariant
qvariantInitFromQSizeF(const QSizeF& sizef);
QVariant
qvariantInitFromQTime(const QTime& time);
QVariant
qvariantInitFromQUrl(const QUrl& url);
QVariant
qvariantInitFromQString(const QString& string);
QVariant
qvariantInitFromU8(quint8 u8);
QVariant
qvariantInitFromU16(quint16 u16);
QVariant
qvariantInitFromU32(quint32 u32);
types::QVariantType
qvariantType(const QVariant& variant);
bool
qvariantToBool(const QVariant& variant);
float
qvariantToF32(const QVariant& variant);
double
qvariantToF64(const QVariant& variant);
qint8
qvariantToI8(const QVariant& variant);
qint16
qvariantToI16(const QVariant& variant);
qint32
qvariantToI32(const QVariant& variant);
QColor
qvariantToQColor(const QVariant& variant);
QDate
qvariantToQDate(const QVariant& variant);
QDateTime
qvariantToQDateTime(const QVariant& variant);
QPoint
qvariantToQPoint(const QVariant& variant);
QPointF
qvariantToQPointF(const QVariant& variant);
QRect
qvariantToQRect(const QVariant& variant);
QRectF
qvariantToQRectF(const QVariant& variant);
QSize
qvariantToQSize(const QVariant& variant);
QSizeF
qvariantToQSizeF(const QVariant& variant);
QTime
qvariantToQTime(const QVariant& variant);
QUrl
qvariantToQUrl(const QVariant& variant);
QString
qvariantToQString(const QVariant& variant);
quint8
qvariantToU8(const QVariant& variant);
quint16
qvariantToU16(const QVariant& variant);
quint32
qvariantToU32(const QVariant& variant);

}
}
