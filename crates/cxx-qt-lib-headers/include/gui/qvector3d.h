// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#ifdef CXX_QT_GUI_FEATURE
#include <QtGui/QVector3D>

namespace rust {
namespace cxxqtlib1 {

// Qt 6 uses by-value, Qt 5 uses by-ref
float
qvector3DDistanceToLine(const QVector3D& vector,
                        QVector3D point,
                        QVector3D direction);
float
qvector3DDistanceToPlane(const QVector3D& vector,
                         QVector3D plane,
                         QVector3D normal);
float
qvector3DDistanceToPoint(const QVector3D& vector, QVector3D point);

}
}
#endif
