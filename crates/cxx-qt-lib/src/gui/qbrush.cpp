// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qbrush.h"

#include "../assertion_utils.h"

// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qbrush.h?h=v5.15.6-lts-lgpl#n130
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qbrush.h?h=v6.2.4#n123
assert_alignment_and_size(QBrush,
                          alignof(::std::size_t),
                          sizeof(::std::size_t));

static_assert(!::std::is_trivially_copy_assignable<QBrush>::value);
static_assert(!::std::is_trivially_copy_constructible<QBrush>::value);

static_assert(!::std::is_trivially_destructible<QBrush>::value);

static_assert(QTypeInfo<QBrush>::isRelocatable);
