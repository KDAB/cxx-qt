// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cinttypes>

#include <QtCore/QDate>
#include <QtCore/QString>

namespace rust {
namespace cxxqtlib1 {

inline QDate (*qdateCurrentDate)() = QDate::currentDate;

inline bool (*qdateIsLeapYear)(int) = QDate::isLeapYear;

QDate
qdateFromString(const QString& string, const QString& format);
QDate
qdateFromString(const QString& string, Qt::DateFormat format);
// In Qt 5 d is const-ref, in Qt 6 it is value
qint64
qdateDaysTo(const QDate& date, QDate d);
QString
qdateToFormat(const QDate& date, const QString& format);

}
}
