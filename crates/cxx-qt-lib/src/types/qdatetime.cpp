// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qdatetime.h"

#include "assertion_utils.h"

// QDateTime has a single member, which is a union, with the largest member
// being a pointer
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v5.15.6-lts-lgpl#n426
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v5.15.6-lts-lgpl#n270
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v6.2.4#n394
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v6.2.4#n255
assert_alignment_and_size(QDateTime, alignof(std::size_t), sizeof(std::size_t));

static_assert(!std::is_trivially_copy_assignable<QDateTime>::value);
static_assert(!std::is_trivially_copy_constructible<QDateTime>::value);
static_assert(!std::is_trivially_destructible<QDateTime>::value);

namespace rust {
namespace cxxqtlib1 {

void
qdatetimeDrop(QDateTime& datetime)
{
  return datetime.~QDateTime();
}

QDateTime
qdatetimeInitDefault()
{
  return QDateTime();
}

QDateTime
qdatetimeInitFromDateAndTime(const QDate& date, const QTime& time)
{
  return QDateTime(date, time);
}

QDateTime
qdatetimeInitFromQDateTime(const QDateTime& datetime)
{
  return QDateTime(datetime);
}

void
qdatetimeSetDate(QDateTime& datetime, QDate date)
{
  datetime.setDate(date);
}

void
qdatetimeSetTime(QDateTime& datetime, QTime time)
{
  datetime.setTime(time);
}
}
}
