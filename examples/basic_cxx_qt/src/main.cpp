// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtCore/QDebug>
#include <QtTest/QSignalSpy>

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/include/sub_object.h"

TEST_CASE("CXX-Qt allows basic interaction between C++ (with Qt) and Rust")
{
  MyObject obj;
  obj.say_hi(QStringLiteral("Hello World!"), 32);

  SubObject sub;

  // Check that an invokable can be called and the return value is correct
  const auto value = obj.double_number(32);
  qInfo() << "Double of 32 is:" << value;
  CHECK(value == 64);

  // Track the signal count of numberChanged, stringChanged, and subChanged
  QSignalSpy numberSpy(&obj, &MyObject::numberChanged);
  QSignalSpy stringSpy(&obj, &MyObject::stringChanged);
  QSignalSpy subSpy(&obj, &MyObject::subChanged);

  // Check the number property
  CHECK(obj.getNumber() == 0);
  CHECK(numberSpy.count() == 0);
  obj.setNumber(16);
  CHECK(numberSpy.count() == 1);
  CHECK(obj.getNumber() == 16);

  // Check the string property
  CHECK(obj.getString() == QString());
  CHECK(stringSpy.count() == 0);
  obj.setString(QStringLiteral("Hello"));
  CHECK(stringSpy.count() == 1);
  CHECK(obj.getString() == QStringLiteral("Hello"));

  // Check the sub property
  CHECK(obj.getSub() == nullptr);
  CHECK(subSpy.count() == 0);
  obj.setSub(&sub);
  CHECK(subSpy.count() == 1);
  CHECK(obj.getSub() == &sub);

  qInfo() << "Number is:" << obj.getNumber() << "String is:" << obj.getString();
}
