// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qsize.h"

#include "assertion_utils.h"

#include <cstdint>

// QSize has two "int" members
// Rust represents these as 32-bit integers.
// https://codebrowser.dev/qt5/qtbase/src/corelib/tools/qsize.h.html#QSize::wd
//
// https://codebrowser.dev/qt6/qtbase/src/corelib/tools/qsize.h.html#QSize::wd
assert_alignment_and_size(QSize,
                          alignof(std::int32_t),
                          sizeof(std::int32_t[2]));

static_assert(std::is_trivially_copyable<QSize>::value,
              "QSize must be trivially copyable!");

namespace rust {
namespace cxxqtlib1 {

QSize
qsizeInitDefault()
{
  return QSize();
}

QSize
qsizeInit(int width, int height)
{
  return QSize(width, height);
}

}
}
