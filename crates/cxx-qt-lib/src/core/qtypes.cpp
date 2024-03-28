// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qtypes.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(qsizetype, { ::std::size_t a0; });
assert_alignment_and_size(qint64, { ::std::int64_t a0; });

// static_assert(!::std::is_trivially_copy_assignable<qsizetype>::value);
// static_assert(!::std::is_trivially_copy_constructible<qsizetype>::value);

// static_assert(!::std::is_trivially_destructible<qsizetype>::value);

// static_assert(QTypeInfo<qsizetype>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

::qint64
qint64FromI64(::std::int64_t value)
{
  return static_cast<::qint64>(value);
}

::std::int64_t
qint64IntoI64(::qint64 value)
{
  return static_cast<::std::int64_t>(value);
}

::qsizetype
qsizetypeFromIsize(::rust::isize value)
{
  return static_cast<::qsizetype>(value);
}

::rust::isize
qsizetypeIntoIsize(::qsizetype value)
{
  return static_cast<::rust::isize>(value);
}

}
}
