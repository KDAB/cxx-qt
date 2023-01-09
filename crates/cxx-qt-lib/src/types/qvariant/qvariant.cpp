// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qvariant.h"

#include <QtCore/QMetaObject>

#include "../assertion_utils.h"

// The layout has changed between Qt 5 and Qt 6
//
// Qt5 QVariant has one member, which contains three uints and a union.
// The three uints are optimised to a reduced size, resulting in a combined size
// of two pointers.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qvariant.h?h=v5.15.6-lts-lgpl#n491
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qvariant.h?h=v5.15.6-lts-lgpl#n411
//
// Qt6 QVariant has one member, which contains three pointers and a union
// (with a pointer as the largest member)
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qvariant.h?h=v6.2.4#n540
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qvariant.h?h=v6.2.4#n474
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
assert_alignment_and_size(QVariant,
                          alignof(::std::size_t),
                          sizeof(::std::size_t[4]));
#else
assert_alignment_and_size(QVariant,
                          alignof(::std::size_t),
                          sizeof(::std::size_t[2]));
#endif

static_assert(!::std::is_trivially_copy_assignable<QVariant>::value);
static_assert(!::std::is_trivially_copy_constructible<QVariant>::value);

// Ensure that trivially destructible is correct
// If this is false then we need to manually implement Drop rather than derive
static_assert(!::std::is_trivially_destructible<QVariant>::value);

static_assert(QTypeInfo<QVariant>::isRelocatable);

// Need to use a macro here as we can't template because the types
// are always QVariant and bool. So then CXX can't decide which to use.
#define CXX_QT_QVARIANT_CAN_CONVERT_IMPL(typeName, name)                       \
  bool qvariantCanConvert##name(const QVariant& variant)                       \
  {                                                                            \
    return variant.canConvert<typeName>();                                     \
  }

namespace rust {
namespace cxxqtlib1 {
namespace qvariant {

CXX_QT_QVARIANT_CAN_CONVERT_IMPL(bool, Bool)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(float, F32)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(double, F64)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(::std::int8_t, I8)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(::std::int16_t, I16)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(::std::int32_t, I32)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(::std::int64_t, I64)
#ifdef CXX_QT_GUI_FEATURE
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QColor, QColor)
#endif
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QDate, QDate)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QDateTime, QDateTime)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QPoint, QPoint)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QPointF, QPointF)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QRect, QRect)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QRectF, QRectF)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QSize, QSize)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QSizeF, QSizeF)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QString, QString)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QTime, QTime)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(QUrl, QUrl)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(::std::uint8_t, U8)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(::std::uint16_t, U16)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(::std::uint32_t, U32)
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(::std::uint64_t, U64)

}
}
}
