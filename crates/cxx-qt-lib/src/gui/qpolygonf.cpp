// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qpolygonf.h"

#include <cxx-qt-lib/assertion_utils.h>

// The layout has changed between Qt 5 and Qt 6
//
// Qt5 QPolygonF has one pointer as a member
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qlist.h?h=v5.15.6-lts-lgpl#n157
//
// Qt6 QPolygonF  has one member, which contains two pointers and a size_t
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qlist.h?h=v6.2.4#n110
// DataPointer is then a QArrayDataPointer<QPoint>
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qarraydatapointer.h?h=v6.2.4#n390
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
assert_alignment_and_size(QPolygonF, {
  ::std::size_t a0;
  ::std::size_t a1;
  ::std::size_t a2;
});
#else
assert_alignment_and_size(QPolygonF, { ::std::size_t a0; });
#endif

static_assert(!::std::is_trivially_copy_assignable<QPolygonF>::value);
static_assert(!::std::is_trivially_copy_constructible<QPolygonF>::value);
static_assert(!::std::is_trivially_destructible<QPolygonF>::value);

static_assert(QTypeInfo<QPolygonF>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {
const QList<QPointF>&
qpolygonfAsQListQPointFRef(const QPolygonF& shape)
{
  return static_cast<const QList<QPointF>&>(shape);
}

QList<QPointF>&
qpolygonfAsQListQPointFRefMut(QPolygonF& shape)
{
  return static_cast<QList<QPointF>&>(shape);
}

}
}
