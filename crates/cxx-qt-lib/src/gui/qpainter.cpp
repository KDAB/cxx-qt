// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#ifdef CXX_QT_GUI_FEATURE
#include "cxx-qt-lib/qpainter.h"

#include "../assertion_utils.h"

#include <cstdint>

// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qpainter.h?h=v5.15.6-lts-lgpl#n504
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qpainter.h?h=v6.2.4#n451
assert_alignment_and_size(QPainter,
                          alignof(::std::size_t),
                          sizeof(::std::size_t));

static_assert(!::std::is_trivially_copy_assignable<QPainter>::value);
static_assert(!::std::is_trivially_copy_constructible<QPainter>::value);

static_assert(!::std::is_trivially_destructible<QPainter>::value);

#endif
