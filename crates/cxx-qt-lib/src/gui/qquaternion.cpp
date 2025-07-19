// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qquaternion.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QQuaternion, {
  float wp;
  float xp;
  float yp;
  float zp;
});

static_assert(::std::is_trivially_copyable<QQuaternion>::value,
              "QQuaternion should be trivially copyable");
