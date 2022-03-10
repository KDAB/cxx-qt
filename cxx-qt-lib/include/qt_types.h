// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group
// company <info@kdab.com> SPDX-FileContributor: Andrew Hayzen
// <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once
#include <memory>

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
#include <QVariant>

#include "rust/cxx.h"

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
  String = 16,
  QTime = 17,
  QUrl = 18,
  U8 = 19,
  U16 = 20,
  U32 = 21,
};

} // namespace types

std::unique_ptr<QColor>
qcolorInitFromRgba(std::int32_t r,
                   std::int32_t g,
                   std::int32_t b,
                   std::int32_t a);
std::unique_ptr<QColor>
qcolorInitFromQColor(const QColor& color);

QDate
qdateInitDefault();
QDate
qdateInit(int y, int m, int d);

std::unique_ptr<QDateTime>
qdatetimeInitFromDateAndTime(const QDate& date, const QTime& time);
std::unique_ptr<QDateTime>
qdatetimeInitFromQDateTime(const QDateTime& datetime);
void
qdatetimeSetDate(QDateTime& datetime, QDate date);
void
qdatetimeSetTime(QDateTime& datetime, QTime time);

QPoint
qpointInitDefault();
QPoint
qpointInit(int x, int y);

QPointF
qpointfInitDefault();
QPointF
qpointfInit(qreal x, qreal y);

QRect
qrectInitDefault();
QRect
qrectInit(int x, int y, int w, int h);

QRectF
qrectfInitDefault();
QRectF
qrectfInit(qreal x, qreal y, qreal w, qreal h);

QSize
qsizeInitDefault();
QSize
qsizeInit(int width, int height);

QSizeF
qsizefInitDefault();
QSizeF
qsizefInit(qreal width, qreal height);

std::unique_ptr<QString>
qstringInitFromRustString(rust::Str string);
rust::String
qstringToRustString(const QString& string);

QTime
qtimeInitDefault();
QTime
qtimeInit(int h, int m, int s, int ms);

std::unique_ptr<QUrl>
qurlInitFromString(rust::Str string);
std::unique_ptr<QUrl>
qurlInitFromQUrl(const QUrl& url);
rust::String
qurlToRustString(const QUrl& url);

std::unique_ptr<QVariant>
qvariantInitFromQVariant(const QVariant& variant);
std::unique_ptr<QVariant>
qvariantInitFromBool(bool b);
std::unique_ptr<QVariant>
qvariantInitFromF32(float f32);
std::unique_ptr<QVariant>
qvariantInitFromF64(double f64);
std::unique_ptr<QVariant>
qvariantInitFromI8(qint8 i8);
std::unique_ptr<QVariant>
qvariantInitFromI16(qint16 i16);
std::unique_ptr<QVariant>
qvariantInitFromI32(qint32 i32);
std::unique_ptr<QVariant>
qvariantInitFromQColor(const QColor& color);
std::unique_ptr<QVariant>
qvariantInitFromQDate(const QDate& date);
std::unique_ptr<QVariant>
qvariantInitFromQDateTime(const QDateTime& dateTime);
std::unique_ptr<QVariant>
qvariantInitFromQPoint(const QPoint& point);
std::unique_ptr<QVariant>
qvariantInitFromQPointF(const QPointF& pointf);
std::unique_ptr<QVariant>
qvariantInitFromQRect(const QRect& rect);
std::unique_ptr<QVariant>
qvariantInitFromQRectF(const QRectF& rectf);
std::unique_ptr<QVariant>
qvariantInitFromQSize(const QSize& size);
std::unique_ptr<QVariant>
qvariantInitFromQSizeF(const QSizeF& sizef);
std::unique_ptr<QVariant>
qvariantInitFromQTime(const QTime& time);
std::unique_ptr<QVariant>
qvariantInitFromQUrl(const QUrl& url);
std::unique_ptr<QVariant>
qvariantInitFromRustString(rust::Str string);
std::unique_ptr<QVariant>
qvariantInitFromU8(quint8 u8);
std::unique_ptr<QVariant>
qvariantInitFromU16(quint16 u16);
std::unique_ptr<QVariant>
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
std::unique_ptr<QColor>
qvariantToQColor(const QVariant& variant);
QDate
qvariantToQDate(const QVariant& variant);
std::unique_ptr<QDateTime>
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
std::unique_ptr<QUrl>
qvariantToQUrl(const QVariant& variant);
rust::String
qvariantToRustString(const QVariant& variant);
quint8
qvariantToU8(const QVariant& variant);
quint16
qvariantToU16(const QVariant& variant);
quint32
qvariantToU32(const QVariant& variant);

} // namespace cxxqtlib1
} // namespace rust
