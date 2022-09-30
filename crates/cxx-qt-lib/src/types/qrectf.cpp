// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qrectf.h"

#include "assertion_utils.h"

// QRectF has 4 double members
// https://codebrowser.dev/qt5/qtbase/src/corelib/tools/qrect.h.html#QRectF::xp
//
// https://codebrowser.dev/qt6/qtbase/src/corelib/tools/qrect.h.html#QRectF::xp
assert_alignment_and_size(QRectF, alignof(double), sizeof(double[4]));

static_assert(std::is_trivially_copyable<QRectF>::value,
              "QRectF must be trivially copyable");

namespace rust {
namespace cxxqtlib1 {

QRectF
qrectfInitDefault()
{
  return QRectF();
}

QRectF
qrectfInit(qreal x, qreal y, qreal w, qreal h)
{
  return QRectF(x, y, w, h);
}

}
}
