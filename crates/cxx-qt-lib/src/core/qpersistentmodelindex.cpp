// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qpersistentmodelindex.h"

#include "../assertion_utils.h"

// QPersistentModelIndex is a single pointer to a QPersistentModelIndexData
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/itemmodels/qabstractitemmodel.h?h=v5.15.6-lts-lgpl#n143
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/itemmodels/qabstractitemmodel.h?h=v6.2.4#n243
assert_alignment_and_size(QPersistentModelIndex,
                          alignof(::std::size_t),
                          sizeof(::std::size_t));

static_assert(
  !::std::is_trivially_copy_assignable<QPersistentModelIndex>::value);
static_assert(
  !::std::is_trivially_copy_constructible<QPersistentModelIndex>::value);

static_assert(!::std::is_trivially_destructible<QPersistentModelIndex>::value);

static_assert(QTypeInfo<QPersistentModelIndex>::isRelocatable);
