// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qmodelindex.h"

#include "../assertion_utils.h"

// QModelIndex has two ints, a quint pointer (same as size_t), and a pointer.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/itemmodels/qabstractitemmodel.h?h=v5.15.6-lts-lgpl#n93
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/itemmodels/qabstractitemmodel.h?h=v6.2.4#n195
constexpr static ::std::array<::std::size_t, 4> arr{ sizeof(::std::int32_t),
                                                     sizeof(::std::int32_t),
                                                     sizeof(::std::size_t),
                                                     sizeof(::std::size_t) };
assert_alignment_and_size(QModelIndex, alignof(::std::size_t), arr, arr.size());

static_assert(::std::is_trivially_copyable<QModelIndex>::value);

namespace rust {
namespace cxxqtlib1 {

::std::size_t
qmodelindexInternalId(const QModelIndex& index)
{
  // TODO: need to add support for quintptr
  return static_cast<::std::size_t>(index.internalId());
}

}
}
