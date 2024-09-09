// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qtime.h"

#include <cxx-qt-lib/assertion_utils.h>

#include <cstdint>
// QTime has one int member
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v5.15.6-lts-lgpl#n242
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v6.2.4#n217
assert_alignment_and_size(QTime, { ::std::int32_t a0; });

static_assert(::std::is_trivially_copyable<QTime>::value,
              "QTime must be trivially copyable!");

namespace rust {
namespace cxxqtlib1 {

QTime
qtimeCurrentTime()
{
  return QTime::currentTime();
}

QTime
qtimeFromMSecsSinceStartOfDay(::std::int32_t msecs)
{
  return QTime::fromMSecsSinceStartOfDay(static_cast<int>(msecs));
}

::std::int32_t
qtimeMSecsTo(const QTime& time, QTime t)
{
  // In Qt 5 t is const-ref, in Qt 6 it is value
  return static_cast<::std::int32_t>(time.msecsTo(t));
}

QTime
qtimeFromString(const QString& string, const QString& format)
{
  return QTime::fromString(string, format);
}

QTime
qtimeFromString(const QString& string, Qt::DateFormat format)
{
  return QTime::fromString(string, format);
}

::std::int32_t
qtimeSecsTo(const QTime& time, QTime t)
{
  // In Qt 5 t is const-ref, in Qt 6 it is value
  return static_cast<::std::int32_t>(time.secsTo(t));
}

bool
qtimeIsValid(int h, int m, int s, int ms)
{
  return QTime::isValid(h, m, s, ms);
}

}
}
