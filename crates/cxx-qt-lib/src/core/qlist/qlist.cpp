// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qlist.h"

#include "../../assertion_utils.h"

#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
#define CXX_QT_QLIST_ALIGN_AND_SIZE(typeName, name)                            \
  assert_alignment_and_size(                                                   \
    QList_##name, alignof(::std::size_t), sizeof(::std::size_t[3]));
#else
#define CXX_QT_QLIST_ALIGN_AND_SIZE(typeName, name)                            \
  assert_alignment_and_size(                                                   \
    QList_##name, alignof(::std::size_t), sizeof(::std::size_t));
#endif

#define CXX_QT_QLIST_ASSERTS(typeName, name)                                   \
  CXX_QT_QLIST_ALIGN_AND_SIZE(typeName, name);                                 \
                                                                               \
  static_assert(!::std::is_trivially_copy_assignable<QList_##name>::value);    \
  static_assert(!::std::is_trivially_copy_constructible<QList_##name>::value); \
  static_assert(!::std::is_trivially_destructible<QList_##name>::value);       \
                                                                               \
  static_assert(QTypeInfo<QList_##name>::isRelocatable);                       \
                                                                               \
  static_assert(::std::is_copy_assignable<typeName>::value);                   \
  static_assert(::std::is_copy_constructible<typeName>::value);

CXX_QT_QLIST_ASSERTS(bool, bool);
CXX_QT_QLIST_ASSERTS(float, f32);
CXX_QT_QLIST_ASSERTS(double, f64);
CXX_QT_QLIST_ASSERTS(::std::int8_t, i8);
CXX_QT_QLIST_ASSERTS(::std::int16_t, i16);
CXX_QT_QLIST_ASSERTS(::std::int32_t, i32);
CXX_QT_QLIST_ASSERTS(::std::int64_t, i64);
CXX_QT_QLIST_ASSERTS(::QByteArray, QByteArray);
#ifdef CXX_QT_GUI_FEATURE
CXX_QT_QLIST_ASSERTS(::QColor, QColor);
#endif
CXX_QT_QLIST_ASSERTS(::QDate, QDate);
CXX_QT_QLIST_ASSERTS(::QDateTime, QDateTime);
CXX_QT_QLIST_ASSERTS(::QMargins, QMargins);
CXX_QT_QLIST_ASSERTS(::QMarginsF, QMarginsF);
CXX_QT_QLIST_ASSERTS(::QPersistentModelIndex, QPersistentModelIndex);
CXX_QT_QLIST_ASSERTS(::QPoint, QPoint);
CXX_QT_QLIST_ASSERTS(::QPointF, QPointF);
CXX_QT_QLIST_ASSERTS(::QRect, QRect);
CXX_QT_QLIST_ASSERTS(::QRectF, QRectF);
CXX_QT_QLIST_ASSERTS(::QSize, QSize);
CXX_QT_QLIST_ASSERTS(::QSizeF, QSizeF);
CXX_QT_QLIST_ASSERTS(::QString, QString);
CXX_QT_QLIST_ASSERTS(::QTime, QTime);
CXX_QT_QLIST_ASSERTS(::QUrl, QUrl);
CXX_QT_QLIST_ASSERTS(::QVariant, QVariant);
CXX_QT_QLIST_ASSERTS(::std::uint8_t, u8);
CXX_QT_QLIST_ASSERTS(::std::uint16_t, u16);
CXX_QT_QLIST_ASSERTS(::std::uint32_t, u32);
CXX_QT_QLIST_ASSERTS(::std::uint64_t, u64);

static const int register_QList_bool =
  qRegisterMetaType<::QList_bool>("QList_bool");
static const int register_QList_f32 =
  qRegisterMetaType<::QList_f32>("QList_f32");
static const int register_QList_f64 =
  qRegisterMetaType<::QList_f64>("QList_f64");
static const int register_QList_i8 = qRegisterMetaType<::QList_i8>("QList_i8");
static const int register_QList_i16 =
  qRegisterMetaType<::QList_i16>("QList_i16");
static const int register_QList_i32 =
  qRegisterMetaType<::QList_i32>("QList_i32");
static const int register_QList_i64 =
  qRegisterMetaType<::QList_i64>("QList_i64");
static const int register_QList_QByteArray =
  qRegisterMetaType<::QList_QByteArray>("QList_QByteArray");
#ifdef CXX_QT_GUI_FEATURE
static const int register_QList_QColor =
  qRegisterMetaType<::QList_QColor>("QList_QColor");
#endif
static const int register_QList_QDate =
  qRegisterMetaType<::QList_QDate>("QList_QDate");
static const int register_QList_QDateTime =
  qRegisterMetaType<::QList_QDateTime>("QList_QDateTime");
static const int register_QList_QMargins =
  qRegisterMetaType<::QList_QMargins>("QList_QMargins");
static const int register_QList_QMarginsF =
  qRegisterMetaType<::QList_QMarginsF>("QList_QMarginsF");
static const int register_QList_QPersistentModelIndex =
  qRegisterMetaType<::QList_QPersistentModelIndex>(
    "QList_QPersistentModelIndex");
static const int register_QList_QPoint =
  qRegisterMetaType<::QList_QPoint>("QList_QPoint");
static const int register_QList_QPointF =
  qRegisterMetaType<::QList_QPointF>("QList_QPointF");
static const int register_QList_QRect =
  qRegisterMetaType<::QList_QRect>("QList_QRect");
static const int register_QList_QRectF =
  qRegisterMetaType<::QList_QRectF>("QList_QRectF");
static const int register_QList_QSize =
  qRegisterMetaType<::QList_QSize>("QList_QSize");
static const int register_QList_QSizeF =
  qRegisterMetaType<::QList_QSizeF>("QList_QSizeF");
static const int register_QList_QString =
  qRegisterMetaType<::QList_QString>("QList_QString");
static const int register_QList_QTime =
  qRegisterMetaType<::QList_QTime>("QList_QTime");
static const int register_QList_QUrl =
  qRegisterMetaType<::QList_QUrl>("QList_QUrl");
// Ensure that QList<QVariant> (aka QVariantList) is registered
// otherwise it cannot be used in QML
static const int register_QList_QVariant =
  qRegisterMetaType<::QList_QVariant>("QList_QVariant");
static const int register_QList_u8 = qRegisterMetaType<::QList_u8>("QList_u8");
static const int register_QList_u16 =
  qRegisterMetaType<::QList_u16>("QList_u16");
static const int register_QList_u32 =
  qRegisterMetaType<::QList_u32>("QList_u32");
static const int register_QList_u64 =
  qRegisterMetaType<::QList_u64>("QList_u64");
