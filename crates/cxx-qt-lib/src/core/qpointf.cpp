// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qpointf.h"

#include "../assertion_utils.h"

// QPointF has two "qreal" members - xp and yp
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qpoint.h?h=v5.15.6-lts-lgpl#n271
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qpoint.h?h=v6.2.4#n313
constexpr static ::std::array<::std::size_t, 2> arr{ sizeof(double),
                                                     sizeof(double) };
assert_alignment_and_size(QPointF, alignof(double), arr);

static_assert(::std::is_trivially_copyable<QPointF>::value,
              "QPointF should be trivially copyable");

namespace rust {
namespace cxxqtlib1 {

double
qpointfDotProduct(const QPointF& p1, const QPointF& p2)
{
  return QPointF::dotProduct(p1, p2);
}

}
}
