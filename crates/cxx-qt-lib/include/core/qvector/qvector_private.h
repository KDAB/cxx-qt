// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//!This is an auto-generated file. Do not edit. Edit instead: generate.sh, in: cxx-qt-lib/src/core/qvector

#pragma once

#include <cstdint>

#include <QtCore/QVector>

#include "rust/cxx.h"

// In Qt 6 QList and QVector are the same, so we only need IsRelocatable defined
// once
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
#include "qlist_qvector.h"
#else
// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

// This has static asserts in the cpp file to ensure this is valid.
template<typename T>
struct IsRelocatable<QVector<T>> : ::std::true_type
{};

} // namespace rust
#endif

namespace rust {
namespace cxxqtlib1 {
namespace qvector {

template<typename T>
::rust::isize
qvectorLen(const QVector<T>& v) noexcept;

template<typename T>
void
qvectorAppend(QVector<T>& v, const T& value) noexcept
{
  // Qt 5 has const T& Qt 6 has QList<T>::rvalue_ref or QList<T>::parameter_type
  v.append(value);
}

template<typename T>
const T&
qvectorGetUnchecked(const QVector<T>& v, ::rust::isize pos) noexcept
{
  Q_ASSERT(pos < qvectorLen(v));
  Q_ASSERT(pos >= 0);
  // Qt has an int Qt 6 has a qsizetype
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  return v.at(static_cast<qsizetype>(pos));
#else
  return v.at(static_cast<int>(pos));
#endif
}

template<typename T>
::rust::isize
qvectorIndexOf(const QVector<T>& v, const T& value) noexcept
{
  // Qt 5 has an int Qt 6 has a qsizetype
  return static_cast<::rust::isize>(v.indexOf(value));
}

template<typename T>
void
qvectorInsert(QVector<T>& v, ::rust::isize pos, const T& value) noexcept
{
  Q_ASSERT(pos >= 0);
  // Qt 5 has an int Qt 6 has a qsizetype
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  v.insert(static_cast<qsizetype>(pos), value);
#else
  v.insert(static_cast<int>(pos), value);
#endif
}

template<typename T>
::rust::isize
qvectorLen(const QVector<T>& v) noexcept
{
  // In Qt 5 the type was int now it is qsizetype, so we need to ensure the type
  // is the same for CXX
  return static_cast<::rust::isize>(v.size());
}

template<typename T>
void
qvectorRemove(QVector<T>& v, ::rust::isize pos) noexcept
{
  Q_ASSERT(pos >= 0);
  // Qt 5 has an int Qt 6 has a qsizetype
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  v.removeAt(static_cast<qsizetype>(pos));
#else
  v.removeAt(static_cast<int>(pos));
#endif
}

template<typename T>
void
qvectorReserve(QVector<T>& v, ::rust::isize size) noexcept
{
  Q_ASSERT(size >= 0);
  // Qt has an int Qt 6 has a qsizetype
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  v.reserve(static_cast<qsizetype>(size));
#else
  v.reserve(static_cast<int>(size));
#endif
}

}
}
}

