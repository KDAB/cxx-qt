// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qmargins.h"

#include "../assertion_utils.h"

#include <cstdint>

// QMargins has "int" members - left, top, right, bottom
// Rust represents these as 32-bit integer types.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qmargins.h?h=v5.15.6-lts-lgpl#n79
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qmargins.h?h=v6.2.4#n79
constexpr static ::std::array<::std::size_t, 4> arr{ sizeof(::std::int32_t),
                                                     sizeof(::std::int32_t),
                                                     sizeof(::std::int32_t),
                                                     sizeof(::std::int32_t) };
assert_alignment_and_size(QMargins, alignof(::std::int32_t), arr);

static_assert(::std::is_trivially_copyable<QMargins>::value);
