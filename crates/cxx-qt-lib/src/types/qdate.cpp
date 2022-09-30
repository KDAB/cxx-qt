// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qdate.h"

#include "assertion_utils.h"

#include <cstdint>

// QDate has a single 64-Bit integer member
// https://codebrowser.dev/qt5/qtbase/src/corelib/time/qdatetime.h.html#QDate::jd
//
// https://codebrowser.dev/qt6/qtbase/src/corelib/time/qdatetime.h.html#QDate::jd
assert_alignment_and_size(QDate, alignof(std::int64_t), sizeof(std::int64_t));

static_assert(std::is_trivially_copy_assignable<QDate>::value);
static_assert(std::is_trivially_copy_constructible<QDate>::value);

static_assert(std::is_trivially_destructible<QDate>::value);

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
