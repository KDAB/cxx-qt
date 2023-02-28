// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qpoint.h"

#include "../assertion_utils.h"

// QPoint has "int" members - xp and yp
// Rust represents these as 32-bit integer types.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qpoint.h?h=v5.15.6-lts-lgpl#n271
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qpoint.h?h=v6.2.4#n313
assert_alignment_and_size(QPoint,
                          alignof(::std::int32_t),
                          sizeof(::std::int32_t[2]));

static_assert(::std::is_trivially_copyable<QPoint>::value);

namespace rust {
namespace cxxqtlib1 {

::std::int32_t
qpointDotProduct(const QPoint& p1, const QPoint& p2)
{
  return QPoint::dotProduct(p1, p2);
}

}
}
