// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qregion.h"

#include "../assertion_utils.h"

// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qregion.h?h=v5.15.6-lts-lgpl#n178
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qregion.h?h=v6.2.4#n161
assert_alignment_and_size(QRegion,
                          alignof(::std::size_t),
                          sizeof(::std::size_t));

static_assert(!::std::is_trivially_copy_assignable<QRegion>::value);
static_assert(!::std::is_trivially_copy_constructible<QRegion>::value);

static_assert(!::std::is_trivially_destructible<QRegion>::value);

static_assert(QTypeInfo<QRegion>::isRelocatable);
