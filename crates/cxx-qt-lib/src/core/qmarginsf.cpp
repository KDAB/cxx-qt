// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qmarginsf.h"

#include <cxx-qt-lib/assertion_utils.h>

#include <cstdint>

// QMarginsF has "qreal" members - left, top, right, bottom
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qmargins.h?h=v5.15.6-lts-lgpl#n314
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qmargins.h?h=v6.2.4#n329
assert_alignment_and_size(QMarginsF, {
  double a0;
  double a1;
  double a2;
  double a3;
});

static_assert(::std::is_trivially_copyable<QMarginsF>::value);
