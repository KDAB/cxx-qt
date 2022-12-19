// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qt5list.h"

#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))
#include "../assertion_utils.h"

#define CXX_QT_QT5LIST_ASSERTS(typeName, name)                                 \
  assert_alignment_and_size(                                                   \
    Qt5List_##name, alignof(std::size_t), sizeof(std::size_t));                \
                                                                               \
  static_assert(!std::is_trivially_copy_assignable<Qt5List_##name>::value);    \
  static_assert(!std::is_trivially_copy_constructible<Qt5List_##name>::value); \
  static_assert(!std::is_trivially_destructible<Qt5List_##name>::value);       \
                                                                               \
  static_assert(QTypeInfo<Qt5List_##name>::isRelocatable);                     \
                                                                               \
  static_assert(std::is_copy_assignable<typeName>::value);                     \
  static_assert(std::is_copy_constructible<typeName>::value);

CXX_QT_QT5LIST_ASSERTS(bool, bool);
CXX_QT_QT5LIST_ASSERTS(float, f32);
CXX_QT_QT5LIST_ASSERTS(double, f64);
CXX_QT_QT5LIST_ASSERTS(::std::int8_t, i8);
CXX_QT_QT5LIST_ASSERTS(::std::int16_t, i16);
CXX_QT_QT5LIST_ASSERTS(::std::int32_t, i32);
CXX_QT_QT5LIST_ASSERTS(::std::int64_t, i64);
CXX_QT_QT5LIST_ASSERTS(::QColor, QColor);
CXX_QT_QT5LIST_ASSERTS(::QDate, QDate);
CXX_QT_QT5LIST_ASSERTS(::QDateTime, QDateTime);
CXX_QT_QT5LIST_ASSERTS(::QPoint, QPoint);
CXX_QT_QT5LIST_ASSERTS(::QPointF, QPointF);
CXX_QT_QT5LIST_ASSERTS(::QRect, QRect);
CXX_QT_QT5LIST_ASSERTS(::QRectF, QRectF);
CXX_QT_QT5LIST_ASSERTS(::QSize, QSize);
CXX_QT_QT5LIST_ASSERTS(::QSizeF, QSizeF);
CXX_QT_QT5LIST_ASSERTS(::QString, QString);
CXX_QT_QT5LIST_ASSERTS(::QTime, QTime);
CXX_QT_QT5LIST_ASSERTS(::QUrl, QUrl);
CXX_QT_QT5LIST_ASSERTS(::QVariant, QVariant);
CXX_QT_QT5LIST_ASSERTS(::std::uint8_t, u8);
CXX_QT_QT5LIST_ASSERTS(::std::uint16_t, u16);
CXX_QT_QT5LIST_ASSERTS(::std::uint32_t, u32);
CXX_QT_QT5LIST_ASSERTS(::std::uint64_t, u64);

#endif
