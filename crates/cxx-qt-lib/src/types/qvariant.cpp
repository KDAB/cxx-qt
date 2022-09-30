// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qvariant.h"

#include <QMetaObject>

#include "assertion_utils.h"

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
assert_alignment_and_size(QVariant,
                          alignof(std::size_t),
                          sizeof(std::size_t[4]));
#else
assert_alignment_and_size(QVariant,
                          alignof(std::size_t),
                          sizeof(std::size_t[2]));
#endif

static_assert(!std::is_trivially_copy_assignable<QVariant>::value);
static_assert(!std::is_trivially_copy_constructible<QVariant>::value);

// Ensure that trivially destructible is correct
// If this is false then we need to manually implement Drop rather than derive
static_assert(!std::is_trivially_destructible<QVariant>::value);

static_assert(QTypeInfo<QVariant>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

void
qvariantDrop(QVariant& variant)
{
  variant.~QVariant();
}

QVariant
qvariantInitDefault()
{
  return QVariant();
}

#define CXX_QT_VARIANT_INIT(typeName, name)                                    \
  QVariant qvariantInitFrom##name(typeName value)                              \
  {                                                                            \
    return QVariant(value);                                                    \
  }

#define CXX_QT_VARIANT_INIT_REF(typeName, name)                                \
  QVariant qvariantInitFrom##name(const typeName& value)                       \
  {                                                                            \
    return QVariant(value);                                                    \
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
CXX_QT_VARIANT_INIT_REF(QString, QString)
CXX_QT_VARIANT_INIT_REF(QTime, QTime)
CXX_QT_VARIANT_INIT_REF(QUrl, QUrl)
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
      return types::QVariantType::QString;
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
CXX_QT_VARIANT_TRIVIAL_VALUE(QColor, QColor)
CXX_QT_VARIANT_TRIVIAL_VALUE(QDate, QDate)
CXX_QT_VARIANT_TRIVIAL_VALUE(QDateTime, QDateTime)
CXX_QT_VARIANT_TRIVIAL_VALUE(QPoint, QPoint)
CXX_QT_VARIANT_TRIVIAL_VALUE(QPointF, QPointF)
CXX_QT_VARIANT_TRIVIAL_VALUE(QRect, QRect)
CXX_QT_VARIANT_TRIVIAL_VALUE(QRectF, QRectF)
CXX_QT_VARIANT_TRIVIAL_VALUE(QSize, QSize)
CXX_QT_VARIANT_TRIVIAL_VALUE(QSizeF, QSizeF)
CXX_QT_VARIANT_TRIVIAL_VALUE(QString, QString)
CXX_QT_VARIANT_TRIVIAL_VALUE(QTime, QTime)
CXX_QT_VARIANT_TRIVIAL_VALUE(QUrl, QUrl)
CXX_QT_VARIANT_TRIVIAL_VALUE(quint8, U8)
CXX_QT_VARIANT_TRIVIAL_VALUE(quint16, U16)
CXX_QT_VARIANT_TRIVIAL_VALUE(quint32, U32)

} // namespace cxxqtlib1
} // namespace rust
