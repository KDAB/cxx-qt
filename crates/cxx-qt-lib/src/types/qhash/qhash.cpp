// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qhash.h"

#include "../assertion_utils.h"

#define CXX_QT_QHASH_ASSERTS(keyTypeName, valueTypeName, combinedName)         \
  assert_alignment_and_size(                                                   \
    QHash_##combinedName, alignof(::std::size_t), sizeof(::std::size_t));      \
                                                                               \
  static_assert(                                                               \
    !::std::is_trivially_copy_assignable<QHash_##combinedName>::value);        \
  static_assert(                                                               \
    !::std::is_trivially_copy_constructible<QHash_##combinedName>::value);     \
  static_assert(                                                               \
    !::std::is_trivially_destructible<QHash_##combinedName>::value);           \
                                                                               \
  static_assert(QTypeInfo<QHash_##combinedName>::isRelocatable);               \
                                                                               \
  static_assert(::std::is_copy_assignable<keyTypeName>::value);                \
  static_assert(::std::is_copy_constructible<keyTypeName>::value);             \
  static_assert(::std::is_copy_assignable<valueTypeName>::value);              \
  static_assert(::std::is_copy_constructible<valueTypeName>::value);

CXX_QT_QHASH_ASSERTS(QString, QVariant, QString_QVariant);
CXX_QT_QHASH_ASSERTS(::std::int32_t, QByteArray, i32_QByteArray);
