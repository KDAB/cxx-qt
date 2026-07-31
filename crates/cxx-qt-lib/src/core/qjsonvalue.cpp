// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qjsonvalue.h"

#include <cxx-qt-lib/assertion_utils.h>

#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
assert_alignment_and_size(QJsonValue, {
  // QCborValue value
  // {
  //   qint64 n;
  //   QCborContainerPrivate *container;
  //   Type t;
  // };
  ::std::uint64_t n;
  ::std::size_t container;
  ::std::size_t t;
});
#else
assert_alignment_and_size(QJsonValue, {
  // qint64 n;
  ::std::uint64_t n;

  // QExplicitlySharedDataPointer<QCborContainerPrivate> d;
  ::std::size_t d;

  // QCborValue::Type t;
  ::std::size_t t;
});
#endif

static_assert(!::std::is_trivially_copy_assignable<QJsonValue>::value);
static_assert(!::std::is_trivially_copy_constructible<QJsonValue>::value);

static_assert(!::std::is_trivially_destructible<QJsonValue>::value);

static_assert(QTypeInfo<QJsonValue>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

QJsonValue
qjsonvalueFromI64(::rust::i64 value)
{
  return QJsonValue(static_cast<qint64>(value));
}

bool
qjsonvalueToBool(const QJsonValue& value)
{
  return value.toBool(false);
}

::rust::f64
qjsonvalueToDouble(const QJsonValue& value)
{
  return value.toDouble(0.0);
}

::rust::i32
qjsonvalueToInt(const QJsonValue& value)
{
  return static_cast<::rust::i32>(value.toInt(0));
}

QString
qjsonvalueToQString(const QJsonValue& value)
{
  return value.toString(QString{});
}

QJsonArray
qjsonvalueToArray(const QJsonValue& value)
{
  return value.toArray(QJsonArray{});
}

QJsonObject
qjsonvalueToObject(const QJsonValue& value)
{
  return value.toObject(QJsonObject{});
}

} // namespace cxxqtlib1
} // namespace rust
