// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qvector.h"

#include "../assertion_utils.h"

#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
#define CXX_QT_QVECTOR_ALIGN_AND_SIZE(typeName, name)                          \
  assert_alignment_and_size(                                                   \
    QVector_##name, alignof(std::size_t), sizeof(std::size_t[3]));
#else
#define CXX_QT_QVECTOR_ALIGN_AND_SIZE(typeName, name)                          \
  assert_alignment_and_size(                                                   \
    QVector_##name, alignof(std::size_t), sizeof(std::size_t));
#endif

#define CXX_QT_QVECTOR_ASSERTS(typeName, name)                                 \
  CXX_QT_QVECTOR_ALIGN_AND_SIZE(typeName, name);                               \
                                                                               \
  static_assert(!std::is_trivially_copy_assignable<QVector_##name>::value);    \
  static_assert(!std::is_trivially_copy_constructible<QVector_##name>::value); \
  static_assert(!std::is_trivially_destructible<QVector_##name>::value);       \
                                                                               \
  static_assert(QTypeInfo<QVector_##name>::isRelocatable);                     \
                                                                               \
  static_assert(std::is_copy_assignable<typeName>::value);                     \
  static_assert(std::is_copy_constructible<typeName>::value);

CXX_QT_QVECTOR_ASSERTS(bool, bool);
CXX_QT_QVECTOR_ASSERTS(float, f32);
CXX_QT_QVECTOR_ASSERTS(double, f64);
CXX_QT_QVECTOR_ASSERTS(::std::int8_t, i8);
CXX_QT_QVECTOR_ASSERTS(::std::int16_t, i16);
CXX_QT_QVECTOR_ASSERTS(::std::int32_t, i32);
CXX_QT_QVECTOR_ASSERTS(::std::int64_t, i64);
CXX_QT_QVECTOR_ASSERTS(::QColor, QColor);
CXX_QT_QVECTOR_ASSERTS(::QDate, QDate);
CXX_QT_QVECTOR_ASSERTS(::QDateTime, QDateTime);
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
