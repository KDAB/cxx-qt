// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QFont>

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {
namespace cxxqtlib1 {
using QFontStyle = QFont::Style;
using QFontHintingPreference = QFont::HintingPreference;
using QFontCapitalization = QFont::Capitalization;
using QFontSpacingType = QFont::SpacingType;
using QFontStyleStrategy = QFont::StyleStrategy;
using QFontStyleHint = QFont::StyleHint;
using QFontWeight = QFont::Weight;

void
qfontResolve(const QFont& font, const QFont& other, QFont* uninit);

} // namespace cxxqtlib1
} // namespace rust
