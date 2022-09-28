// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QDateTime>

#include "rust/cxx.h"

template<>
struct rust::IsRelocatable<QDateTime> : std::true_type
{
};
static_assert(QTypeInfo<QDateTime>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

void
qdatetimeDrop(QDateTime& datetime);
QDateTime
qdatetimeInitDefault();
QDateTime
qdatetimeInitFromDateAndTime(const QDate& date, const QTime& time);
QDateTime
qdatetimeInitFromQDateTime(const QDateTime& datetime);
void
qdatetimeSetDate(QDateTime& datetime, QDate date);
void
qdatetimeSetTime(QDateTime& datetime, QTime time);

}
}
