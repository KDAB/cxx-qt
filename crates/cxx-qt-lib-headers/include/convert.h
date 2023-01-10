// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <memory>

namespace {

template<typename R, typename T>
struct CxxQtConvertCheck
{
  static_assert(::std::is_convertible_v<T, R>, R"(
    CXX-Qt: No viable conversion between types found.
    Consider defining your own type conversion.
    See: https://kdab.github.io/cxx-qt/book/concepts/type-conversions.html
    )");
};

}

namespace rust {
namespace cxxqtlib1 {

// T -> R
template<typename R, typename T>
struct cxx_qt_convert
{
  constexpr static CxxQtConvertCheck<R, T> check{};
  R operator()(T val) { return val; }
};

// ::std::unique_ptr<T> -> R
template<typename R, typename T>
struct cxx_qt_convert<R, ::std::unique_ptr<T>>
{
  constexpr static CxxQtConvertCheck<R, T> check{};
  R operator()(::std::unique_ptr<T> ptr) { return ::std::move(*ptr); }
};

// const ::std::unique_ptr<T>& -> const R&
template<typename R, typename T>
struct cxx_qt_convert<const R&, const ::std::unique_ptr<T>&>
{
  constexpr static CxxQtConvertCheck<const R&, T> check{};
  const R& operator()(const ::std::unique_ptr<T>& ptr) { return *ptr; }
};

// const T& -> ::std::unique_ptr<T>
template<typename R, typename T>
struct cxx_qt_convert<::std::unique_ptr<R>, const T&>
{
  static_assert(::std::is_constructible_v<R, const T&>, R"(
    CXX-Qt: Cannot construct type. Consider defining your own constructor.
    )");

  ::std::unique_ptr<R> operator()(const T& value)
  {
    return ::std::make_unique<R>(value);
  }
};

} // namespace cxxqtlib1
} // namespace rust
