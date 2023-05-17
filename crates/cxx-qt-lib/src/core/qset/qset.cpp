// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qset.h"

#include "../../assertion_utils.h"

#define CXX_QT_QSET_ASSERTS(typeName, name)                                    \
  assert_alignment_and_size(                                                   \
    QSet_##name, alignof(::std::size_t), sizeof(::std::size_t));               \
                                                                               \
  static_assert(!::std::is_trivially_copy_assignable<QSet_##name>::value);     \
  static_assert(!::std::is_trivially_copy_constructible<QSet_##name>::value);  \
  static_assert(!::std::is_trivially_destructible<QSet_##name>::value);        \
                                                                               \
  static_assert(QTypeInfo<QSet_##name>::isRelocatable);                        \
                                                                               \
  static_assert(::std::is_copy_assignable<typeName>::value);                   \
  static_assert(::std::is_copy_constructible<typeName>::value);

CXX_QT_QSET_ASSERTS(bool, bool);
CXX_QT_QSET_ASSERTS(float, f32);
CXX_QT_QSET_ASSERTS(double, f64);
CXX_QT_QSET_ASSERTS(::std::int8_t, i8);
CXX_QT_QSET_ASSERTS(::std::int16_t, i16);
CXX_QT_QSET_ASSERTS(::std::int32_t, i32);
CXX_QT_QSET_ASSERTS(::std::int64_t, i64);
CXX_QT_QSET_ASSERTS(::QByteArray, QByteArray);
CXX_QT_QSET_ASSERTS(::QDate, QDate);
CXX_QT_QSET_ASSERTS(::QDateTime, QDateTime);
CXX_QT_QSET_ASSERTS(::QPersistentModelIndex, QPersistentModelIndex);
CXX_QT_QSET_ASSERTS(::QString, QString);
CXX_QT_QSET_ASSERTS(::QTime, QTime);
CXX_QT_QSET_ASSERTS(::QUrl, QUrl);
CXX_QT_QSET_ASSERTS(::std::uint8_t, u8);
CXX_QT_QSET_ASSERTS(::std::uint16_t, u16);
CXX_QT_QSET_ASSERTS(::std::uint32_t, u32);
CXX_QT_QSET_ASSERTS(::std::uint64_t, u64);

static const int register_QSet_bool =
  qRegisterMetaType<::QSet_bool>("QSet_bool");
static const int register_QSet_f32 = qRegisterMetaType<::QSet_f32>("QSet_f32");
static const int register_QSet_f64 = qRegisterMetaType<::QSet_f64>("QSet_f64");
static const int register_QSet_i8 = qRegisterMetaType<::QSet_i8>("QSet_i8");
static const int register_QSet_i16 = qRegisterMetaType<::QSet_i16>("QSet_i16");
static const int register_QSet_i32 = qRegisterMetaType<::QSet_i32>("QSet_i32");
static const int register_QSet_i64 = qRegisterMetaType<::QSet_i64>("QSet_i64");
static const int register_QSet_QByteArray =
  qRegisterMetaType<::QSet_QByteArray>("QSet_QByteArray");
static const int register_QSet_QDate =
  qRegisterMetaType<::QSet_QDate>("QSet_QDate");
static const int register_QSet_QDateTime =
  qRegisterMetaType<::QSet_QDateTime>("QSet_QDateTime");
static const int register_QSet_QPersistentModelIndex =
  qRegisterMetaType<::QSet_QPersistentModelIndex>("QSet_QPersistentModelIndex");
static const int register_QSet_QString =
  qRegisterMetaType<::QSet_QString>("QSet_QString");
static const int register_QSet_QTime =
  qRegisterMetaType<::QSet_QTime>("QSet_QTime");
static const int register_QSet_QUrl =
  qRegisterMetaType<::QSet_QUrl>("QSet_QUrl");
static const int register_QSet_u8 = qRegisterMetaType<::QSet_u8>("QSet_u8");
static const int register_QSet_u16 = qRegisterMetaType<::QSet_u16>("QSet_u16");
static const int register_QSet_u32 = qRegisterMetaType<::QSet_u32>("QSet_u32");
static const int register_QSet_u64 = qRegisterMetaType<::QSet_u64>("QSet_u64");
