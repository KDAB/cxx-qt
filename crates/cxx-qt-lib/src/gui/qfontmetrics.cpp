// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qfontmetrics.h"

#include "../assertion_utils.h"

// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/text/qfontmetrics.h?h=v5.15.6-lts-lgpl#n147
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/text/qfontmetrics.h?h=v6.2.4#n117
assert_alignment_and_size(QFontMetrics,
                          alignof(::std::size_t),
                          sizeof(::std::size_t));

static_assert(!::std::is_trivially_copy_assignable<QFontMetrics>::value);
static_assert(!::std::is_trivially_copy_constructible<QFontMetrics>::value);

static_assert(!::std::is_trivially_destructible<QFontMetrics>::value);
static_assert(QTypeInfo<QFontMetrics>::isRelocatable);
