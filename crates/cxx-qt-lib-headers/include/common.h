// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <QtCore/QDebug>
#include <QtCore/QString>

namespace rust {
namespace cxxqtlib1 {

template<typename T, typename... Args>
T
construct(Args... args)
{
  return T(args...);
}

template<typename T>
void
drop(T& value)
{
  value.~T();
}

template<typename T>
QString
toQString(const T& value)
{
  // We can't convert value directly into a string.
  // However most Qt types are able to stream into a QDebug object such as
  // qDebug() << value We can then construct a QDebug object that outputs into a
  // string (instead of logging), and return that string Thus we have a pretty
  // reliable and generic "toString" implementation for most Qt types
  QString res;
  QDebug serializer{ &res };
  serializer << value;
  return res;
}

} // namespace cxxqtlib1
} // namespace rust
