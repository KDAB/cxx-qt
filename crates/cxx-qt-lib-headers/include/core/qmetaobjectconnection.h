// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <memory>

#include <QtCore/QObject>

namespace rust {
namespace cxxqtlib1 {

class QMetaObjectConnectionGuard
{
public:
  explicit QMetaObjectConnectionGuard(::QMetaObject::Connection inner);
  ~QMetaObjectConnectionGuard();
  void disconnect() const;
  void release();

private:
  ::std::unique_ptr<::QMetaObject::Connection> m_inner;
};

}
}
