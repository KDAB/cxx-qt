// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cinttypes>

#include <QtCore/QByteArray>
#include <QtCore/QList>
#include <QtCore/QTimeZone>

#include "rust/cxx.h"

// This has static asserts in the cpp file to ensure this is valid.
template<>
struct rust::IsRelocatable<QTimeZone> : ::std::true_type
{
};

namespace rust {
namespace cxxqtlib1 {

QList<QByteArray>
qtimezoneAvailableTimeZoneIds();
QTimeZone
qtimezoneFromOffsetSeconds(::std::int32_t offsetSeconds);
QTimeZone
qtimezoneFromIana(const QByteArray& ianaId);
QTimeZone
qtimezoneSystemTimeZone();
QByteArray
qtimezoneSystemTimeZoneId();
QTimeZone
qtimezoneUtc();

}
}
