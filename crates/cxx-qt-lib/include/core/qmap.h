// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QMap>

#include <QtCore/QString>
#include <QtCore/QVariant>

#include "rust/cxx.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

// This has static asserts in the cpp file to ensure this is valid.
template<typename K, typename V>
struct IsRelocatable<QMap<K, V>> : ::std::true_type
{
};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {
namespace qmap {

template<typename K, typename V>
::rust::isize
qmapLen(const QMap<K, V>& m) noexcept;

template<typename K, typename V>
V
qmapGetOrDefault(const QMap<K, V>& m, const K& key) noexcept
{
  // Qt 6 returns a T and Qt 5 returns an const T
  // so we need to define our own method here for CXX
  return m.value(key);
}

template<typename K, typename V>
const K&
qmapGetUncheckedKey(const QMap<K, V>& m, ::rust::isize pos) noexcept
{
  Q_ASSERT(pos < qmapLen(m));
  Q_ASSERT(pos >= 0);
  auto it = m.cbegin();
  ::std::advance(it, pos);
  return it.key();
}

template<typename K, typename V>
const V&
qmapGetUncheckedValue(const QMap<K, V>& m, ::rust::isize pos) noexcept
{
  Q_ASSERT(pos < qmapLen(m));
  Q_ASSERT(pos >= 0);
  auto it = m.cbegin();
  ::std::advance(it, pos);
  return it.value();
}

template<typename K, typename V>
void
qmapInsert(QMap<K, V>& m, const K& key, const V& value) noexcept
{
  m.insert(key, value);
}

template<typename K, typename V>
::rust::isize
qmapLen(const QMap<K, V>& m) noexcept
{
  // Qt has an int as the QMap::size_type
  return static_cast<::rust::isize>(m.size());
}

template<typename K, typename V>
bool
qmapRemove(QMap<K, V>& m, const K& key) noexcept
{
  return m.remove(key) >= 1;
}

}
}
}

using QMap_QString_QVariant = QMap<QString, QVariant>;
