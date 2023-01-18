// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qtimezone.h"

#include "../assertion_utils.h"

// QTimeZone is a single QSharedDataPointer to a QTimeZonePrivate
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qtimezone.h?h=v5.15.6-lts-lgpl#n171
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qtimezone.h?h=v6.2.4#n175
assert_alignment_and_size(QTimeZone,
                          alignof(::std::size_t),
                          sizeof(::std::size_t));

static_assert(!::std::is_trivially_copy_assignable<QTimeZone>::value);
static_assert(!::std::is_trivially_copy_constructible<QTimeZone>::value);

static_assert(!::std::is_trivially_destructible<QTimeZone>::value);

static_assert(QTypeInfo<QTimeZone>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

QList<QByteArray>
qtimezoneAvailableTimeZoneIds()
{
  return QTimeZone::availableTimeZoneIds();
}

QTimeZone
qtimezoneFromOffsetSeconds(::std::int32_t offsetSeconds)
{
  return QTimeZone(static_cast<int>(offsetSeconds));
}

QTimeZone
qtimezoneFromIana(const QByteArray& ianaId)
{
  return QTimeZone(ianaId);
}

QTimeZone
qtimezoneSystemTimeZone()
{
  return QTimeZone::systemTimeZone();
}

QByteArray
qtimezoneSystemTimeZoneId()
{
  return QTimeZone::systemTimeZoneId();
}

QTimeZone
qtimezoneUtc()
{
  return QTimeZone::utc();
}

}
}
