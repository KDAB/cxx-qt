// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qcolor.h"
#include "cxx-qt-lib/qstring.h"

#include <cxx-qt-lib/assertion_utils.h>

// QColor has an enum with six values and a union with the largest being five
// ushorts. This results in std::int32_t + (5 * std::uint16) = 14, then due to
// compiler padding this results in a sizeof 16.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qcolor.h?h=v5.15.6-lts-lgpl#n262
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/gui/painting/qcolor.h?h=v6.2.4#n237
assert_alignment_and_size(QColor, {
  ::std::int32_t a0;
  ::std::uint16_t a1;
  ::std::uint16_t a2;
  ::std::uint16_t a3;
  ::std::uint16_t a4;
  ::std::uint16_t a5;
});

// QColor still had copy & move constructors in Qt 5 but they were basically
// trivial.
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
static_assert(::std::is_trivially_copyable<QColor>::value);
#else
static_assert(QTypeInfo<QColor>::isRelocatable);
#endif

static_assert(::std::is_trivially_destructible<QColor>::value);

namespace rust {
namespace cxxqtlib1 {

QStringList
qcolorColorNames()
{
  return QColor::colorNames();
}

QColor
qcolorInitFromCmyk(::std::int32_t c,
                   ::std::int32_t m,
                   ::std::int32_t y,
                   ::std::int32_t k,
                   ::std::int32_t a)
{
  return QColor::fromCmyk(c, m, y, k, a);
}

QColor
qcolorInitFromCmykF(float c, float m, float y, float k, float a)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  return QColor::fromCmykF(c, m, y, k, a);
#else
  return QColor::fromCmykF(static_cast<qreal>(c),
                           static_cast<qreal>(m),
                           static_cast<qreal>(y),
                           static_cast<qreal>(k),
                           static_cast<qreal>(a));
#endif
}

QColor
qcolorInitFromHsl(::std::int32_t h,
                  ::std::int32_t s,
                  ::std::int32_t l,
                  ::std::int32_t a)
{
  return QColor::fromHsl(h, s, l, a);
}

QColor
qcolorInitFromHslF(float h, float s, float l, float a)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  return QColor::fromHslF(h, s, l, a);
#else
  return QColor::fromHslF(static_cast<qreal>(h),
                          static_cast<qreal>(s),
                          static_cast<qreal>(l),
                          static_cast<qreal>(a));
#endif
}

QColor
qcolorInitFromHsv(::std::int32_t h,
                  ::std::int32_t s,
                  ::std::int32_t v,
                  ::std::int32_t a)
{
  return QColor::fromHsv(h, s, v, a);
}

QColor
qcolorInitFromHsvF(float h, float s, float v, float a)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  return QColor::fromHsvF(h, s, v, a);
#else
  return QColor::fromHsvF(static_cast<qreal>(h),
                          static_cast<qreal>(s),
                          static_cast<qreal>(v),
                          static_cast<qreal>(a));
#endif
}

QColor
qcolorInitFromRgb(::std::int32_t red,
                  ::std::int32_t green,
                  ::std::int32_t blue,
                  ::std::int32_t alpha)
{
  return QColor::fromRgb(red, green, blue, alpha);
}

QColor
qcolorInitFromRgbF(float red, float green, float blue, float alpha)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  return QColor::fromRgbF(red, green, blue, alpha);
#else
  return QColor::fromRgbF(static_cast<qreal>(red),
                          static_cast<qreal>(green),
                          static_cast<qreal>(blue),
                          static_cast<qreal>(alpha));
#endif
}

QColor
qcolorInitFromRustString(::rust::Str string)
{
  return QColor(qstringInitFromRustString(string));
}

// Qt 5 uses qreal and Qt 6 uses float, so cast all to floats

float
qcolorAlphaF(const QColor& color)
{
  return static_cast<float>(color.alphaF());
}

float
qcolorBlackF(const QColor& color)
{
  return static_cast<float>(color.blackF());
}

float
qcolorBlueF(const QColor& color)
{
  return static_cast<float>(color.blueF());
}

float
qcolorCyanF(const QColor& color)
{
  return static_cast<float>(color.cyanF());
}

float
qcolorGreenF(const QColor& color)
{
  return static_cast<float>(color.greenF());
}

float
qcolorHslHueF(const QColor& color)
{
  return static_cast<float>(color.hslHueF());
}

float
qcolorHslSaturationF(const QColor& color)
{
  return static_cast<float>(color.hslSaturationF());
}

float
qcolorHsvHueF(const QColor& color)
{
  return static_cast<float>(color.hsvHueF());
}

float
qcolorHsvSaturationF(const QColor& color)
{
  return static_cast<float>(color.hsvSaturationF());
}

float
qcolorHueF(const QColor& color)
{
  return static_cast<float>(color.hueF());
}

float
qcolorLightnessF(const QColor& color)
{
  return static_cast<float>(color.lightnessF());
}

float
qcolorMagentaF(const QColor& color)
{
  return static_cast<float>(color.magentaF());
}

float
qcolorRedF(const QColor& color)
{
  return static_cast<float>(color.redF());
}

float
qcolorSaturationF(const QColor& color)
{
  return static_cast<float>(color.saturationF());
}

void
qcolorSetAlphaF(QColor& color, float alpha)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  color.setAlphaF(alpha);
#else
  color.setAlphaF(static_cast<qreal>(alpha));
#endif
}

void
qcolorSetBlueF(QColor& color, float blue)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  color.setBlueF(blue);
#else
  color.setBlueF(static_cast<qreal>(blue));
#endif
}

void
qcolorSetCmykF(QColor& color, float c, float m, float y, float k, float a)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  color.setCmykF(c, m, y, k, a);
#else
  color.setCmykF(static_cast<qreal>(c),
                 static_cast<qreal>(m),
                 static_cast<qreal>(y),
                 static_cast<qreal>(k),
                 static_cast<qreal>(a));
#endif
}

void
qcolorSetGreenF(QColor& color, float green)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  color.setGreenF(green);
#else
  color.setGreenF(static_cast<qreal>(green));
#endif
}

void
qcolorSetHslF(QColor& color, float h, float s, float l, float a)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  color.setHslF(h, s, l, a);
#else
  color.setHslF(static_cast<qreal>(h),
                static_cast<qreal>(s),
                static_cast<qreal>(l),
                static_cast<qreal>(a));
#endif
}

void
qcolorSetHsvF(QColor& color, float h, float s, float v, float a)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  color.setHsvF(h, s, v, a);
#else
  color.setHsvF(static_cast<qreal>(h),
                static_cast<qreal>(s),
                static_cast<qreal>(v),
                static_cast<qreal>(a));
#endif
}

void
qcolorSetRedF(QColor& color, float red)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  color.setRedF(red);
#else
  color.setRedF(static_cast<qreal>(red));
#endif
}

void
qcolorSetRgbF(QColor& color, float r, float g, float b, float a)
{
  // Qt 6 is float and Qt 5 is qreal
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  color.setRgbF(r, g, b, a);
#else
  color.setRgbF(static_cast<qreal>(r),
                static_cast<qreal>(g),
                static_cast<qreal>(b),
                static_cast<qreal>(a));
#endif
}

float
qcolorValueF(const QColor& color)
{
  return static_cast<float>(color.valueF());
}

float
qcolorYellowF(const QColor& color)
{
  return static_cast<float>(color.yellowF());
}

}
}
