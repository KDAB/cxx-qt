// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qvector2d.h"

#include "../assertion_utils.h"

// QVector2D has two float members - xp and yp
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/math3d/qvector2d.h?h=v5.15.6-lts-lgpl#n126
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/math3d/qvectornd.h?h=v6.2.4#n176
constexpr static ::std::array<::std::size_t, 2> arr{ sizeof(float),
                                                     sizeof(float) };
assert_alignment_and_size(QVector2D, alignof(float), arr);

static_assert(::std::is_trivially_copyable<QVector2D>::value,
              "QVector2D should be trivially copyable");

namespace rust {
namespace cxxqtlib1 {

// Qt 6 uses by-value, Qt 5 uses by-ref
float
qvector2DDistanceToLine(const QVector2D& vector,
                        QVector2D point,
                        QVector2D direction)
{
  return vector.distanceToLine(point, direction);
}

float
qvector2DDistanceToPoint(const QVector2D& vector, QVector2D point)
{
  return vector.distanceToPoint(point);
}

}
}
