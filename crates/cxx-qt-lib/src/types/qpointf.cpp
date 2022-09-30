// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qpointf.h"

#include "assertion_utils.h"

// QPointF has two "qreal" members - xp and yp
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qpoint.h?h=v5.15.6-lts-lgpl#n271
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qpoint.h?h=v6.2.4#n313
assert_alignment_and_size(QPointF, alignof(double), sizeof(double[2]));

static_assert(std::is_trivially_copyable<QPointF>::value,
              "QPointF should be trivially copyable");

namespace rust {
namespace cxxqtlib1 {

QPointF
qpointfInitDefault()
{
  return QPointF();
}

QPointF
qpointfInit(qreal x, qreal y)
{
  return QPointF(x, y);
}

}
}
