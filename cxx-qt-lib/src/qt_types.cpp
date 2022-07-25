// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group
// company <info@kdab.com> SPDX-FileContributor: Andrew Hayzen
// <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/include/qt_types.h"

#include <QMetaObject>

namespace rust {
namespace cxxqtlib1 {

UpdateRequester::UpdateRequester(QPointer<QObject> obj, const char* method)
  : m_method(method)
  , m_obj(obj)
{
}

bool
UpdateRequester::requestUpdate() const
{
  if (m_obj == nullptr) {
    return false;
  }

  return QMetaObject::invokeMethod(m_obj, m_method, Qt::QueuedConnection);
}

std::unique_ptr<QColor>
qcolorInit()
{
  return std::make_unique<QColor>();
}

std::unique_ptr<QColor>
qcolorInitFromRgba(std::int32_t r,
                   std::int32_t g,
                   std::int32_t b,
                   std::int32_t a)
{
  return std::make_unique<QColor>(r, g, b, a);
}

std::unique_ptr<QColor>
qcolorInitFromQColor(const QColor& color)
{
  return std::make_unique<QColor>(color);
}

QDate
qdateInitDefault()
{
  return QDate();
}

QDate
qdateInit(int y, int m, int d)
{
  return QDate(y, m, d);
}

std::unique_ptr<QDateTime>
qdatetimeInit()
{
  return std::make_unique<QDateTime>();
}

std::unique_ptr<QDateTime>
qdatetimeInitFromDateAndTime(const QDate& date, const QTime& time)
{
  return std::make_unique<QDateTime>(date, time);
}

std::unique_ptr<QDateTime>
qdatetimeInitFromQDateTime(const QDateTime& datetime)
{
  return std::make_unique<QDateTime>(datetime);
}

void
qdatetimeSetDate(QDateTime& datetime, QDate date)
{
  datetime.setDate(date);
}

void
qdatetimeSetTime(QDateTime& datetime, QTime time)
{
  datetime.setTime(time);
}

QPoint
qpointInitDefault()
{
  return QPoint();
}

QPoint
qpointInit(int x, int y)
{
  return QPoint(x, y);
}

QPointF
qpointfInitDefault()
{
  return QPointF();
}

QPointF
qpointfInit(qreal x, qreal y)
{
  return QPointF(x, y);
}

QRect
qrectInitDefault()
{
  return QRect();
}

QRect
qrectInit(int x, int y, int w, int h)
{
  return QRect(x, y, w, h);
}

QRectF
qrectfInitDefault()
{
  return QRectF();
}

QRectF
qrectfInit(qreal x, qreal y, qreal w, qreal h)
{
  return QRectF(x, y, w, h);
}

QSize
qsizeInitDefault()
{
  return QSize();
}

QSize
qsizeInit(int width, int height)
{
  return QSize(width, height);
}

QSizeF
qsizefInitDefault()
{
  return QSizeF();
}

QSizeF
qsizefInit(qreal width, qreal height)
{
  return QSizeF(width, height);
}

std::unique_ptr<QString>
qstringInitFromRustString(rust::Str string)
{
  // Note that rust::Str here is borrowed
  // and we convert back from UTF-8 to UTF-16
  return std::make_unique<QString>(qstringFromRustString(string));
}

QString
qstringFromRustString(rust::Str string)
{
  // Note that rust::Str here is borrowed
  // and we convert back from UTF-8 to UTF-16
  return QString::fromUtf8(string.data(), string.size());
}

rust::String
qstringToRustString(const QString& string)
{
  // Note that this changes UTF-16 to UTF-8
  const auto byteArray = string.toUtf8();
  return rust::String(byteArray.constData(), byteArray.size());
}

QTime
qtimeInitDefault()
{
  return QTime();
}

QTime
qtimeInit(int h, int m, int s, int ms)
{
  return QTime(h, m, s, ms);
}

std::unique_ptr<QUrl>
qurlInit()
{
  return std::make_unique<QUrl>();
}

std::unique_ptr<QUrl>
qurlInitFromString(rust::Str string)
{
  // Note that rust::Str here is borrowed
  // and we convert back from UTF-8 to UTF-16
  return std::make_unique<QUrl>(qstringFromRustString(string));
}

std::unique_ptr<QUrl>
qurlInitFromQUrl(const QUrl& url)
{
  return std::make_unique<QUrl>(url);
}

rust::String
qurlToRustString(const QUrl& url)
{
  // Note that this changes UTF-16 to UTF-8
  return qstringToRustString(url.toString());
}

std::unique_ptr<QVariant>
qvariantInit()
{
  return std::make_unique<QVariant>();
}

#define CXX_QT_VARIANT_INIT(typeName, name)                                    \
  std::unique_ptr<QVariant> qvariantInitFrom##name(typeName value)             \
  {                                                                            \
    return std::make_unique<QVariant>(value);                                  \
  }

#define CXX_QT_VARIANT_INIT_REF(typeName, name)                                \
  std::unique_ptr<QVariant> qvariantInitFrom##name(const typeName& value)      \
  {                                                                            \
    return std::make_unique<QVariant>(value);                                  \
  }

CXX_QT_VARIANT_INIT_REF(QVariant, QVariant)
CXX_QT_VARIANT_INIT(bool, Bool)
CXX_QT_VARIANT_INIT(float, F32)
CXX_QT_VARIANT_INIT(double, F64)
CXX_QT_VARIANT_INIT(qint8, I8)
CXX_QT_VARIANT_INIT(qint16, I16)
CXX_QT_VARIANT_INIT(qint32, I32)
CXX_QT_VARIANT_INIT_REF(QColor, QColor)
CXX_QT_VARIANT_INIT_REF(QDate, QDate)
CXX_QT_VARIANT_INIT_REF(QDateTime, QDateTime)
CXX_QT_VARIANT_INIT_REF(QPoint, QPoint)
CXX_QT_VARIANT_INIT_REF(QPointF, QPointF)
CXX_QT_VARIANT_INIT_REF(QRect, QRect)
CXX_QT_VARIANT_INIT_REF(QRectF, QRectF)
CXX_QT_VARIANT_INIT_REF(QSize, QSize)
CXX_QT_VARIANT_INIT_REF(QSizeF, QSizeF)
CXX_QT_VARIANT_INIT_REF(QTime, QTime)
CXX_QT_VARIANT_INIT_REF(QUrl, QUrl)

std::unique_ptr<QVariant>
qvariantInitFromRustString(rust::Str string)
{
  // Note that rust::Str here is borrowed
  // and we convert back from UTF-8 to UTF-16
  return std::make_unique<QVariant>(qstringFromRustString(string));
}

CXX_QT_VARIANT_INIT(quint8, U8)
CXX_QT_VARIANT_INIT(quint16, U16)
CXX_QT_VARIANT_INIT(quint32, U32)

types::QVariantType
qvariantType(const QVariant& variant)
{
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  switch (static_cast<QMetaType::Type>(variant.metaType().id())) {
#else
  // QVariant::Type is obsolete, ensure we use QMetaType::Type to avoid
  // warnings
  switch (static_cast<QMetaType::Type>(variant.type())) {
#endif
    case QMetaType::Bool:
      return types::QVariantType::Bool;
    case QMetaType::Float:
      return types::QVariantType::F32;
    case QMetaType::Double:
      return types::QVariantType::F64;
    case QMetaType::SChar:
      return types::QVariantType::I8;
    case QMetaType::Short:
      return types::QVariantType::I16;
    case QMetaType::Int:
      return types::QVariantType::I32;
    case QMetaType::QColor:
      return types::QVariantType::QColor;
    case QMetaType::QDate:
      return types::QVariantType::QDate;
    case QMetaType::QDateTime:
      return types::QVariantType::QDateTime;
    case QMetaType::QPoint:
      return types::QVariantType::QPoint;
    case QMetaType::QPointF:
      return types::QVariantType::QPointF;
    case QMetaType::QRect:
      return types::QVariantType::QRect;
    case QMetaType::QRectF:
      return types::QVariantType::QRectF;
    case QMetaType::QSize:
      return types::QVariantType::QSize;
    case QMetaType::QSizeF:
      return types::QVariantType::QSizeF;
    case QMetaType::QString:
      return types::QVariantType::String;
    case QMetaType::QTime:
      return types::QVariantType::QTime;
    case QMetaType::QUrl:
      return types::QVariantType::QUrl;
    case QMetaType::UChar:
      return types::QVariantType::U8;
    case QMetaType::UShort:
      return types::QVariantType::U16;
    case QMetaType::UInt:
      return types::QVariantType::U32;

    default:
      return types::QVariantType::Unsupported;
  }
}

#define CXX_QT_VARIANT_OPAQUE_VALUE(typeName, name)                            \
  std::unique_ptr<typeName> qvariantTo##name(const QVariant& variant)          \
  {                                                                            \
    Q_ASSERT(variant.canConvert<typeName>());                                  \
    return std::make_unique<typeName>(variant.value<typeName>());              \
  }

#define CXX_QT_VARIANT_TRIVIAL_VALUE(typeName, name)                           \
  typeName qvariantTo##name(const QVariant& variant)                           \
  {                                                                            \
    Q_ASSERT(variant.canConvert<typeName>());                                  \
    return variant.value<typeName>();                                          \
  }

CXX_QT_VARIANT_TRIVIAL_VALUE(bool, Bool)
CXX_QT_VARIANT_TRIVIAL_VALUE(float, F32)
CXX_QT_VARIANT_TRIVIAL_VALUE(double, F64)
CXX_QT_VARIANT_TRIVIAL_VALUE(qint8, I8)
CXX_QT_VARIANT_TRIVIAL_VALUE(qint16, I16)
CXX_QT_VARIANT_TRIVIAL_VALUE(qint32, I32)
CXX_QT_VARIANT_OPAQUE_VALUE(QColor, QColor)
CXX_QT_VARIANT_TRIVIAL_VALUE(QDate, QDate)
CXX_QT_VARIANT_OPAQUE_VALUE(QDateTime, QDateTime)
CXX_QT_VARIANT_TRIVIAL_VALUE(QPoint, QPoint)
CXX_QT_VARIANT_TRIVIAL_VALUE(QPointF, QPointF)
CXX_QT_VARIANT_TRIVIAL_VALUE(QRect, QRect)
CXX_QT_VARIANT_TRIVIAL_VALUE(QRectF, QRectF)
CXX_QT_VARIANT_TRIVIAL_VALUE(QSize, QSize)
CXX_QT_VARIANT_TRIVIAL_VALUE(QSizeF, QSizeF)
CXX_QT_VARIANT_TRIVIAL_VALUE(QTime, QTime)
CXX_QT_VARIANT_OPAQUE_VALUE(QUrl, QUrl)

rust::String
qvariantToRustString(const QVariant& variant)
{
  return qstringToRustString(variant.toString());
}

CXX_QT_VARIANT_TRIVIAL_VALUE(quint8, U8)
CXX_QT_VARIANT_TRIVIAL_VALUE(quint16, U16)
CXX_QT_VARIANT_TRIVIAL_VALUE(quint32, U32)

}
}
