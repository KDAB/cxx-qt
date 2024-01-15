// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QFont>

#include "rust/cxx.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<QFont> : ::std::true_type
{
};

namespace cxxqtlib1 {
using QFontStyle = QFont::Style;
using QFontHintingPreference = QFont::HintingPreference;
using QFontCapitalization = QFont::Capitalization;
using QFontSpacingType = QFont::SpacingType;

} // namespace cxxqtlib1
} // namespace rust
