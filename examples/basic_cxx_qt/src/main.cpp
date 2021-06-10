#include <QDebug>

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "cxx-qt-gen/include/my_object.h"

TEST_CASE("CXX-Qt allows basic interaction between C++ (with Qt) and Rust")
{
  MyObject obj;
  obj.say_hi(QStringLiteral("Hello World!"), 32);

  const auto value = obj.double_number(32);
  qInfo() << "Double of 32 is:" << value;
  CHECK(value == 64);
}
