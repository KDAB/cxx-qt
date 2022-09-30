// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qcolor.h"

#include "assertion_utils.h"

// QColor has an enum with six values and a union with the largest being five
// ushorts. This results in (5 * std::uint16) + std::uint32_t = 14, then due to
// compiler padding this results in a sizeof 16 or two pointers.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qcolor.h?h=v5.15.6-lts-lgpl#n262
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qcolor.h?h=v6.2.4#n237
assert_alignment_and_size(QColor, alignof(std::size_t), sizeof(std::size_t[2]));

// QColor still had copy & move constructors in Qt 5 but they were basically
// trivial.
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
static_assert(std::is_trivially_copyable<QColor>::value);
#else
static_assert(QTypeInfo<QColor>::isRelocatable);
#endif

static_assert(std::is_trivially_destructible<QColor>::value);

namespace rust {
namespace cxxqtlib1 {

QColor
qcolorInitDefault()
{
  return QColor();
}

QColor
qcolorInitFromRgba(std::int32_t r,
                   std::int32_t g,
                   std::int32_t b,
                   std::int32_t a)
{
  return QColor(r, g, b, a);
}

QColor
qcolorInitFromQColor(const QColor& color)
{
  return QColor(color);
}

}

}
