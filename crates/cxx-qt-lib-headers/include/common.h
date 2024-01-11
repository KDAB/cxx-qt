// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <QtCore/QDebug>
#include <QtCore/QString>
#include <cinttypes>

namespace rust {
namespace cxxqtlib1 {

using c_void = void;

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
  // qDebug() << value
  //
  // We can then construct a QDebug object that outputs into a string (instead
  // of logging), and return that string. Thus we have a pretty reliable and
  // generic "toString" implementation for most Qt types.
  QString res;
  QDebug serializer{ &res };
  serializer << value;
  return res;
}

template<typename T>
bool
operatorEq(const T& a, const T& b)
{
  return a == b;
}

template<typename T>
::std::int8_t
operatorCmp(const T& a, const T& b)
{
  return operatorEq(a, b) ? 0 : (a < b ? -1 : 1);
}

template<typename A, typename B>
A
operatorPlus(const A& a, const B& b)
{
  return a + b;
}

template<typename A, typename B>
A
operatorMinus(const A& a, const B& b)
{
  return a - b;
}

template<typename S, typename T>
T
operatorMul(const S scalar, const T& t)
{
  return scalar * t;
}

template<typename S, typename T>
T
operatorDiv(const S scalar, const T& t)
{
  return t / scalar;
}

template <typename T, typename... Args>
std::unique_ptr<T> make_unique(Args... args) {
  return std::make_unique<T>(args...);
}

} // namespace cxxqtlib1
} // namespace rust
