// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#ifdef CXX_QT_QUICKCONTROLS_FEATURE

#include <memory>

#include <QtQuickControls2/QQuickStyle>

namespace rust {
namespace cxxqtlib1 {

QString
qquickstyleName();

void
qquickstyleSetFallbackStyle(const QString& style);

void
qquickstyleSetStyle(const QString& style);

}
}

#endif