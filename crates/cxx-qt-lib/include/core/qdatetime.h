// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cinttypes>
#include <memory>

#include <QtCore/QDateTime>
#include <QtCore/QTimeZone>

#include "rust/cxx.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<QDateTime> : ::std::true_type
{};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {

QDateTime
qdatetimeAddDays(const QDateTime& datetime, ::std::int64_t ndays);
QDateTime
qdatetimeAddMSecs(const QDateTime& datetime, ::std::int64_t msecs);
QDateTime
qdatetimeAddSecs(const QDateTime& datetime, ::std::int64_t secs);
QDateTime
qdatetimeCurrentDateTime();
QDateTime
qdatetimeCurrentDateTimeUtc();
::std::int64_t
qdatetimeCurrentMSecsSinceEpoch();
::std::int64_t
qdatetimeCurrentSecsSinceEpoch();
::std::int64_t
qdatetimeDaysTo(const QDateTime& datetime, const QDateTime& other);
QDateTime
qdatetimeFromMSecsSinceEpoch(::std::int64_t msecs, const QTimeZone& timeZone);
QDateTime
qdatetimeFromSecsSinceEpoch(::std::int64_t secs, const QTimeZone& timeZone);
::std::int64_t
qdatetimeMSecsTo(const QDateTime& datetime, const QDateTime& other);
::std::int64_t
qdatetimeSecsTo(const QDateTime& datetime, const QDateTime& other);
void
qdatetimeSetDate(QDateTime& datetime, QDate date);
void
qdatetimeSetMSecsSinceEpoch(QDateTime& datetime, ::std::int64_t msecs);
void
qdatetimeSetSecsSinceEpoch(QDateTime& datetime, ::std::int64_t secs);
void
qdatetimeSetTime(QDateTime& datetime, QTime time);
::std::unique_ptr<QTimeZone>
qdatetimeTimeZone(const QDateTime& datetime);
::std::int64_t
qdatetimeToMSecsSinceEpoch(const QDateTime& datetime);
::std::int64_t
qdatetimeToSecsSinceEpoch(const QDateTime& datetime);
void
qdatetimeSetTimeZone(QDateTime& datetime, const QTimeZone& timeZone);
QDateTime
qdatetimeFromQString(const QString& string, Qt::DateFormat format);
}
}
