// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qvector3d.h"

#include "assertion_utils.h"

// QVector2D has two float members - xp and yp
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/math3d/qvector3d.h?h=v5.15.6-lts-lgpl#n141
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/math3d/qvectornd.h?h=v6.2.4#n334
assert_alignment_and_size(QVector3D, alignof(float), sizeof(float[3]));

static_assert(::std::is_trivially_copyable<QVector3D>::value,
              "QPointF should be trivially copyable");

namespace rust {
namespace cxxqtlib1 {

// Qt 6 uses by-value, Qt 5 uses by-ref
float
qvector3DDistanceToLine(const QVector3D& vector,
                        QVector3D point,
                        QVector3D direction)
{
  return vector.distanceToLine(point, direction);
}

float
qvector3DDistanceToPlane(const QVector3D& vector,
                         QVector3D plane,
                         QVector3D normal)
{
  return vector.distanceToPlane(plane, normal);
}

float
qvector3DDistanceToPoint(const QVector3D& vector, QVector3D point)
{
  return vector.distanceToPoint(point);
}

}
}
