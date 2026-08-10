// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qjsonobject.h"

#include <cxx-qt-lib/assertion_utils.h>

#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
assert_alignment_and_size(QJsonObject, {
  // QExplicitlySharedDataPointer<QCborContainerPrivate> o;
  ::std::size_t o;
});
#else
assert_alignment_and_size(QJsonObject, {
  // void *dead = nullptr;
  ::std::size_t d;

  // QExplicitlySharedDataPointer<QCborContainerPrivate> o;
  ::std::size_t o;
});
#endif

static_assert(!::std::is_trivially_copy_assignable<QJsonObject>::value);
static_assert(!::std::is_trivially_copy_constructible<QJsonObject>::value);

static_assert(!::std::is_trivially_destructible<QJsonObject>::value);

static_assert(QTypeInfo<QJsonObject>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

::rust::isize
qjsonobjectLen(const QJsonObject& object)
{
  return static_cast<::rust::isize>(object.size());
}

void
qjsonobjectInsert(QJsonObject& object,
                  const QString& key,
                  const QJsonValue& value)
{
  object.insert(key, value);
}

} // namespace cxxqtlib1
} // namespace rust
