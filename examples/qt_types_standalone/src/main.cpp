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