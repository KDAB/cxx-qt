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
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v5.15.6-lts-lgpl#n242
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v6.2.4#n217
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
