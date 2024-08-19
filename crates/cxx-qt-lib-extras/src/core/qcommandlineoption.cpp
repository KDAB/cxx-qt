// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib-extras/qcommandlineoption.h"

#include "../assertion_utils.h"

#include <cstdint>

// QCommandLineOption has 1 pointer
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qcommandlineoption.h?h=v5.15.6-lts-lgpl#n59
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qcommandlineoption.h?h=v6.2.4#n96
constexpr static ::std::array<::std::size_t, 1> arr{ sizeof(::std::size_t) };
assert_alignment_and_size(QCommandLineOption, alignof(::std::size_t), arr);

static_assert(!::std::is_trivially_copy_assignable<QCommandLineOption>::value);
static_assert(
  !::std::is_trivially_copy_constructible<QCommandLineOption>::value);

static_assert(QTypeInfo<QCommandLineOption>::isRelocatable);
