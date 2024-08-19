// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qfont.h"

#include "../assertion_utils.h"

// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/text/qfont.h?h=v5.15.6-lts-lgpl#n344
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/text/qfont.h?h=v6.2.4#n323
constexpr static ::std::array<::std::size_t, 2> arr{ sizeof(::std::size_t),
                                                     sizeof(uint) };
assert_alignment_and_size(QFont, alignof(::std::size_t), arr);

static_assert(!::std::is_trivially_copy_assignable<QFont>::value);
static_assert(!::std::is_trivially_copy_constructible<QFont>::value);

static_assert(!::std::is_trivially_destructible<QFont>::value);
static_assert(QTypeInfo<QFont>::isRelocatable);
