// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qvector.h"

#include "../../assertion_utils.h"

#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
#define CXX_QT_QVECTOR_ALIGN_AND_SIZE(typeName, name)                          \
  assert_alignment_and_size(                                                   \
    QVector_##name, alignof(::std::size_t), sizeof(::std::size_t[3]));
#else
#define CXX_QT_QVECTOR_ALIGN_AND_SIZE(typeName, name)                          \
  assert_alignment_and_size(                                                   \
    QVector_##name, alignof(::std::size_t), sizeof(::std::size_t));
#endif

#define CXX_QT_QVECTOR_ASSERTS(typeName, name)                                 \
  CXX_QT_QVECTOR_ALIGN_AND_SIZE(typeName, name);                               \
                                                                               \
  static_assert(!::std::is_trivially_copy_assignable<QVector_##name>::value);  \
  static_assert(                                                               \
    !::std::is_trivially_copy_constructible<QVector_##name>::value);           \
  static_assert(!::std::is_trivially_destructible<QVector_##name>::value);     \
                                                                               \
  static_assert(QTypeInfo<QVector_##name>::isRelocatable);                     \
                                                                               \
  static_assert(::std::is_copy_assignable<typeName>::value);                   \
  static_assert(::std::is_copy_constructible<typeName>::value);

CXX_QT_QVECTOR_ASSERTS(bool, bool);
CXX_QT_QVECTOR_ASSERTS(float, f32);
CXX_QT_QVECTOR_ASSERTS(double, f64);
CXX_QT_QVECTOR_ASSERTS(::std::int8_t, i8);
CXX_QT_QVECTOR_ASSERTS(::std::int16_t, i16);
CXX_QT_QVECTOR_ASSERTS(::std::int32_t, i32);
CXX_QT_QVECTOR_ASSERTS(::std::int64_t, i64);
CXX_QT_QVECTOR_ASSERTS(::QByteArray, QByteArray);
#ifdef CXX_QT_GUI_FEATURE
CXX_QT_QVECTOR_ASSERTS(::QColor, QColor);
#endif
CXX_QT_QVECTOR_ASSERTS(::QDate, QDate);
CXX_QT_QVECTOR_ASSERTS(::QDateTime, QDateTime);
CXX_QT_QVECTOR_ASSERTS(::QMargins, QMargins);
CXX_QT_QVECTOR_ASSERTS(::QMarginsF, QMarginsF);
CXX_QT_QVECTOR_ASSERTS(::QPersistentModelIndex, QPersistentModelIndex);
CXX_QT_QVECTOR_ASSERTS(::QPoint, QPoint);
CXX_QT_QVECTOR_ASSERTS(::QPointF, QPointF);
CXX_QT_QVECTOR_ASSERTS(::QRect, QRect);
CXX_QT_QVECTOR_ASSERTS(::QRectF, QRectF);
CXX_QT_QVECTOR_ASSERTS(::QSize, QSize);
CXX_QT_QVECTOR_ASSERTS(::QSizeF, QSizeF);
CXX_QT_QVECTOR_ASSERTS(::QString, QString);
CXX_QT_QVECTOR_ASSERTS(::QTime, QTime);
CXX_QT_QVECTOR_ASSERTS(::QUrl, QUrl);
CXX_QT_QVECTOR_ASSERTS(::QVariant, QVariant);
CXX_QT_QVECTOR_ASSERTS(::std::uint8_t, u8);
CXX_QT_QVECTOR_ASSERTS(::std::uint16_t, u16);
CXX_QT_QVECTOR_ASSERTS(::std::uint32_t, u32);
CXX_QT_QVECTOR_ASSERTS(::std::uint64_t, u64);

static const int register_QVector_bool =
  qRegisterMetaType<::QVector_bool>("QVector_bool");
static const int register_QVector_f32 =
  qRegisterMetaType<::QVector_f32>("QVector_f32");
static const int register_QVector_f64 =
  qRegisterMetaType<::QVector_f64>("QVector_f64");
static const int register_QVector_i8 =
  qRegisterMetaType<::QVector_i8>("QVector_i8");
static const int register_QVector_i16 =
  qRegisterMetaType<::QVector_i16>("QVector_i16");
static const int register_QVector_i32 =
  qRegisterMetaType<::QVector_i32>("QVector_i32");
static const int register_QVector_i64 =
  qRegisterMetaType<::QVector_i64>("QVector_i64");
static const int register_QVector_QByteArray =
  qRegisterMetaType<::QVector_QByteArray>("QVector_QByteArray");
#ifdef CXX_QT_GUI_FEATURE
static const int register_QVector_QColor =
  qRegisterMetaType<::QVector_QColor>("QVector_QColor");
#endif
static const int register_QVector_QDate =
  qRegisterMetaType<::QVector_QDate>("QVector_QDate");
static const int register_QVector_QDateTime =
  qRegisterMetaType<::QVector_QDateTime>("QVector_QDateTime");
static const int register_QVector_QMargins =
  qRegisterMetaType<::QVector_QMargins>("QVector_QMargins");
static const int register_QVector_QMarginsF =
  qRegisterMetaType<::QVector_QMarginsF>("QVector_QMarginsF");
static const int register_QVector_QPersistentModelIndex =
  qRegisterMetaType<::QVector_QPersistentModelIndex>(
    "QVector_QPersistentModelIndex");
static const int register_QVector_QPoint =
  qRegisterMetaType<::QVector_QPoint>("QVector_QPoint");
static const int register_QVector_QPointF =
  qRegisterMetaType<::QVector_QPointF>("QVector_QPointF");
static const int register_QVector_QRect =
  qRegisterMetaType<::QVector_QRect>("QVector_QRect");
static const int register_QVector_QRectF =
  qRegisterMetaType<::QVector_QRectF>("QVector_QRectF");
static const int register_QVector_QSize =
  qRegisterMetaType<::QVector_QSize>("QVector_QSize");
static const int register_QVector_QSizeF =
  qRegisterMetaType<::QVector_QSizeF>("QVector_QSizeF");
static const int register_QVector_QString =
  qRegisterMetaType<::QVector_QString>("QVector_QString");
static const int register_QVector_QTime =
  qRegisterMetaType<::QVector_QTime>("QVector_QTime");
static const int register_QVector_QUrl =
  qRegisterMetaType<::QVector_QUrl>("QVector_QUrl");
// Ensure that QVector<QVariant> (aka QVariantList) is registered
// otherwise it cannot be used in QML
static const int register_QVector_QVariant =
  qRegisterMetaType<::QVector_QVariant>("QVector_QVariant");
static const int register_QVector_u8 =
  qRegisterMetaType<::QVector_u8>("QVector_u8");
static const int register_QVector_u16 =
  qRegisterMetaType<::QVector_u16>("QVector_u16");
static const int register_QVector_u32 =
  qRegisterMetaType<::QVector_u32>("QVector_u32");
static const int register_QVector_u64 =
  qRegisterMetaType<::QVector_u64>("QVector_u64");
