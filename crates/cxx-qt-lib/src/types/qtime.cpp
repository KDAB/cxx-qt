// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qtime.h"

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
