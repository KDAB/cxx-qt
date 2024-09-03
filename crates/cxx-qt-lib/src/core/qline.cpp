// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qline.h"

#include <cxx-qt-lib/assertion_utils.h>

#include <cstdint>

// QLine has "QPoint" members - pt1, pt2
// Rust represents these as 32-bit integer types.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qline.h?h=v5.15.6-lts-lgpl#n90
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qline.h?h=v6.2.4#n90
assert_alignment_and_size(QLine, {
  ::std::int32_t a0;
  ::std::int32_t a1;
  ::std::int32_t a2;
  ::std::int32_t a3;
});

static_assert(::std::is_trivially_copyable<QLine>::value);
