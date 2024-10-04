// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <memory>
#include <mutex>
#include <shared_mutex>
#include <stdexcept>

#include <QtCore/QDebug>
#include <QtCore/QMetaObject>

#include "rust/cxx.h"

namespace rust {
namespace cxxqt1 {

template<typename T>
class CxxQtGuardedPointer final
{
public:
  explicit CxxQtGuardedPointer(T* ptr)
    : ptr(ptr)
  {
  }

  T* ptr;
  ::std::shared_mutex mutex;
};

template<typename T>
class CxxQtThread final
{
public:
  CxxQtThread(::std::shared_ptr<CxxQtGuardedPointer<T>> obj)
    : m_obj(obj)
  {
  }

  ~CxxQtThread() = default;
  CxxQtThread(const CxxQtThread<T>& other) = default;
  CxxQtThread(CxxQtThread<T>&& other) = default;

  bool isDestroyed() const
  {
    const auto guard = ::std::shared_lock(m_obj->mutex);
    return m_obj->ptr == nullptr;
  }

  template<typename A>
  void queue(::rust::Fn<void(T& self, ::rust::Box<A> arg)> func,
             ::rust::Box<A> arg) const
  {
    // Ensure that we can read the pointer and it's not being written to
    const auto guard = ::std::shared_lock(m_obj->mutex);
    if (!m_obj->ptr) {
      throw ::std::runtime_error(
        "Cannot queue function pointer as object has been destroyed");
      return;
    }

    // Construct the lambda
    auto obj = m_obj;
    auto lambda = [obj = ::std::move(obj),
                   func = ::std::move(func),
                   arg = ::std::move(arg)]() mutable {
      // Ensure that we can read the pointer and it's not being written to
      const auto guard = ::std::shared_lock(obj->mutex);
      if (obj->ptr) {
        func(*obj->ptr, ::std::move(arg));
      } else {
        qWarning()
          << "Could not call the function pointer as object has been destroyed";
      }
    };

    // Add the lambda to the queue
    if (!QMetaObject::invokeMethod(
          m_obj->ptr, ::std::move(lambda), Qt::QueuedConnection)) {
      throw ::std::runtime_error(
        "Cannot queue function pointer as invokeMethod on object failed");
    }
  }

private:
  ::std::shared_ptr<CxxQtGuardedPointer<T>> m_obj;
};

template<typename T>
CxxQtThread<T>
cxxQtThreadClone(const CxxQtThread<T>& cxxQtThread)
{
  return CxxQtThread<T>(cxxQtThread);
}

template<typename T>
void
cxxQtThreadDrop(CxxQtThread<T>& cxxQtThread)
{
  cxxQtThread.~CxxQtThread<T>();
}

template<typename A, typename T>
void
cxxQtThreadQueue(const CxxQtThread<T>& cxxQtThread,
                 ::rust::Fn<void(T& self, ::rust::Box<A> arg)> func,
                 ::rust::Box<A> arg)
{
  cxxQtThread.queue(::std::move(func), ::std::move(arg));
}

template<typename T>
bool
cxxQtThreadIsDestroyed(const CxxQtThread<T>& cxxQtThread)
{
  return cxxQtThread.isDestroyed();
}

} // namespace cxxqt1
} // namespace rust

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<typename T>
struct IsRelocatable<::rust::cxxqt1::CxxQtThread<T>> : ::std::true_type
{};

} // namespace rust
