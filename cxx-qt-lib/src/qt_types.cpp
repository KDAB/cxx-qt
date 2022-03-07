// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group
// company <info@kdab.com> SPDX-FileContributor: Andrew Hayzen
// <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/include/qt_types.h"

namespace rust {
namespace cxxqtlib1 {

std::unique_ptr<QColor>
qcolorInitFromRgba(std::int32_t r,
                   std::int32_t g,
                   std::int32_t b,
                   std::int32_t a)
{
  return std::make_unique<QColor>(r, g, b, a);
}

std::unique_ptr<QColor>
qcolorInitFromQColor(const QColor& color)
{
  return std::make_unique<QColor>(color);
}

QPoint
qpointInitDefault()
{
  return QPoint();
}

QPoint
qpointInit(int x, int y)
{
  return QPoint(x, y);
}

}
}
