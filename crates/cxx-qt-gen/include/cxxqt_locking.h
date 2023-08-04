// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <memory>
#include <mutex>

namespace rust::cxxqtlib1 {

class CxxQtLocking
{
public:
  explicit CxxQtLocking()
    : m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
  {
  }

  virtual ~CxxQtLocking() = default;

protected:
  [[nodiscard]] ::std::lock_guard<::std::recursive_mutex> unsafeRustLock() const
  {
    return ::std::lock_guard<::std::recursive_mutex>(*m_rustObjMutex);
  }

  ::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;

  // Friend MaybeLockGuard so that it can use unsafeRustLock()
  template<typename T, typename D>
  friend class MaybeLockGuard;
};

}
