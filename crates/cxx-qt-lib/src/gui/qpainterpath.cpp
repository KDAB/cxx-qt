// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#ifdef CXX_QT_GUI_FEATURE
#include "cxx-qt-lib/qpainterpath.h"
#include "../assertion_utils.h"

// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qpainterpath.h?h=v5.15.6-lts-lgpl#n227
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qpainterpath.h?h=v6.2.4#n200
assert_alignment_and_size(QPainterPath,
                          alignof(::std::size_t),
                          sizeof(::std::size_t));

static_assert(QTypeInfo<QPainterPath>::isRelocatable);

static_assert(!::std::is_trivially_copy_assignable<QPainterPath>::value);
static_assert(!::std::is_trivially_copy_constructible<QPainterPath>::value);

static_assert(!::std::is_trivially_destructible<QPainterPath>::value);

#endif
