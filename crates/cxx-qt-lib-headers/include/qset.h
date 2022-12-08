// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QSet>

#include <QtCore/QDate>
#include <QtCore/QDateTime>
#include <QtCore/QString>
#include <QtCore/QTime>
#include <QtCore/QUrl>

#include "rust/cxx.h"

// This has static asserts in the cpp file to ensure this is valid.
template<typename T>
struct rust::IsRelocatable<QSet<T>> : std::true_type
{
};

namespace rust {
namespace cxxqtlib1 {

template<typename T>
const T&
qsetGetUnchecked(const QSet<T>& s, ::std::size_t pos) noexcept
{
  Q_ASSERT(pos < static_cast<::std::size_t>(s.size()));
  auto it = s.cbegin();
  std::advance(it, pos);
  return *it;
}

template<typename T>
void
qsetInsert(QSet<T>& s, const T& value) noexcept
{
  s.insert(value);
}

template<typename T>
::std::size_t
qsetLen(const QSet<T>& s) noexcept
{
  // In Qt 5 the type was int now it is qsizetype, so we need to ensure the type
  // is the same for CXX
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  return s.size();
#else
  return static_cast<::std ::size_t>(s.size());
#endif
}

}
}

using QSet_bool = QSet<bool>;
using QSet_f32 = QSet<float>;
using QSet_f64 = QSet<double>;
using QSet_i8 = QSet<::qint8>;
using QSet_i16 = QSet<::qint16>;
using QSet_i32 = QSet<::qint32>;
using QSet_QDate = QSet<::QDate>;
using QSet_QDateTime = QSet<::QDateTime>;
using QSet_QString = QSet<::QString>;
using QSet_QTime = QSet<::QTime>;
using QSet_QUrl = QSet<::QUrl>;
using QSet_u8 = QSet<::quint8>;
using QSet_u16 = QSet<::quint16>;
using QSet_u32 = QSet<::quint32>;
