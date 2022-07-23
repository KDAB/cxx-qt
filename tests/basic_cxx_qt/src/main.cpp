// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QDebug>
#include <QSignalSpy>
#include <QTimer>

#define DOCTEST_CONFIG_IMPLEMENT
#include "doctest.h"

#include "my_data.cxxqt.h"
#include "my_object.cxxqt.h"
#include "my_types.cxxqt.h"
#include "sub_object.cxxqt.h"

int
main(int argc, char** argv)
{
  QCoreApplication app{ argc, argv };

  QTimer::singleShot(0, [&]() {
    doctest::Context context;
    context.applyCommandLine(argc, argv);
    const auto result = context.run();
    app.exit(result);
  });

  return app.exec();
}

TEST_CASE("CXX-Qt allows basic interaction between C++ (with Qt) and Rust")
{
  cxx_qt::my_object::MyObject obj;

  obj.sayHi(QStringLiteral("Hello World!"), 32);

  cxx_qt::sub_object::SubObject sub;

  // Check that an invokable can be called and the return value is correct
  const auto value = obj.doubleNumber(32);
  qInfo() << "Double of 32 is:" << value;
  CHECK(value == 64);

  // Track the signal count of numberChanged, stringChanged, and subChanged
  QSignalSpy numberSpy(&obj, &cxx_qt::my_object::MyObject::numberChanged);
  QSignalSpy stringSpy(&obj, &cxx_qt::my_object::MyObject::stringChanged);
  QSignalSpy subSpy(&obj, &cxx_qt::my_object::MyObject::subChanged);
  QSignalSpy subNumberSpy(&sub, &cxx_qt::sub_object::SubObject::numberChanged);

  // Check the number property
  CHECK(obj.getNumber() == 0);
  CHECK(numberSpy.count() == 0);
  obj.setNumber(16);
  CHECK(numberSpy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(numberSpy.count() == 1);
  CHECK(obj.getNumber() == 16);

  // Check the string property
  CHECK(obj.getString() == QString());
  CHECK(stringSpy.count() == 0);
  obj.setString(QStringLiteral("Hello"));
  CHECK(stringSpy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(stringSpy.count() == 1);
  CHECK(obj.getString() == QStringLiteral("Hello"));

  // Check the double number self
  CHECK(obj.getNumber() == 16);
  CHECK(numberSpy.count() == 1);
  obj.doubleNumberSelf();
  CHECK(obj.getNumber() == 32);
  CHECK(numberSpy.count() == 1);
  QCoreApplication::processEvents();
  CHECK(numberSpy.count() == 2);

  // Check the sub property
  CHECK(obj.getSub() == nullptr);
  CHECK(subSpy.count() == 0);
  obj.setSub(&sub);
  CHECK(subSpy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(subSpy.count() == 1);
  CHECK(obj.getSub() == &sub);

  // Check the sub increment number self
  sub.setNumber(1);
  CHECK(sub.getNumber() == 1);
  CHECK(subNumberSpy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(subNumberSpy.count() == 1);
  sub.incrementNumberSelf();
  CHECK(sub.getNumber() == 2);
  CHECK(subNumberSpy.count() == 1);
  QCoreApplication::processEvents();
  CHECK(subNumberSpy.count() == 2);

  // Check the double number sub
  CHECK(sub.getNumber() == 2);
  CHECK(subNumberSpy.count() == 2);
  obj.doubleNumberSub(&sub);
  CHECK(sub.getNumber() == 4);
  CHECK(subNumberSpy.count() == 2);
  QCoreApplication::processEvents();
  CHECK(subNumberSpy.count() == 3);

  qInfo() << "Number is:" << obj.getNumber() << "String is:" << obj.getString();
}

TEST_CASE("CXX-Qt allows basic interaction between C++ (with Qt) and Rust "
          "using Serde")
{
  cxx_qt::my_data::MyData data;

  // Track the signal count of numberChanged, stringChanged, and subChanged
  QSignalSpy numberSpy(&data, &cxx_qt::my_data::MyData::numberChanged);
  QSignalSpy stringSpy(&data, &cxx_qt::my_data::MyData::stringChanged);

  // Check that initial value of the deserialised data
  CHECK(data.getNumber() == 4);
  CHECK(data.getString() == QStringLiteral("Hello World!"));

  // Check the number changed property
  CHECK(numberSpy.count() == 0);
  data.setNumber(16);
  CHECK(numberSpy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(numberSpy.count() == 1);
  CHECK(data.getNumber() == 16);

  // Check the string property
  CHECK(stringSpy.count() == 0);
  data.setString(QStringLiteral("Hello"));
  CHECK(stringSpy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(stringSpy.count() == 1);
  CHECK(data.getString() == QStringLiteral("Hello"));

  // Check that initial value of the deserialised data
  CHECK(data.asJsonStr() ==
        QStringLiteral("{\"number\":16,\"string\":\"Hello\"}"));
}

TEST_CASE("CXX-Qt allows Rust code to handle an update request")
{
  cxx_qt::my_object::MyObject obj;
  CHECK(obj.updateCallCount() == 0);
  obj.requestUpdateTest();
  CHECK(obj.updateCallCount() == 0);
  QCoreApplication::processEvents();
  CHECK(obj.updateCallCount() == 1);
}

TEST_CASE("CXX-Qt allows Rust code to handle multiple update requests")
{
  cxx_qt::my_object::MyObject obj;
  CHECK(obj.updateCallCount() == 0);
  obj.requestUpdateTest();
  obj.requestUpdateTest();
  CHECK(obj.updateCallCount() == 0);
  QCoreApplication::processEvents();
  CHECK(obj.updateCallCount() == 2);
}

TEST_CASE(
  "CXX-Qt allows Rust code to handle update requests in multiple threads")
{
  cxx_qt::my_object::MyObject obj;
  CHECK(obj.updateCallCount() == 0);
  obj.requestUpdateTestMultiThread();
  CHECK(obj.updateCallCount() == 0);
  QCoreApplication::processEvents();
  CHECK(obj.updateCallCount() == 100);
}

TEST_CASE("CXX-Qt types are exposed to C++ correctly")
{
  cxx_qt::my_types::MyTypes types;

  QSignalSpy booleanSpy(&types, &cxx_qt::my_types::MyTypes::booleanChanged);
  QSignalSpy float32Spy(&types, &cxx_qt::my_types::MyTypes::float32Changed);
  QSignalSpy float64Spy(&types, &cxx_qt::my_types::MyTypes::float64Changed);
  QSignalSpy int8Spy(&types, &cxx_qt::my_types::MyTypes::int8Changed);
  QSignalSpy int16Spy(&types, &cxx_qt::my_types::MyTypes::int16Changed);
  QSignalSpy int32Spy(&types, &cxx_qt::my_types::MyTypes::int32Changed);
  QSignalSpy uint8Spy(&types, &cxx_qt::my_types::MyTypes::uint8Changed);
  QSignalSpy uint16Spy(&types, &cxx_qt::my_types::MyTypes::uint16Changed);
  QSignalSpy uint32Spy(&types, &cxx_qt::my_types::MyTypes::uint32Changed);

  CHECK(types.getBoolean() == false);
  CHECK(booleanSpy.count() == 0);
  types.setBoolean(true);
  CHECK(booleanSpy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(booleanSpy.count() == 1);
  CHECK(types.getBoolean() == true);

  CHECK(types.getFloat32() == 0.0);
  CHECK(float32Spy.count() == 0);
  types.setFloat32(0.33f);
  CHECK(float32Spy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(float32Spy.count() == 1);
  CHECK(types.getFloat32() == 0.33f);

  CHECK(types.getFloat64() == 0.0);
  CHECK(float64Spy.count() == 0);
  types.setFloat64(0.33);
  CHECK(float64Spy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(float64Spy.count() == 1);
  CHECK(types.getFloat64() == 0.33);

  CHECK(types.getInt8() == 0);
  CHECK(int8Spy.count() == 0);
  types.setInt8(4);
  CHECK(int8Spy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(int8Spy.count() == 1);
  CHECK(types.getInt8() == 4);

  CHECK(types.getInt16() == 0);
  CHECK(int16Spy.count() == 0);
  types.setInt16(4);
  CHECK(int16Spy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(int16Spy.count() == 1);
  CHECK(types.getInt16() == 4);

  CHECK(types.getInt32() == 0);
  CHECK(int32Spy.count() == 0);
  types.setInt32(4);
  CHECK(int32Spy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(int32Spy.count() == 1);
  CHECK(types.getInt32() == 4);

  CHECK(types.getUint8() == 0);
  CHECK(uint8Spy.count() == 0);
  types.setUint8(4);
  CHECK(uint8Spy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(uint8Spy.count() == 1);
  CHECK(types.getUint8() == 4);

  CHECK(types.getUint16() == 0);
  CHECK(uint16Spy.count() == 0);
  types.setUint16(4);
  CHECK(uint16Spy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(uint16Spy.count() == 1);
  CHECK(types.getUint16() == 4);

  CHECK(types.getUint32() == 0);
  CHECK(uint32Spy.count() == 0);
  types.setUint32(4);
  CHECK(uint32Spy.count() == 0);
  QCoreApplication::processEvents();
  CHECK(uint32Spy.count() == 1);
  CHECK(types.getUint32() == 4);
}
