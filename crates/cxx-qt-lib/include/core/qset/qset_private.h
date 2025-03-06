// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cstdint>

#include <QtCore/QSet>

#include "rust/cxx.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

// This has static asserts in the cpp file to ensure this is valid.
template<typename T>
struct IsRelocatable<QSet<T>> : ::std::true_type
{};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {
namespace qset {

template<typename T>
::rust::isize
qsetLen(const QSet<T>& s) noexcept;

template<typename T>
const T&
qsetGetUnchecked(const QSet<T>& s, ::rust::isize pos) noexcept
{
  Q_ASSERT(pos < qsetLen(s));
  Q_ASSERT(pos >= 0);
  auto it = s.cbegin();
  ::std::advance(it, pos);
  return *it;
}

template<typename T>
void
qsetInsert(QSet<T>& s, const T& value) noexcept
{
  s.insert(value);
}

template<typename T>
::rust::isize
qsetLen(const QSet<T>& s) noexcept
{
  // In Qt 5 the type was int now it is qsizetype, so we need to ensure the type
  // is the same for CXX
  return static_cast<::rust::isize>(s.size());
}

template<typename T>
void
qsetReserve(QSet<T>& s, ::rust::isize size) noexcept
{
  Q_ASSERT(size >= 0);
  // Qt 5 has an int Qt 6 has a qsizetype
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  s.reserve(static_cast<qsizetype>(size));
#else
  s.reserve(static_cast<int>(size));
#endif
}

}
}
}