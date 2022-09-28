// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qdate.h"

namespace rust {
namespace cxxqtlib1 {

QDate
qdateInitDefault()
{
  return QDate();
}

QDate
qdateInit(int y, int m, int d)
{
  return QDate(y, m, d);
}

}
}
