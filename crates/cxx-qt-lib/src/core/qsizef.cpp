// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qsizef.h"

#include "../assertion_utils.h"

// QSizeF has two qreal members
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qsize.h?h=v5.15.6-lts-lgpl#n276
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qsize.h?h=v6.2.4#n302
assert_alignment_and_size(QSizeF, alignof(double), sizeof(double[2]));

static_assert(::std::is_trivially_copyable<QSizeF>::value,
              "QSizeF must be trivially copyable!");
