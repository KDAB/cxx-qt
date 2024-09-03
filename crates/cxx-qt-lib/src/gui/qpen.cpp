// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qpen.h"

#include <cxx-qt-lib/assertion_utils.h>

// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qpen.h?h=v5.15.6-lts-lgpl#n124
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qpen.h?h=v6.2.4#n94
assert_alignment_and_size(QPen, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QPen>::value);
static_assert(!::std::is_trivially_copy_constructible<QPen>::value);

static_assert(!::std::is_trivially_destructible<QPen>::value);

static_assert(QTypeInfo<QPen>::isRelocatable);
