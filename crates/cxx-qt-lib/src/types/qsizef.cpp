// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qsizef.h"

#include "assertion_utils.h"

// QSizeF has two qreal members
// https://codebrowser.dev/qt5/qtbase/src/corelib/tools/qsize.h.html#QSizeF::wd
//
// https://codebrowser.dev/qt6/qtbase/src/corelib/tools/qsize.h.html#QSizeF::wd
assert_alignment_and_size(QSizeF, alignof(double), sizeof(double[2]));

static_assert(std::is_trivially_copyable<QSizeF>::value,
              "QSizeF must be trivially copyable!");

namespace rust {
namespace cxxqtlib1 {

QSizeF
qsizefInitDefault()
{
  return QSizeF();
}

QSizeF
qsizefInit(qreal width, qreal height)
{
  return QSizeF(width, height);
}

}
}
