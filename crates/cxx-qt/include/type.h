// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <memory>
#include <mutex>

#include "rust/cxx.h"

namespace rust::cxxqt1 {

template<typename T>
class CxxQtType
{
public:
  explicit CxxQtType(::rust::Box<T>&& rustObj)
    : m_rustObj(::std::move(rustObj))
  {
  }

  virtual ~CxxQtType() = default;

  T const& unsafeRust() const { return *m_rustObj; }
  T& unsafeRustMut() { return *m_rustObj; }

protected:
  ::rust::Box<T> m_rustObj;
};

template<typename Inner, typename Outer>
Inner&
unsafeRustMut(Outer& outer)
{
  return static_cast<CxxQtType<Inner>&>(outer).unsafeRustMut();
}

template<typename Inner, typename Outer>
const Inner&
unsafeRust(const Outer& outer)
{
  return static_cast<const CxxQtType<Inner>&>(outer).unsafeRust();
}

}
