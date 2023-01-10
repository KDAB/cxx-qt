// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qsize.h"

#include "assertion_utils.h"

#include <cstdint>

// QSize has two "int" members
// Rust represents these as 32-bit integers.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qsize.h?h=v5.15.6-lts-lgpl#n104
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qsize.h?h=v6.2.4#n113
assert_alignment_and_size(QSize,
                          alignof(::std::int32_t),
                          sizeof(::std::int32_t[2]));

static_assert(::std::is_trivially_copyable<QSize>::value,
              "QSize must be trivially copyable!");
