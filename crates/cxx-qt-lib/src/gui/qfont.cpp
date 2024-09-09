// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qfont.h"

#include <cxx-qt-lib/assertion_utils.h>

// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/text/qfont.h?h=v5.15.6-lts-lgpl#n344
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/text/qfont.h?h=v6.2.4#n323
assert_alignment_and_size(QFont, {
  ::std::size_t a0;
  ::std::uint32_t a1;
}); // uint can be 16 or 32 but should align to at least 32

static_assert(!::std::is_trivially_copy_assignable<QFont>::value);
static_assert(!::std::is_trivially_copy_constructible<QFont>::value);

static_assert(!::std::is_trivially_destructible<QFont>::value);
static_assert(QTypeInfo<QFont>::isRelocatable);
