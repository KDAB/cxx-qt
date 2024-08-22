// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qrectf.h"

#include "../assertion_utils.h"

// QRectF has 4 double members
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qrect.h?h=v5.15.6-lts-lgpl#n621
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qrect.h?h=v6.2.4#n623
constexpr static ::std::array<::std::size_t, 4> arr{ sizeof(double),
                                                     sizeof(double),
                                                     sizeof(double),
                                                     sizeof(double) };
assert_alignment_and_size(QRectF, alignof(double), arr, arr.size());

static_assert(::std::is_trivially_copyable<QRectF>::value,
              "QRectF must be trivially copyable");
