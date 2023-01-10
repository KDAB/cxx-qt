// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cstdint>

#include <QtCore/QList>

#include <QtCore/QDate>
#include <QtCore/QDateTime>
#include <QtCore/QPoint>
#include <QtCore/QPointF>
#include <QtCore/QRect>
#include <QtCore/QRectF>
#include <QtCore/QSize>
#include <QtCore/QSizeF>
#include <QtCore/QString>
#include <QtCore/QTime>
#include <QtCore/QUrl>
#include <QtCore/QVariant>

#include <QtGui/QColor>

#include "rust/cxx.h"

// In Qt 6 QList and QVector are the same, so we only need IsRelocatable defined
// once
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
#include "qlist_qvector.h"
#else
// This has static asserts in the cpp file to ensure this is valid.
template<typename T>
struct rust::IsRelocatable<QList<T>> : ::std::true_type
{
};
#endif

namespace rust {
namespace cxxqtlib1 {
namespace qlist {

template<typename T>
::rust::isize
qlistLen(const QList<T>& v) noexcept;

template<typename T>
void
qlistAppend(QList<T>& v, const T& value) noexcept
{
  // Qt 5 has const T& Qt 6 has QList<T>::rvalue_ref or QList<T>::parameter_type
  v.append(value);
}

template<typename T>
const T&
qlistGetUnchecked(const QList<T>& v, ::rust::isize pos) noexcept
{
  Q_ASSERT(pos < qlistLen(v));
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
qlistIndexOf(const QList<T>& v, const T& value) noexcept
{
  // Qt 5 has an int Qt 6 has a qsizetype
  return static_cast<::rust::isize>(v.indexOf(value));
}

template<typename T>
void
qlistInsert(QList<T>& v, ::rust::isize pos, const T& value) noexcept
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
qlistLen(const QList<T>& v) noexcept
{
  // In Qt 5 the type was int now it is qsizetype, so we need to ensure the type
  // is the same for CXX
  return static_cast<::rust::isize>(v.size());
}

template<typename T>
void
qlistRemove(QList<T>& v, ::rust::isize pos) noexcept
{
  Q_ASSERT(pos >= 0);
  // Qt 5 has an int Qt 6 has a qsizetype
  // Qt 5 only has removeAt Qt 6 has remove or removeAt
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  v.remove(static_cast<qsizetype>(pos));
#else
  v.removeAt(static_cast<int>(pos));
#endif
}

}
}
}

using QList_bool = QList<bool>;
using QList_f32 = QList<float>;
using QList_f64 = QList<double>;
using QList_i8 = QList<::std::int8_t>;
using QList_i16 = QList<::std::int16_t>;
using QList_i32 = QList<::std::int32_t>;
using QList_i64 = QList<::std::int64_t>;
using QList_QColor = QList<::QColor>;
using QList_QDate = QList<::QDate>;
using QList_QDateTime = QList<::QDateTime>;
using QList_QPoint = QList<::QPoint>;
using QList_QPointF = QList<::QPointF>;
using QList_QRect = QList<::QRect>;
using QList_QRectF = QList<::QRectF>;
using QList_QSize = QList<::QSize>;
using QList_QSizeF = QList<::QSizeF>;
using QList_QString = QList<::QString>;
using QList_QTime = QList<::QTime>;
using QList_QUrl = QList<::QUrl>;
using QList_QVariant = QList<::QVariant>;
using QList_u8 = QList<::std::uint8_t>;
using QList_u16 = QList<::std::uint16_t>;
using QList_u32 = QList<::std::uint32_t>;
using QList_u64 = QList<::std::uint64_t>;
