// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <cxx-qt/locking.h>
#include <memory>
#include <mutex>
#include <type_traits>

namespace rust::cxxqt1 {

// An empty implementation of MaybeLockGuard
//
// This means for types that do not implement CxxQtLocking we do nothing
template<typename T, typename Derived = void>
struct MaybeLockGuard
{
  MaybeLockGuard(const T&) {}
};

// Create a lock guard for types that implement CxxQtLocking
template<typename T>
struct MaybeLockGuard<T,
                      ::std::enable_if_t<::std::is_base_of_v<CxxQtLocking, T>>>
{
  MaybeLockGuard(const CxxQtLocking& locking)
    : m_lock(locking.unsafeRustLock())
  {
  }

private:
  ::std::lock_guard<::std::recursive_mutex> m_lock;
};

}
