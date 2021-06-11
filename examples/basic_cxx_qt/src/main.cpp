#include <QtCore/QDebug>
#include <QtTest/QSignalSpy>

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "cxx-qt-gen/include/my_object.h"

TEST_CASE("CXX-Qt allows basic interaction between C++ (with Qt) and Rust")
{
  MyObject obj;
  obj.say_hi(QStringLiteral("Hello World!"), 32);

  // Check that an invokable can be called and the return value is correct
  const auto value = obj.double_number(32);
  qInfo() << "Double of 32 is:" << value;
  CHECK(value == 64);

  // Track the signal count of numberChanged and stringChanged
  QSignalSpy numberSpy(&obj, &MyObject::numberChanged);
  QSignalSpy stringSpy(&obj, &MyObject::stringChanged);

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

  qInfo() << "Number is:" << obj.getNumber() << "String is:" << obj.getString();
}
