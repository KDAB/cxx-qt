// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qset.h"

#include <cxx-qt-lib/assertion_utils.h>

#define CXX_QT_QSET_ASSERTS(typeName, name)                                    \
  assert_alignment_and_size(QSet_##name, { ::std::size_t a0; });               \
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
CXX_QT_QSET_ASSERTS(::QUuid, QUuid);
CXX_QT_QSET_ASSERTS(::std::uint8_t, u8);
CXX_QT_QSET_ASSERTS(::std::uint16_t, u16);
CXX_QT_QSET_ASSERTS(::std::uint32_t, u32);
CXX_QT_QSET_ASSERTS(::std::uint64_t, u64);
