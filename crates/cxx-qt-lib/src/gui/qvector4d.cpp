// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qvector4d.h"

#include <cxx-qt-lib/assertion_utils.h>

// QVector2D has two float members - xp and yp
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/math3d/qvector4d.h?h=v5.15.6-lts-lgpl#n131
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/math3d/qvectornd.h?h=v6.2.4#n490
assert_alignment_and_size(QVector4D, {
  float a0;
  float a1;
  float a2;
  float a3;
});

static_assert(::std::is_trivially_copyable<QVector4D>::value,
              "QVector4D should be trivially copyable");
