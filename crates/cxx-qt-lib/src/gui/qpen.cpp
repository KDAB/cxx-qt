// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qpen.h"

#include "../assertion_utils.h"

// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qpen.h?h=v5.15.6-lts-lgpl#n124
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qpen.h?h=v6.2.4#n94
constexpr static ::std::array<::std::size_t, 1> arr{ sizeof(::std::size_t) };
assert_alignment_and_size(QPen, alignof(::std::size_t), arr);

static_assert(!::std::is_trivially_copy_assignable<QPen>::value);
static_assert(!::std::is_trivially_copy_constructible<QPen>::value);

static_assert(!::std::is_trivially_destructible<QPen>::value);

static_assert(QTypeInfo<QPen>::isRelocatable);
