// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <QtCore/QDebug>

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "bridge.h"
#include "cxx-qt-gen/include/lib.rs.h"

TEST_CASE("Can construct a QString on the Rust side")
{
  // From a slice
  CHECK(can_construct_qstring(true));

  // From a rust::String
  CHECK(can_construct_qstring(false));
}

TEST_CASE("Can read a QString on the Rust side")
{
  const auto s = QStringLiteral("String constructed by C++");
  CHECK(can_read_qstring(s));
}

TEST_CASE("Can modify a QString on the Rust side")
{
  auto s = QStringLiteral("String constructed by C++");
  modify_qstring(s);
  CHECK_EQ(s, QStringLiteral("Updated string value"));
}

TEST_CASE("Can map Rust &str to &QString")
{
  CHECK(can_map_to_qstring());
}

TEST_CASE("Can handle a QString modified on the Rust side")
{
  CHECK(can_handle_qstring_change());
}

bool
test_constructed_qstring(const QString& s)
{
  return s == QStringLiteral("String constructed by Rust");
}

void
assign_to_qstring(QString& s, const QString& v)
{
  s = v;

  // Force some more activity for valgrind to inspect
  s.detach();
}

TEST_CASE("Can construct a QColor on the Rust side")
{
  CHECK(can_construct_qcolor(ColorTest::Rgb_Red));
  CHECK(can_construct_qcolor(ColorTest::Rgb_Green));
  CHECK(can_construct_qcolor(ColorTest::Rgb_Blue));
  CHECK(can_construct_qcolor(ColorTest::Rgb_Transparent));
}

bool
test_constructed_qcolor(const QColor& c, ColorTest test)
{
  switch (test) {
    case ColorTest::Rgb_Red:
      return c.alpha() == 255 && c.red() == 255 && c.green() == 0 &&
             c.blue() == 0;
    case ColorTest::Rgb_Green:
      return c.alpha() == 255 && c.red() == 0 && c.green() == 255 &&
             c.blue() == 0;
    case ColorTest::Rgb_Blue:
      return c.alpha() == 255 && c.red() == 0 && c.green() == 0 &&
             c.blue() == 255;
    case ColorTest::Rgb_Transparent:
      return c.alpha() == 0 && c.red() == 0 && c.green() == 0 && c.blue() == 0;
    default:
      return false;
  }
}

TEST_CASE("Can construct a QVariant on the Rust side")
{
  CHECK(can_construct_qvariant(VariantTest::String));
  CHECK(can_construct_qvariant(VariantTest::Int));
  CHECK(can_construct_qvariant(VariantTest::Bool));
}

bool
test_constructed_qvariant(const QVariant& v, VariantTest test)
{
  switch (test) {
    case VariantTest::String:
      return v.toString() == QStringLiteral("Rust string");
    case VariantTest::Int:
      return v.toInt() == 123;
    case VariantTest::Bool:
      return v.toBool() == true;

    default:
      return false;
  }
}

TEST_CASE("Can convert Rust Variant to QVariant")
{
  const auto runTest = [](auto test) {
    return test_constructed_qvariant(
      CxxQt::rustVariantToQVariant(make_variant(test)), test);
  };

  CHECK(runTest(VariantTest::String));
  CHECK(runTest(VariantTest::Int));
  CHECK(runTest(VariantTest::Bool));
}

TEST_CASE("Can read a QColor on the Rust side")
{
  CHECK(can_read_qcolor(QColor(255, 0, 0, 255), ColorTest::Rgb_Red));
  CHECK(can_read_qcolor(QColor(0, 255, 0, 255), ColorTest::Rgb_Green));
  CHECK(can_read_qcolor(QColor(0, 0, 255, 255), ColorTest::Rgb_Blue));
  CHECK(can_read_qcolor(QColor(0, 0, 0, 0), ColorTest::Rgb_Transparent));

  CHECK(can_read_qcolor(QColor(Qt::red), ColorTest::Rgb_Red));
  CHECK(can_read_qcolor(QColor(Qt::green), ColorTest::Rgb_Green));
  CHECK(can_read_qcolor(QColor(Qt::blue), ColorTest::Rgb_Blue));
  CHECK(can_read_qcolor(QColor(Qt::transparent), ColorTest::Rgb_Transparent));
}

TEST_CASE("Can read a QVariant on the Rust side")
{
  CHECK(can_read_qvariant(QVariant::fromValue(QStringLiteral("C++ string")),
                          VariantTest::String));
  CHECK(can_read_qvariant(QVariant::fromValue(8910), VariantTest::Int));
  CHECK(can_read_qvariant(QVariant::fromValue(false), VariantTest::Bool));
}

TEST_CASE("Can construct a QPointF on the Rust side")
{
  const auto p = construct_qpointf();
  CHECK(p.x() == 1.23);
  CHECK(p.y() == 4.56);
}

TEST_CASE("Can read a QPointF on the Rust side")
{
  const auto p = QPointF(1.23, 4.56);
  CHECK(read_qpointf(p));
}

TEST_CASE("Can copy a QPointF on the Rust side")
{
  const auto p = QPointF(1.23, 4.56);
  const auto c = copy_qpointf(p);
  CHECK(c.x() == 1.23);
  CHECK(c.y() == 4.56);
}

TEST_CASE("Can copy a value QPointF on the Rust side")
{
  const auto p = QPointF(1.23, 4.56);
  const auto c = copy_value_qpointf(p);
  CHECK(c.x() == 1.23);
  CHECK(c.y() == 4.56);
}

TEST_CASE("Can construct a QSizeF on the Rust side")
{
  const auto s = construct_qsizef();
  CHECK(s.width() == 1.23);
  CHECK(s.height() == 4.56);
}

TEST_CASE("Can read a QSizeF on the Rust side")
{
  const auto s = QSizeF(1.23, 4.56);
  CHECK(read_qsizef(s));
}

TEST_CASE("Can copy a QSizeF on the Rust side")
{
  const auto s = QSizeF(1.23, 4.56);
  const auto c = copy_qsizef(s);
  CHECK(c.width() == 1.23);
  CHECK(c.height() == 4.56);
}

TEST_CASE("Can copy a value QSizeF on the Rust side")
{
  const auto s = QSizeF(1.23, 4.56);
  const auto c = copy_value_qsizef(s);
  CHECK(c.width() == 1.23);
  CHECK(c.height() == 4.56);
}
