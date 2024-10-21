// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Nicolas Fella <nicolas.fella@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib-extras/qlocale.h"
#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QLocale, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QLocale>::value);
static_assert(!::std::is_trivially_copy_constructible<QLocale>::value);

static_assert(!::std::is_trivially_destructible<QLocale>::value);

static_assert(QTypeInfo<QLocale>::isRelocatable);
