// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qmap.h"

#include "../../assertion_utils.h"

#define CXX_QT_QMAP_ASSERTS(keyTypeName, valueTypeName, combinedName)          \
  assert_alignment_and_size(                                                   \
    QMap_##combinedName, alignof(::std::size_t), sizeof(::std::size_t));       \
                                                                               \
  static_assert(                                                               \
    !::std::is_trivially_copy_assignable<QMap_##combinedName>::value);         \
  static_assert(                                                               \
    !::std::is_trivially_copy_constructible<QMap_##combinedName>::value);      \
  static_assert(                                                               \
    !::std::is_trivially_destructible<QMap_##combinedName>::value);            \
                                                                               \
  static_assert(QTypeInfo<QMap_##combinedName>::isRelocatable);                \
                                                                               \
  static_assert(::std::is_copy_assignable<keyTypeName>::value);                \
  static_assert(::std::is_copy_constructible<keyTypeName>::value);             \
  static_assert(::std::is_copy_assignable<valueTypeName>::value);              \
  static_assert(::std::is_copy_constructible<valueTypeName>::value);

CXX_QT_QMAP_ASSERTS(QString, QVariant, QString_QVariant);

// Ensure that QMap<QString, QVariant> (aka QVariantMap) is registered
// otherwise it cannot be used in QML
static const int register_QMap_QString_QVariant =
  qRegisterMetaType<::QMap_QString_QVariant>("QMap_QString_QVariant");
