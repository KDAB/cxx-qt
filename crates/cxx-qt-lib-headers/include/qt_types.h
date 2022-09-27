// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once
#include <memory>
#include <type_traits>

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

// Define which Qt types are relocatable
template<>
struct rust::IsRelocatable<QColor> : std::true_type
{
};
static_assert(QTypeInfo<QColor>::isRelocatable);

template<>
struct rust::IsRelocatable<QDateTime> : std::true_type
{
};
static_assert(QTypeInfo<QDateTime>::isRelocatable);

template<>
struct rust::IsRelocatable<QString> : std::true_type
{
};
static_assert(QTypeInfo<QString>::isRelocatable);

template<>
struct rust::IsRelocatable<QUrl> : std::true_type
{
};
static_assert(QTypeInfo<QUrl>::isRelocatable);

template<>
struct rust::IsRelocatable<QVariant> : std::true_type
{
};
static_assert(QTypeInfo<QVariant>::isRelocatable);

// Ensure that trivially copy assignable and constructible is correct
// If this is false then we need to manually implement Clone rather than derive

// QColor still had copy constructors in Qt 5 but they could have been trivial
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
static_assert(std::is_trivially_copy_assignable<QColor>::value);
static_assert(std::is_trivially_copy_constructible<QColor>::value);
#endif
static_assert(!std::is_trivially_copy_assignable<QDateTime>::value);
static_assert(!std::is_trivially_copy_constructible<QDateTime>::value);
static_assert(!std::is_trivially_copy_assignable<QString>::value);
static_assert(!std::is_trivially_copy_constructible<QString>::value);
static_assert(!std::is_trivially_copy_assignable<QUrl>::value);
static_assert(!std::is_trivially_copy_constructible<QUrl>::value);
static_assert(!std::is_trivially_copy_assignable<QVariant>::value);
static_assert(!std::is_trivially_copy_constructible<QVariant>::value);

// Ensure that trivially destructible is correct
// If this is false then we need to manually implement Drop rather than derive
static_assert(std::is_trivially_destructible<QColor>::value);
static_assert(!std::is_trivially_destructible<QDateTime>::value);
static_assert(!std::is_trivially_destructible<QString>::value);
static_assert(!std::is_trivially_destructible<QUrl>::value);
static_assert(!std::is_trivially_destructible<QVariant>::value);

// Ensure that types have the alignment and size we are expecting

// QColor has an enum with six values and a union with the largest being five
// ushorts. This results in (5 * std::uint16) + std::uint32_t = 14, then due to
// compiler padding this results in a sizeof 16 or two pointers.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qcolor.h?h=v5.15.6-lts-lgpl#n262
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qcolor.h?h=v6.2.4#n237
static_assert(alignof(QColor) <= alignof(std::size_t[2]),
              "unexpectedly large QColor alignment");
static_assert(sizeof(QColor) == sizeof(std::size_t[2]),
              "unexpected QColor size");

// QDateTime has a single member, which is a union, with the largest member
// being a pointer
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v5.15.6-lts-lgpl#n426
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v5.15.6-lts-lgpl#n270
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v6.2.4#n394
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v6.2.4#n255
static_assert(alignof(QDateTime) <= alignof(std::size_t),
              "unexpectedly large QDateTime alignment");
static_assert(sizeof(QDateTime) == sizeof(std::size_t),
              "unexpected QDateTime size");

// The layout has changed between Qt 5 and Qt 6
//
// Qt5 QString has one pointer as a member
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/text/qstring.h?h=v5.15.6-lts-lgpl#n979
//
// Qt6 QString has one member, which contains two pointers and a size_t
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/text/qstring.h?h=v6.2.4#n1094
// DataPointer is then a QStringPrivate, which is a QArrayDataPointer<char16_t>
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qarraydatapointer.h?h=v6.2.4#n390
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
static_assert(alignof(QString) <= alignof(std::size_t[3]),
              "unexpectedly large QString alignment");
static_assert(sizeof(QString) == sizeof(std::size_t[3]),
              "unexpected QString size");
#else
static_assert(alignof(QString) <= alignof(std::size_t),
              "unexpectedly large QString alignment");
static_assert(sizeof(QString) == sizeof(std::size_t),
              "unexpected QString size");
#endif

// QUrl has a single pointer as it's member
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/io/qurl.h?h=v5.15.6-lts-lgpl#n367
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/io/qurl.h?h=v6.2.4#n294
static_assert(alignof(QUrl) <= alignof(std::size_t),
              "unexpectedly large QUrl alignment");
static_assert(sizeof(QUrl) == sizeof(std::size_t), "unexpected QUrl size");

// The layout has changed between Qt 5 and Qt 6
//
// Qt5 QVariant has one member, which contains three uints and a union.
// The three uints are optimised to a reduced size, resulting in a combined size
// of two pointers.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qvariant.h?h=v5.15.6-lts-lgpl#n491
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qvariant.h?h=v5.15.6-lts-lgpl#n411
//
// Qt6 QVariant has one member, which contains three pointers and a union
// (with a pointer as the largest member)
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qvariant.h?h=v6.2.4#n540
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qvariant.h?h=v6.2.4#n474
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
static_assert(alignof(QVariant) <= alignof(std::size_t[4]),
              "unexpectedly large QVariant alignment");
static_assert(sizeof(QVariant) == sizeof(std::size_t[4]),
              "unexpected QVariant size");
#else
static_assert(alignof(QVariant) <= alignof(std::size_t[2]),
              "unexpectedly large QVariant alignment");
static_assert(sizeof(QVariant) == sizeof(std::size_t[2]),
              "unexpected QVariant size");
#endif

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

QColor
qcolorInitDefault();
QColor
qcolorInitFromRgba(std::int32_t r,
                   std::int32_t g,
                   std::int32_t b,
                   std::int32_t a);

QDate
qdateInitDefault();
QDate
qdateInit(int y, int m, int d);

void
qdatetimeDrop(QDateTime& datetime);
QDateTime
qdatetimeInitDefault();
QDateTime
qdatetimeInitFromDateAndTime(const QDate& date, const QTime& time);
QDateTime
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

void
qstringDrop(QString& string);
QString
qstringInitDefault();
QString
qstringInitFromRustString(rust::Str string);
QString
qstringInitFromQString(const QString& string);
rust::String
qstringToRustString(const QString& string);

QTime
qtimeInitDefault();
QTime
qtimeInit(int h, int m, int s, int ms);

void
qurlDrop(QUrl& url);
QUrl
qurlInitDefault();
QUrl
qurlInitFromQString(const QString& string);
QUrl
qurlInitFromString(rust::Str string);
QUrl
qurlInitFromQUrl(const QUrl& url);
QString
qurlToQString(const QUrl& url);
rust::String
qurlToRustString(const QUrl& url);

void
qvariantDrop(QVariant& variant);
QVariant
qvariantInitDefault();
QVariant
qvariantInitFromQVariant(const QVariant& variant);
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

} // namespace cxxqtlib1
} // namespace rust
