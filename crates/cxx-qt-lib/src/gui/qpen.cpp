// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#ifdef CXX_QT_GUI_FEATURE
#include "cxx-qt-lib/qpen.h"

#include "../assertion_utils.h"

// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qpen.h?h=v5.15.6-lts-lgpl#n124
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qpen.h??h=v6.2.4#n94
#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))
assert_alignment_and_size(QPen,
                          alignof(::std::size_t),
                          sizeof(::std::size_t) * 2);
#else
assert_alignment_and_size(QPen,
                          alignof(::std::size_t),
                          sizeof(::std::size_t));
#endif

static_assert(!::std::is_trivially_copy_assignable<QPen>::value);
static_assert(!::std::is_trivially_copy_constructible<QPen>::value);

static_assert(!::std::is_trivially_destructible<QPen>::value);

#endif
