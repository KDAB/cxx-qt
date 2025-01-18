// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qtimezone.h"

namespace rust {
namespace cxxqtlib1 {

QList<QByteArray>
qtimezoneAvailableTimeZoneIds()
{
  return QTimeZone::availableTimeZoneIds();
}

::std::unique_ptr<QTimeZone>
qtimezoneClone(const QTimeZone& timezone)
{
  return ::std::make_unique<QTimeZone>(timezone);
}

::std::unique_ptr<QTimeZone>
qtimezoneDefault()
{
  return ::std::make_unique<QTimeZone>();
}

QString
qtimezoneDisplayName(const QTimeZone& timezone,
                     QTimeZoneTimeType timeType,
                     QTimeZoneNameType nameType)
{
  return timezone.displayName(timeType, nameType);
}

::std::unique_ptr<QTimeZone>
qtimezoneFromOffsetSeconds(::std::int32_t offsetSeconds)
{
  return ::std::make_unique<QTimeZone>(static_cast<int>(offsetSeconds));
}

::std::unique_ptr<QTimeZone>
qtimezoneFromIana(const QByteArray& ianaId)
{
  return ::std::make_unique<QTimeZone>(ianaId);
}

::std::unique_ptr<QTimeZone>
qtimezoneSystemTimeZone()
{
  return ::std::make_unique<QTimeZone>(QTimeZone::systemTimeZone());
}

QByteArray
qtimezoneSystemTimeZoneId()
{
  return QTimeZone::systemTimeZoneId();
}

::std::unique_ptr<QTimeZone>
qtimezoneUtc()
{
  return ::std::make_unique<QTimeZone>(QTimeZone::utc());
}

}
}
