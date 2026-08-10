// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QJsonArray>
#include <QtCore/QJsonValue>

#include "rust/cxx.h"

namespace rust {

template<>
struct IsRelocatable<QJsonArray> : ::std::true_type
{};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {

::rust::isize
qjsonarrayLen(const QJsonArray& array);

QJsonValue
qjsonarrayAt(const QJsonArray& array, ::rust::isize i);

} // namespace cxxqtlib1
} // namespace rust
