// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <memory>
#include <mutex>

#include <cxx-qt/include/thread.h>

namespace rust::cxxqt1 {

template<typename T>
class CxxQtThreading
{
public:
  explicit CxxQtThreading(T* obj)
    : m_cxxQtThreadObj(::std::make_shared<CxxQtGuardedPointer<T>>(obj))
  {
  }

  virtual ~CxxQtThreading()
  {
    const auto guard = ::std::unique_lock(m_cxxQtThreadObj->mutex);
    m_cxxQtThreadObj->ptr = nullptr;
  }

  CxxQtThread<T> qtThread() const { return CxxQtThread<T>(m_cxxQtThreadObj); }

private:
  ::std::shared_ptr<CxxQtGuardedPointer<T>> m_cxxQtThreadObj;
};

template<typename T>
CxxQtThread<T>
qtThread(const T& qobject)
{
  return static_cast<const CxxQtThreading<T>&>(qobject).qtThread();
}

}
