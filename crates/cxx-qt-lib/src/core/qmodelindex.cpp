// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qmodelindex.h"

#include <cxx-qt-lib/assertion_utils.h>

// QModelIndex has two ints, a quint pointer (same as size_t), and a pointer.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/itemmodels/qabstractitemmodel.h?h=v5.15.6-lts-lgpl#n93
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/itemmodels/qabstractitemmodel.h?h=v6.2.4#n195
assert_alignment_and_size(QModelIndex, {
  ::std::int32_t a0;
  ::std::int32_t a1;
  ::std::size_t a2;
  ::std::size_t a3;
});

static_assert(::std::is_trivially_copyable<QModelIndex>::value);
