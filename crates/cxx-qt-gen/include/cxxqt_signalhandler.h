// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <type_traits>

#include "rust/cxx.h"

namespace rust::cxxqtlib1 {

template<typename CXXArguments>
class SignalHandler
{

public:
  SignalHandler() = delete;
  SignalHandler(const SignalHandler&) = delete;

  SignalHandler(SignalHandler&& other)
  {
    data[0] = other.data[0];
    data[1] = other.data[1];
    other.data[0] = nullptr;
    other.data[1] = nullptr;
  }

  ~SignalHandler() noexcept;
  template<typename... Arguments>
  void operator()(Arguments... args);

private:
  void* data[2];
};

} // rust::cxxqtlib1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<typename CXXArguments>
struct IsRelocatable<rust::cxxqtlib1::SignalHandler<CXXArguments>>
  : ::std::true_type
{
};

} // namespace rust
