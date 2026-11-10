// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qchar.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QChar, { ::std::uint16_t ucs; });

static_assert(::std::is_trivially_copy_assignable<QChar>::value);
static_assert(::std::is_trivially_copy_constructible<QChar>::value);
static_assert(::std::is_trivially_destructible<QChar>::value);
static_assert(::std::is_move_constructible<QChar>::value);
static_assert(QTypeInfo<QChar>::isRelocatable);
