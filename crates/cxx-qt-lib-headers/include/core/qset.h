// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cstdint>

#include <QtCore/QSet>

#include <QtCore/QByteArray>
#include <QtCore/QDate>
#include <QtCore/QDateTime>
#include <QtCore/QPersistentModelIndex>
#include <QtCore/QString>
#include <QtCore/QTime>
#include <QtCore/QUrl>

#include "rust/cxx.h"

// This has static asserts in the cpp file to ensure this is valid.
template<typename T>
struct rust::IsRelocatable<QSet<T>> : ::std::true_type
{
};

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

}
}
}

using QSet_bool = QSet<bool>;
using QSet_f32 = QSet<float>;
using QSet_f64 = QSet<double>;
using QSet_i8 = QSet<::std::int8_t>;
using QSet_i16 = QSet<::std::int16_t>;
using QSet_i32 = QSet<::std::int32_t>;
using QSet_i64 = QSet<::std::int64_t>;
using QSet_QByteArray = QSet<::QByteArray>;
using QSet_QDate = QSet<::QDate>;
using QSet_QDateTime = QSet<::QDateTime>;
using QSet_QPersistentModelIndex = QSet<::QPersistentModelIndex>;
using QSet_QString = QSet<::QString>;
using QSet_QTime = QSet<::QTime>;
using QSet_QUrl = QSet<::QUrl>;
using QSet_u8 = QSet<::std::uint8_t>;
using QSet_u16 = QSet<::std::uint16_t>;
using QSet_u32 = QSet<::std::uint32_t>;
using QSet_u64 = QSet<::std::uint64_t>;
