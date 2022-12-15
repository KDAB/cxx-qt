// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cstdint>

#include <QtCore/QHash>

#include <QtCore/QByteArray>
#include <QtCore/QString>
#include <QtCore/QVariant>

#include "rust/cxx.h"

// This has static asserts in the cpp file to ensure this is valid.
template<typename K, typename V>
struct rust::IsRelocatable<QHash<K, V>> : std::true_type
{
};

namespace rust {
namespace cxxqtlib1 {
namespace qhash {

template<typename K, typename V>
::rust::isize
qhashLen(const QHash<K, V>& h) noexcept;

template<typename K, typename V>
const K&
qhashGetUncheckedKey(const QHash<K, V>& h, ::rust::isize pos) noexcept
{
  Q_ASSERT(pos < qhashLen(h));
  Q_ASSERT(pos >= 0);
  auto it = h.cbegin();
  std::advance(it, pos);
  return it.key();
}

template<typename K, typename V>
const V&
qhashGetUncheckedValue(const QHash<K, V>& h, ::rust::isize pos) noexcept
{
  Q_ASSERT(pos < qhashLen(h));
  Q_ASSERT(pos >= 0);
  auto it = h.cbegin();
  std::advance(it, pos);
  return it.value();
}

template<typename K, typename V>
void
qhashInsert(QHash<K, V>& h, const K& key, const V& value) noexcept
{
  h.insert(key, value);
}

template<typename K, typename V>
::rust::isize
qhashLen(const QHash<K, V>& h) noexcept
{
  // Qt 6 returns a qsizetype and Qt 5 returns an int
  return static_cast<::rust::isize>(h.size());
}

template<typename K, typename V>
bool
qhashRemove(QHash<K, V>& h, const K& key) noexcept
{
  // Qt 6 returns a bool and Qt 5 returns an int
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  return h.remove(key);
#else
  return h.remove(key) >= 1;
#endif
}

template<typename K, typename V>
V
qhashValue(const QHash<K, V>& h, const K& key) noexcept
{
  // Qt 6 returns a T and Qt 5 returns an const T
  // so we need to define our own method here for CXX
  return h.value(key);
}

}
}
}

using QHash_i32_QByteArray = QHash<::std::int32_t, QByteArray>;
using QHash_QString_QVariant = QHash<QString, QVariant>;
