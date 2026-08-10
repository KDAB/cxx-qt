// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QJsonArray>
#include <QtCore/QJsonObject>
#include <QtCore/QJsonValue>

#include <cstdint>

#include "rust/cxx.h"

namespace rust {

template<>
struct IsRelocatable<QJsonValue> : ::std::true_type
{};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {

QJsonValue
qjsonvalueFromI64(::rust::i64 value);
bool
qjsonvalueToBool(const QJsonValue& value);
::rust::f64
qjsonvalueToDouble(const QJsonValue& value);
::rust::i32
qjsonvalueToInt(const QJsonValue& value);
QString
qjsonvalueToQString(const QJsonValue& value);
QJsonArray
qjsonvalueToArray(const QJsonValue& value);
QJsonObject
qjsonvalueToObject(const QJsonValue& value);

} // namespace cxxqtlib1
} // namespace rust
