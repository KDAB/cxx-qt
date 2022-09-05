// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <memory>

namespace rust {
namespace cxxqtlib1 {

template<typename R, typename T>
struct cxx_qt_convert
{
  static_assert(std::is_convertible_v<T, R>, R"(

CXXQt: No viable conversion between types found.
Consider defining your own type conversion.
See: https://kdab.github.io/cxx-qt/book/concepts/type-conversions.html
)");
  R operator()(T val) { return val; }
};

template<typename R, typename T>
struct cxx_qt_convert<R&, T>
{
  R& operator()(T& val) { return val; }
};

template<typename R, typename T>
struct cxx_qt_convert<const R&, T>
{
  const R& operator()(const T& val) { return val; }
};

template<typename R, typename T>
struct cxx_qt_convert<R, std::unique_ptr<T>>
{
  R operator()(std::unique_ptr<T> ptr) { return std::move(*ptr); }
};

template<typename R, typename T>
struct cxx_qt_convert<R&, std::unique_ptr<T>>
{
  R& operator()(std::unique_ptr<T>& ptr) { return *ptr; }
};

template<typename R, typename T>
struct cxx_qt_convert<const R&, std::unique_ptr<T>>
{
  const R& operator()(const std::unique_ptr<T>& ptr) { return *ptr; }
};

template<typename R, typename T>
struct cxx_qt_convert<const R&, const std::unique_ptr<T>&>
{
  const R& operator()(const std::unique_ptr<T>& ptr) { return *ptr; }
};

template<typename R, typename T>
struct cxx_qt_convert<std::unique_ptr<R>, const T&>
{
  std::unique_ptr<R> operator()(const T& value)
  {
    return std::make_unique<R>(value);
  }
};

} // namespace cxxqtlib1
} // namespace rust
