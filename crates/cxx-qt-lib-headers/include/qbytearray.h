// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QByteArray>

#include "rust/cxx.h"

template<>
struct rust::IsRelocatable<QByteArray> : std::true_type
{
};

namespace rust {
namespace cxxqtlib1 {

QByteArray
qbytearrayFromRustString(rust::Str string);
rust::String
qbytearrayToRustString(const QByteArray& string);

}
}
