// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cinttypes>
#include <memory>

#include <QtCore/QByteArray>
#include <QtCore/QList>
#include <QtCore/QTimeZone>

namespace rust {
namespace cxxqtlib1 {

QList<QByteArray>
qtimezoneAvailableTimeZoneIds();
::std::unique_ptr<QTimeZone>
qtimezoneClone(const QTimeZone& timezone);
::std::unique_ptr<QTimeZone>
qtimezoneDefault();
::std::unique_ptr<QTimeZone>
qtimezoneFromOffsetSeconds(::std::int32_t offsetSeconds);
::std::unique_ptr<QTimeZone>
qtimezoneFromIana(const QByteArray& ianaId);
::std::unique_ptr<QTimeZone>
qtimezoneSystemTimeZone();
QByteArray
qtimezoneSystemTimeZoneId();
::std::unique_ptr<QTimeZone>
qtimezoneUtc();

}
}
