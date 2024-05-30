// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qquickstyle.h"

namespace rust {
namespace cxxqtlib1 {

QString
qquickstyleName()
{
  return QQuickStyle::name();
}

void
qquickstyleSetFallbackStyle(const QString& style)
{
  QQuickStyle::setFallbackStyle(style);
}

void
qquickstyleSetStyle(const QString& style)
{
  QQuickStyle::setStyle(style);
}

}
}
