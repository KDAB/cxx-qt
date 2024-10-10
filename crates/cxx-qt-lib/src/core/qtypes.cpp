// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qtypes.h"

#include "cxx-qt-lib/assertion_utils.h"

assert_alignment_and_size(qint64, { ::std::int64_t a0; });
assert_alignment_and_size(qintptr, { ::std::intptr_t a0; });
assert_alignment_and_size(quint64, { ::std::uint64_t a0; });
assert_alignment_and_size(quintptr, { ::std::uintptr_t a0; });
assert_alignment_and_size(qsizetype, { ::std::size_t a0; });

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

::qintptr
qintptrFromIsize(::rust::isize value)
{
  return static_cast<::qintptr>(value);
}

::rust::isize
qintptrIntoIsize(::qintptr value)
{
  return static_cast<::rust::isize>(value);
}

::quint64
quint64FromU64(::std::uint64_t value)
{
  return static_cast<::quint64>(value);
}

::std::uint64_t
quint64IntoU64(::quint64 value)
{
  return static_cast<::std::uint64_t>(value);
}

::quintptr
quintptrFromUsize(::rust::usize value)
{
  return static_cast<::quintptr>(value);
}

::rust::usize
quintptrIntoUsize(::quintptr value)
{
  return static_cast<::rust::usize>(value);
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
