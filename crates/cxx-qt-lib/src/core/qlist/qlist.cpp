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
