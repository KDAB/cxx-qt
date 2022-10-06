// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qdate.h"

#include "assertion_utils.h"

#include <cstdint>

// QDate has a single 64-Bit integer member
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v5.15.6-lts-lgpl#n176
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v6.2.4#n147
assert_alignment_and_size(QDate, alignof(std::int64_t), sizeof(std::int64_t));

static_assert(std::is_trivially_copyable<QDate>::value,
              "QDate must be trivially copyable!");

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
