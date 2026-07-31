// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qjsonarray.h"

#include <cxx-qt-lib/assertion_utils.h>

#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
assert_alignment_and_size(QJsonArray, {
  // QExplicitlySharedDataPointer<QCborContainerPrivate> a;
  // QJsonValue[Const]Ref has bitfields put into size_t.
  ::std::size_t a;
});
#else
assert_alignment_and_size(QJsonArray, {
  // void *dead
  ::std::size_t d;

  // QExplicitlySharedDataPointer<QCborContainerPrivate> a;
  ::std::size_t a;
});
#endif

static_assert(!::std::is_trivially_copy_assignable<QJsonArray>::value);
static_assert(!::std::is_trivially_copy_constructible<QJsonArray>::value);

static_assert(!::std::is_trivially_destructible<QJsonArray>::value);

static_assert(QTypeInfo<QJsonArray>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

::rust::isize
qjsonarrayLen(const QJsonArray& array)
{
  return static_cast<::rust::isize>(array.size());
}

QJsonValue
qjsonarrayAt(const QJsonArray& array, ::rust::isize i)
{
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  return array.at(static_cast<qsizetype>(i));
#else
  return array.at(static_cast<int>(i));
#endif
}

} // namespace cxxqtlib1
} // namespace rust
