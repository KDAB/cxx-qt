// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qdatetime.h"

#include <cxx-qt-lib/assertion_utils.h>

// QDateTime has a single member, which is a union, with the largest member
// being a pointer
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v5.15.6-lts-lgpl#n426
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v5.15.6-lts-lgpl#n270
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v6.2.4#n394
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v6.2.4#n255
assert_alignment_and_size(QDateTime, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QDateTime>::value);
static_assert(!::std::is_trivially_copy_constructible<QDateTime>::value);
static_assert(!::std::is_trivially_destructible<QDateTime>::value);

static_assert(QTypeInfo<QDateTime>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

QDateTime
qdatetimeAddDays(const QDateTime& datetime, ::std::int64_t ndays)
{
  return datetime.addDays(static_cast<qint64>(ndays));
}

QDateTime
qdatetimeAddMSecs(const QDateTime& datetime, ::std::int64_t msecs)
{
  return datetime.addMSecs(static_cast<qint64>(msecs));
}

QDateTime
qdatetimeAddSecs(const QDateTime& datetime, ::std::int64_t secs)
{
  return datetime.addSecs(static_cast<qint64>(secs));
}

QDateTime
qdatetimeCurrentDateTime()
{
  return QDateTime::currentDateTime();
}

QDateTime
qdatetimeCurrentDateTimeUtc()
{
  return QDateTime::currentDateTimeUtc();
}

::std::int64_t
qdatetimeCurrentMSecsSinceEpoch()
{
  return QDateTime::currentMSecsSinceEpoch();
}

::std::int64_t
qdatetimeCurrentSecsSinceEpoch()
{
  return QDateTime::currentSecsSinceEpoch();
}

::std::int64_t
qdatetimeDaysTo(const QDateTime& datetime, const QDateTime& other)
{
  return static_cast<::std::int64_t>(datetime.daysTo(other));
}

QDateTime
qdatetimeFromMSecsSinceEpoch(::std::int64_t msecs, const QTimeZone& timeZone)
{
  return QDateTime::fromMSecsSinceEpoch(static_cast<qint64>(msecs), timeZone);
}

QDateTime
qdatetimeFromSecsSinceEpoch(::std::int64_t secs, const QTimeZone& timeZone)
{
  return QDateTime::fromSecsSinceEpoch(static_cast<qint64>(secs), timeZone);
}

::std::int64_t
qdatetimeMSecsTo(const QDateTime& datetime, const QDateTime& other)
{

  return static_cast<::std::int64_t>(datetime.msecsTo(other));
}

::std::int64_t
qdatetimeSecsTo(const QDateTime& datetime, const QDateTime& other)
{

  return static_cast<::std::int64_t>(datetime.secsTo(other));
}

void
qdatetimeSetDate(QDateTime& datetime, QDate date)
{
  datetime.setDate(date);
}

void
qdatetimeSetMSecsSinceEpoch(QDateTime& datetime, ::std::int64_t msecs)
{
  datetime.setMSecsSinceEpoch(static_cast<qint64>(msecs));
}

void
qdatetimeSetSecsSinceEpoch(QDateTime& datetime, ::std::int64_t secs)
{
  datetime.setSecsSinceEpoch(static_cast<qint64>(secs));
}

void
qdatetimeSetTime(QDateTime& datetime, QTime time)
{
  datetime.setTime(time);
}

::std::unique_ptr<QTimeZone>
qdatetimeTimeZone(const QDateTime& datetime)
{
  return ::std::make_unique<QTimeZone>(datetime.timeZone());
}

::std::int64_t
qdatetimeToMSecsSinceEpoch(const QDateTime& datetime)
{
  return static_cast<::std::int64_t>(datetime.toMSecsSinceEpoch());
}

::std::int64_t
qdatetimeToSecsSinceEpoch(const QDateTime& datetime)
{
  return static_cast<::std::int64_t>(datetime.toSecsSinceEpoch());
}

void
qdatetimeSetTimeZone(QDateTime& datetime, const QTimeZone& timeZone)
{
#if (QT_VERSION >= QT_VERSION_CHECK(6, 7, 0))
  datetime.setTimeZone(timeZone,
                       QDateTime::TransitionResolution::LegacyBehavior);
#else
  datetime.setTimeZone(timeZone);
#endif
}

QDateTime
qdatetimeFromQString(const QString& string, const Qt::DateFormat format)
{
  return QDateTime::fromString(string, format);
}

QString
qdatetimeToFormat(const QDateTime& datetime, Qt::DateFormat format)
{
  return datetime.toString(format);
}

}
}
