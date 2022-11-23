// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qset.h"

#include "../assertion_utils.h"

#define CXX_QT_QSET_ASSERTS(typeName, name)                                    \
  assert_alignment_and_size(                                                   \
    QSet_##name, alignof(std::size_t), sizeof(std::size_t));                   \
                                                                               \
  static_assert(!std::is_trivially_copy_assignable<QSet_##name>::value);       \
  static_assert(!std::is_trivially_copy_constructible<QSet_##name>::value);    \
  static_assert(!std::is_trivially_destructible<QSet_##name>::value);          \
                                                                               \
  static_assert(QTypeInfo<QSet_##name>::isRelocatable);                        \
                                                                               \
  static_assert(std::is_copy_assignable<typeName>::value);                     \
  static_assert(std::is_copy_constructible<typeName>::value);

CXX_QT_QSET_ASSERTS(bool, bool);
CXX_QT_QSET_ASSERTS(float, f32);
CXX_QT_QSET_ASSERTS(double, f64);
CXX_QT_QSET_ASSERTS(::qint8, i8);
CXX_QT_QSET_ASSERTS(::qint16, i16);
CXX_QT_QSET_ASSERTS(::qint32, i32);
CXX_QT_QSET_ASSERTS(::QDate, QDate);
CXX_QT_QSET_ASSERTS(::QDateTime, QDateTime);
CXX_QT_QSET_ASSERTS(::QString, QString);
CXX_QT_QSET_ASSERTS(::QTime, QTime);
CXX_QT_QSET_ASSERTS(::QUrl, QUrl);
CXX_QT_QSET_ASSERTS(::quint8, u8);
CXX_QT_QSET_ASSERTS(::quint16, u16);
CXX_QT_QSET_ASSERTS(::quint32, u32);
