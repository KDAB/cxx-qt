// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qrect.h"

#include "assertion_utils.h"

#include <cstdint>

// QRect has 4 int members.
// Rust represents them as 4 32-bit integers
// https://codebrowser.dev/qt5/qtbase/src/corelib/tools/qrect.h.html#QRect::x1
//
// https://codebrowser.dev/qt6/qtbase/src/corelib/tools/qrect.h.html#QRect::x1
assert_alignment_and_size(QRect,
                          alignof(std::int32_t),
                          sizeof(std::int32_t[4]));

static_assert(std::is_trivially_copyable<QRect>::value,
              "QRect must be trivially copyable");

namespace rust {
namespace cxxqtlib1 {

QRect
qrectInitDefault()
{
  return QRect();
}

QRect
qrectInit(int x, int y, int w, int h)
{
  return QRect(x, y, w, h);
}

}
}
