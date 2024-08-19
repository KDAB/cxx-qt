// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qlinef.h"

#include "../assertion_utils.h"

#include <cstdint>

// QLineF has "QPointF" members - pt1, pt2
// Rust represents these as double types.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qline.h?h=v5.15.6-lts-lgpl#n281
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qline.h?h=v6.2.4#n295
constexpr static ::std::array<::std::size_t, 4> arr{ sizeof(double),
                                                     sizeof(double),
                                                     sizeof(double),
                                                     sizeof(double) };
assert_alignment_and_size(QLineF, alignof(double), arr);

static_assert(::std::is_trivially_copyable<QLineF>::value);
