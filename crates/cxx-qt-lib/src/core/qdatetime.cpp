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
qdatetimeCurrentDateTime()
{
  return QDateTime::currentDateTime();
}

QDateTime
qdatetimeCurrentDateTimeUtc()
{
  return QDateTime::currentDateTimeUtc();
}

qint64
qdatetimeCurrentMSecsSinceEpoch()
{
  return QDateTime::currentMSecsSinceEpoch();
}

qint64
qdatetimeCurrentSecsSinceEpoch()
{
  return QDateTime::currentSecsSinceEpoch();
}

QDateTime
qdatetimeFromMSecsSinceEpoch(qint64 msecs, const QTimeZone& timeZone)
{
  return QDateTime::fromMSecsSinceEpoch(msecs, timeZone);
}

QDateTime
qdatetimeFromSecsSinceEpoch(qint64 secs, const QTimeZone& timeZone)
{
  return QDateTime::fromSecsSinceEpoch(secs, timeZone);
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

::std::unique_ptr<QTimeZone>
qdatetimeTimeZone(const QDateTime& datetime)
{
  return ::std::make_unique<QTimeZone>(datetime.timeZone());
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

QDateTime
qdatetimeFromQString(const QString& string, const QString& format)
{
  return QDateTime::fromString(string, format);
}

QString
qdatetimeToQString(const QDateTime& date, const QString& format)
{
  return date.toString(format);
}

QString
qdatetimeToQString(const QDateTime& date, Qt::DateFormat format)
{
  return date.toString(format);
}

}
}
