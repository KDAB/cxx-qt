// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QString>

#include "rust/cxx.h"

template<>
struct rust::IsRelocatable<QString> : std::true_type
{
};

namespace rust {
namespace cxxqtlib1 {

QString
qstringInitFromRustString(rust::Str string);
rust::String
qstringToRustString(const QString& string);

}
}
