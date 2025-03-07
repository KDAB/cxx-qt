// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qpolygon.h"

#include <cxx-qt-lib/assertion_utils.h>

// The layout has changed between Qt 5 and Qt 6
//
// Qt5 QPolygon has one pointer as a member
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qlist.h?h=v5.15.6-lts-lgpl#n157
//
// Qt6 QPolygon  has one member, which contains two pointers and a size_t
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qlist.h?h=v6.2.4#n110
// DataPointer is then a QArrayDataPointer<QPoint>
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qarraydatapointer.h?h=v6.2.4#n390
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
assert_alignment_and_size(QPolygon, {
  ::std::size_t a0;
  ::std::size_t a1;
  ::std::size_t a2;
});
#else
assert_alignment_and_size(QPolygon, { ::std::size_t a0; });
#endif

static_assert(!::std::is_trivially_copy_assignable<QPolygon>::value);
static_assert(!::std::is_trivially_copy_constructible<QPolygon>::value);
static_assert(!::std::is_trivially_destructible<QPolygon>::value);

static_assert(QTypeInfo<QPolygon>::isRelocatable);
