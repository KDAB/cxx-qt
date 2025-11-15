// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QQuaternion>

namespace rust {
namespace cxxqtlib1 {

inline float (*qquaternionDotProduct)(const QQuaternion&, const QQuaternion&) =
  QQuaternion::dotProduct;

inline QQuaternion (*qquaternionFromAxes)(const QVector3D&,
                                          const QVector3D&,
                                          const QVector3D&) =
  QQuaternion::fromAxes;

inline QQuaternion (*qquaternionFromAxisAndAngle)(float, float, float, float) =
  QQuaternion::fromAxisAndAngle;

inline QQuaternion (*qquaternionFromDirection)(const QVector3D&,
                                               const QVector3D&) =
  QQuaternion::fromDirection;

inline QQuaternion (*qquaternionFromEulerAngles)(float, float, float) =
  QQuaternion::fromEulerAngles;

inline QQuaternion (*qquaternionFromRotationMatrix)(const QMatrix3x3&) =
  QQuaternion::fromRotationMatrix;

inline QQuaternion (*qquaternionNlerp)(const QQuaternion&,
                                       const QQuaternion&,
                                       float) = QQuaternion::nlerp;

inline QQuaternion (*qquaternionRotationTo)(const QVector3D&,
                                            const QVector3D&) =
  QQuaternion::rotationTo;

inline QQuaternion (*qquaternionSlerp)(const QQuaternion&,
                                       const QQuaternion&,
                                       float) = QQuaternion::slerp;

}
}
