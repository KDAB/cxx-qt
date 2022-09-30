// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qpoint.h"

#include "assertion_utils.h"

#include <cstdint>

// QPoint has "int" members - xp and yp
// Rust represents these as 32-bit integer types.
// https://codebrowser.dev/qt5/qtbase/src/corelib/tools/qpoint.h.html#QPoint::xp
//
// https://codebrowser.dev/qt6/qtbase/src/corelib/tools/qpoint.h.html#QPoint::xp
assert_alignment_and_size(QPoint,
                          alignof(std::int32_t),
                          sizeof(std::int32_t[2]));

static_assert(std::is_trivially_copyable<QPoint>::value);

namespace rust {
namespace cxxqtlib1 {

QPoint
qpointInitDefault()
{
  return QPoint();
}

QPoint
qpointInit(int x, int y)
{
  return QPoint(x, y);
}

}
}
