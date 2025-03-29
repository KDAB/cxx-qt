// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cinttypes>

#include <QtCore/QString>
#include <QtCore/QTime>

namespace rust {
namespace cxxqtlib1 {

QTime (*qtimeCurrentTime)() = QTime::currentTime;
QTime (*qtimeFromMSecsSinceStartOfDay)(::std::int32_t) =
  QTime::fromMSecsSinceStartOfDay;

bool (*qtimeIsValid)(int, int, int, int) = QTime::isValid;

// In Qt 5 t is const-ref, in Qt 6 it is value
::std::int32_t
qtimeMSecsTo(const QTime& time, QTime t);
QTime
qtimeFromString(const QString& string, const QString& format);
QTime
qtimeFromString(const QString& string, Qt::DateFormat format);
// In Qt 5 t is const-ref, in Qt 6 it is value
::std::int32_t
qtimeSecsTo(const QTime& time, QTime t);

}
}
