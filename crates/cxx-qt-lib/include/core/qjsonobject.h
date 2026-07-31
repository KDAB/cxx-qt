// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QJsonObject>
#include <QtCore/QJsonValue>
#include <QtCore/QString>

#include "rust/cxx.h"

namespace rust {

template<>
struct IsRelocatable<QJsonObject> : ::std::true_type
{};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {

::rust::isize
qjsonobjectLen(const QJsonObject& object);

void
qjsonobjectInsert(QJsonObject& object,
                  const QString& key,
                  const QJsonValue& value);

} // namespace cxxqtlib1
} // namespace rust
