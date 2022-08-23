// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QObject>
#include <QtCore/QPointer>

namespace rust {
namespace cxxqtlib1 {

class UpdateRequester
{
public:
  UpdateRequester(QPointer<QObject> obj, const char* method);
  bool requestUpdate() const;

private:
  const char* m_method;
  QPointer<QObject> m_obj;
};

} // namespace cxxqtlib1
} // namespace rust
