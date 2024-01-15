// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QVector2D>

namespace rust {
namespace cxxqtlib1 {

// Qt 6 uses by-value, Qt 5 uses by-ref
float
qvector2DDistanceToLine(const QVector2D& vector,
                        QVector2D point,
                        QVector2D direction);
float
qvector2DDistanceToPoint(const QVector2D& vector, QVector2D point);

}
}
