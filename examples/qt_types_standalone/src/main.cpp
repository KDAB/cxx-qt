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

TEST_CASE("Can construct a QVariant on the Rust side")
{
  CHECK(can_construct_qvariant(VariantTest::Bool));
  CHECK(can_construct_qvariant(VariantTest::F32));
  CHECK(can_construct_qvariant(VariantTest::F64));
  CHECK(can_construct_qvariant(VariantTest::I8));
  CHECK(can_construct_qvariant(VariantTest::I16));
  CHECK(can_construct_qvariant(VariantTest::I32));
  CHECK(can_construct_qvariant(VariantTest::String));
  CHECK(can_construct_qvariant(VariantTest::U8));
  CHECK(can_construct_qvariant(VariantTest::U16));
  CHECK(can_construct_qvariant(VariantTest::U32));
}

bool
test_constructed_qvariant(const QVariant& v, VariantTest test)
{
  switch (test) {
    case VariantTest::Bool:
      return v.toBool() == true;
    case VariantTest::F32:
      return qFuzzyCompare(v.value<float>(), 1.23f);
    case VariantTest::F64:
      return qFuzzyCompare(v.value<double>(), 1.23);
    case VariantTest::I8:
      return v.value<qint8>() == 12;
    case VariantTest::I16:
      return v.value<qint16>() == 123;
    case VariantTest::I32:
      return v.value<qint32>() == 123;
    case VariantTest::String:
      return v.toString() == QStringLiteral("Rust string");
    case VariantTest::U8:
      return v.value<quint8>() == 12;
    case VariantTest::U16:
      return v.value<quint16>() == 123;
    case VariantTest::U32:
      return v.value<quint32>() == 123;

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

  CHECK(runTest(VariantTest::Bool));
  CHECK(runTest(VariantTest::F32));
  CHECK(runTest(VariantTest::F64));
  CHECK(runTest(VariantTest::I8));
  CHECK(runTest(VariantTest::I16));
  CHECK(runTest(VariantTest::I32));
  CHECK(runTest(VariantTest::String));
  CHECK(runTest(VariantTest::U8));
  CHECK(runTest(VariantTest::U16));
  CHECK(runTest(VariantTest::U32));
}

TEST_CASE("Can read a QVariant on the Rust side")
{
  CHECK(can_read_qvariant(QVariant::fromValue(false), VariantTest::Bool));
  CHECK(can_read_qvariant(QVariant::fromValue<float>(89.1), VariantTest::F32));
  CHECK(can_read_qvariant(QVariant::fromValue<double>(89.1), VariantTest::F64));
  CHECK(can_read_qvariant(QVariant::fromValue<qint8>(89), VariantTest::I8));
  CHECK(can_read_qvariant(QVariant::fromValue<qint8>(89), VariantTest::I8));
  CHECK(can_read_qvariant(QVariant::fromValue<qint16>(8910), VariantTest::I16));
  CHECK(can_read_qvariant(QVariant::fromValue(8910), VariantTest::I32));
  CHECK(can_read_qvariant(QVariant::fromValue(QStringLiteral("C++ string")),
                          VariantTest::String));
  CHECK(can_read_qvariant(QVariant::fromValue<quint8>(89), VariantTest::U8));
  CHECK(
    can_read_qvariant(QVariant::fromValue<quint16>(8910), VariantTest::U16));
  CHECK(
    can_read_qvariant(QVariant::fromValue<quint32>(8910), VariantTest::U32));
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
