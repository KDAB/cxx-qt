// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <memory>
#include <mutex>

#include <cxx-qt/thread.h>

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

// Note: Use auto syntax here, because otherwise it is not possible to fully
// qualify this function when it is declared as as friend.
//
// e.g.:
// ```
// friend rust::cxxqt1::CxxQtThread<T> ::rust::cxxqt1::qtThread<T>(const MyType&
// qobject);
// ```
// is parsed as:
// ```
// friend rust::cxxqt1::CxxQtThread<T>::rust::cxxqt1::qtThread<T>(const MyType&
// qobject);
// ```
// Because the `::` after `CxxQtThread<T>` is scope resolution operator it
// applies to the type `CxxQtThread<T>` rather than starting a new scope
// resolution from the global namespace.
template<typename T>
auto
qtThread(const T& qobject) -> CxxQtThread<T>
{
  return static_cast<const CxxQtThreading<T>&>(qobject).qtThread();
}

}
