// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qrect.h"

#include "assertion_utils.h"

#include <cstdint>

// QRect has 4 int members.
// Rust represents them as 4 32-bit integers
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qrect.h?h=v5.15.6-lts-lgpl#n161
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qrect.h?h=v6.2.4#n161
assert_alignment_and_size(QRect,
                          alignof(::std::int32_t),
                          sizeof(::std::int32_t[4]));

static_assert(::std::is_trivially_copyable<QRect>::value,
              "QRect must be trivially copyable");
