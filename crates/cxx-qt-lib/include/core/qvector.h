// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cstdint>

#include <QtCore/QVector>

#include <QtCore/QByteArray>
#include <QtCore/QDate>
#include <QtCore/QDateTime>
#include <QtCore/QMargins>
#include <QtCore/QMarginsF>
#include <QtCore/QPersistentModelIndex>
#include <QtCore/QPoint>
#include <QtCore/QPointF>
#include <QtCore/QRect>
#include <QtCore/QRectF>
#include <QtCore/QSize>
#include <QtCore/QSizeF>
#include <QtCore/QString>
#include <QtCore/QTime>
#include <QtCore/QUrl>
#include <QtCore/QUuid>
#include <QtCore/QVariant>

#ifdef CXX_QT_GUI_FEATURE
#include <QtGui/QColor>
#endif

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

using QVector_bool = QVector<bool>;
using QVector_f32 = QVector<float>;
using QVector_f64 = QVector<double>;
using QVector_i8 = QVector<::std::int8_t>;
using QVector_i16 = QVector<::std::int16_t>;
using QVector_i32 = QVector<::std::int32_t>;
using QVector_i64 = QVector<::std::int64_t>;
using QVector_QByteArray = QVector<::QByteArray>;
#ifdef CXX_QT_GUI_FEATURE
using QVector_QColor = QVector<::QColor>;
#endif
using QVector_QDate = QVector<::QDate>;
using QVector_QDateTime = QVector<::QDateTime>;
using QVector_QLine = QVector<::QLine>;
using QVector_QLineF = QVector<::QLineF>;
using QVector_QMargins = QVector<::QMargins>;
using QVector_QMarginsF = QVector<::QMarginsF>;
using QVector_QPersistentModelIndex = QVector<::QPersistentModelIndex>;
using QVector_QPoint = QVector<::QPoint>;
using QVector_QPointF = QVector<::QPointF>;
using QVector_QRect = QVector<::QRect>;
using QVector_QRectF = QVector<::QRectF>;
using QVector_QSize = QVector<::QSize>;
using QVector_QSizeF = QVector<::QSizeF>;
using QVector_QString = QVector<::QString>;
using QVector_QTime = QVector<::QTime>;
using QVector_QUrl = QVector<::QUrl>;
using QVector_QUuid = QVector<::QUuid>;
using QVector_QVariant = QVector<::QVariant>;
using QVector_u8 = QVector<::std::uint8_t>;
using QVector_u16 = QVector<::std::uint16_t>;
using QVector_u32 = QVector<::std::uint32_t>;
using QVector_u64 = QVector<::std::uint64_t>;
