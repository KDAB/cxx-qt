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

#include "cxx-qt-gen/include/my_data.h"
#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/include/sub_object.h"

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
  CHECK(numberSpy.count() == 1);
  CHECK(obj.getNumber() == 16);

  // Check the string property
  CHECK(obj.getString() == QString());
  CHECK(stringSpy.count() == 0);
  obj.setString(QStringLiteral("Hello"));
  CHECK(stringSpy.count() == 1);
  CHECK(obj.getString() == QStringLiteral("Hello"));

  // Check the double number self
  CHECK(obj.getNumber() == 16);
  CHECK(numberSpy.count() == 1);
  obj.doubleNumberSelf();
  CHECK(obj.getNumber() == 32);
  CHECK(numberSpy.count() == 2);

  // Check the sub property
  CHECK(obj.getSub() == nullptr);
  CHECK(subSpy.count() == 0);
  obj.setSub(&sub);
  CHECK(subSpy.count() == 1);
  CHECK(obj.getSub() == &sub);

  // Check the sub increment number self
  sub.setNumber(1);
  CHECK(sub.getNumber() == 1);
  CHECK(subNumberSpy.count() == 1);
  sub.incrementNumberSelf();
  CHECK(sub.getNumber() == 2);
  CHECK(subNumberSpy.count() == 2);

  // Check the double number sub
  CHECK(sub.getNumber() == 2);
  CHECK(subNumberSpy.count() == 2);
  obj.doubleNumberSub(&sub);
  CHECK(sub.getNumber() == 4);
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
  CHECK(numberSpy.count() == 1);
  CHECK(data.getNumber() == 16);

  // Check the string property
  CHECK(stringSpy.count() == 0);
  data.setString(QStringLiteral("Hello"));
  CHECK(stringSpy.count() == 1);
  CHECK(data.getString() == QStringLiteral("Hello"));

  // Check that initial value of the deserialised data
  CHECK(data.asJsonStr() ==
        QStringLiteral("{\"number\":16,\"string\":\"Hello\"}"));
}

class UpdateEventCatcher : public QObject
{
  Q_OBJECT

public:
  bool eventFilter(QObject* object, QEvent* event) override
  {
    if (event->type() == CxxQObject::UpdateStateEvent) {
      Q_EMIT receivedUpdateRequest();
      return true;
    }

    return false;
  }

Q_SIGNALS:
  void receivedUpdateRequest();
};

TEST_CASE("CXX-Qt allows Rust code to request an update")
{
  UpdateEventCatcher catcher;
  QSignalSpy updateSpy(&catcher, &UpdateEventCatcher::receivedUpdateRequest);

  cxx_qt::my_object::MyObject obj;
  obj.installEventFilter(&catcher);

  obj.requestUpdate();
  CHECK(updateSpy.wait());
  CHECK(updateSpy.count() == 1);
}

TEST_CASE("CXX-Qt allows Rust code to handle an update request")
{
  cxx_qt::my_object::MyObject obj;
  CHECK(obj.updateCallCount() == 0);
  obj.requestUpdate();
  QCoreApplication::processEvents();
  CHECK(obj.updateCallCount() == 1);
}

#include "main.moc"
