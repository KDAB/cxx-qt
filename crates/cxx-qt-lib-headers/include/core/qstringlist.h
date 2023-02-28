// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QList>
#include <QtCore/QString>
#include <QtCore/QStringList>

#include "rust/cxx.h"

template<>
struct rust::IsRelocatable<QStringList> : ::std::true_type
{
};

namespace rust {
namespace cxxqtlib1 {

QStringList
qstringlistFromQListQString(const QList<QString>& list);
QList<QString>
qstringlistAsQListQString(const QStringList& list);
::rust::isize
qstringlistRemoveDuplicates(QStringList& list);

}
}
