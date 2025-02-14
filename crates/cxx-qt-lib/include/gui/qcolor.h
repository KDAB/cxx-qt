// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cinttypes>

#include <QtCore/QStringList>
#include <QtGui/QColor>

#include "rust/cxx.h"

// QColor still had copy & move constructors in Qt 5 but they were basically
// trivial.
#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))
// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<QColor> : ::std::true_type
{};

} // namespace rust
#endif

namespace rust {
namespace cxxqtlib1 {

using QColorNameFormat = QColor::NameFormat;
using QColorSpec = QColor::Spec;

QStringList
qcolorColorNames();
QColor
qcolorInitFromCmyk(::std::int32_t c,
                   ::std::int32_t m,
                   ::std::int32_t y,
                   ::std::int32_t k,
                   ::std::int32_t a);
QColor
qcolorInitFromCmykF(float c, float m, float y, float k, float a);
QColor
qcolorInitFromHsl(::std::int32_t h,
                  ::std::int32_t s,
                  ::std::int32_t l,
                  ::std::int32_t a);
QColor
qcolorInitFromHslF(float h, float s, float l, float a);
QColor
qcolorInitFromHsv(::std::int32_t h,
                  ::std::int32_t s,
                  ::std::int32_t v,
                  ::std::int32_t a);
QColor
qcolorInitFromHsvF(float h, float s, float v, float a);
QColor
qcolorInitFromRgb(::std::int32_t red,
                  ::std::int32_t green,
                  ::std::int32_t blue,
                  ::std::int32_t alpha);
QColor
qcolorInitFromRgbF(float red, float green, float blue, float alpha);

// Qt 5 uses qreal and Qt 6 uses float, so cast all to floats
float
qcolorAlphaF(const QColor& color);
float
qcolorBlackF(const QColor& color);
float
qcolorBlueF(const QColor& color);
float
qcolorCyanF(const QColor& color);
float
qcolorGreenF(const QColor& color);
float
qcolorHslHueF(const QColor& color);
float
qcolorHslSaturationF(const QColor& color);
float
qcolorHsvHueF(const QColor& color);
float
qcolorHsvSaturationF(const QColor& color);
float
qcolorHueF(const QColor& color);
float
qcolorLightnessF(const QColor& color);
float
qcolorMagentaF(const QColor& color);
float
qcolorRedF(const QColor& color);
float
qcolorSaturationF(const QColor& color);
void
qcolorSetAlphaF(QColor& color, float alpha);
void
qcolorSetBlueF(QColor& color, float blue);
void
qcolorSetCmykF(QColor& color, float c, float m, float y, float k, float a);
void
qcolorSetGreenF(QColor& color, float green);
void
qcolorSetHslF(QColor& color, float h, float s, float l, float a);
void
qcolorSetHsvF(QColor& color, float h, float s, float v, float a);
void
qcolorSetRedF(QColor& color, float red);
void
qcolorSetRgbF(QColor& color, float r, float g, float b, float a);
float
qcolorValueF(const QColor& color);
float
qcolorYellowF(const QColor& color);

}
}
