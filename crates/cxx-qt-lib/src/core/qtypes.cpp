// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qtypes.h"

#include <cstdint>

#include "cxx-qt-lib/assertion_utils.h"

assert_alignment_and_size(qint64, { ::std::int64_t a0; });
assert_alignment_and_size(qintptr, { ::std::intptr_t a0; });
assert_alignment_and_size(quint64, { ::std::uint64_t a0; });
assert_alignment_and_size(quintptr, { ::std::uintptr_t a0; });
assert_alignment_and_size(qsizetype, { ::std::size_t a0; });
// We only support qreal being a double
assert_alignment_and_size(qreal, { double a0; });
