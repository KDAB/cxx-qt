// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qgenericmatrix.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QMatrix2x3, { float m[3][2]; });

static_assert(::std::is_trivially_copyable<QMatrix2x3>::value);
