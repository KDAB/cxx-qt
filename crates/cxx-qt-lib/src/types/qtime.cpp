// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qtime.h"

#include "assertion_utils.h"

#include <cstdint>
// QTime has one int member
// https://codebrowser.dev/qt5/qtbase/src/corelib/time/qdatetime.h.html#QTime::mds
//
// https://codebrowser.dev/qt6/qtbase/src/corelib/time/qdatetime.h.html#QTime::mds
assert_alignment_and_size(QTime, alignof(std::int32_t), sizeof(std::int32_t));

static_assert(std::is_trivially_copyable<QTime>::value,
              "QTime must be trivially copyable!");

namespace rust {
namespace cxxqtlib1 {

QTime
qtimeInitDefault()
{
  return QTime();
}

QTime
qtimeInit(int h, int m, int s, int ms)
{
  return QTime(h, m, s, ms);
}

}
}
