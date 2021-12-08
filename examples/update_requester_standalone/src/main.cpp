// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <QtCore/QCoreApplication>
#include <QtCore/QDebug>
#include <QtCore/QTimer>
#include <QtTest/QSignalSpy>

#define DOCTEST_CONFIG_IMPLEMENT
#include "doctest.h"

#include "TestObject.h"
#include "cxx-qt-gen/include/lib.rs.h"

static TestObject* testObject = nullptr;

int
main(int argc, char** argv)
{
  QCoreApplication app{ argc, argv };

  QTimer::singleShot(0, [&]() {
    testObject = new TestObject();
    init_rust(testObject);

    doctest::Context context;
    context.applyCommandLine(argc, argv);
    const auto result = context.run();

    // This makes sure that requesting an update on a deleted object
    // does not cause a crash.
    delete testObject;
    request_update();

    app.exit(result);
  });

  return app.exec();
}

TEST_CASE("An update can be requested")
{
  Q_ASSERT(testObject != nullptr);

  QSignalSpy updatedSpy(testObject, &TestObject::updateStateRequested);
  request_update();
  CHECK(updatedSpy.wait());
  CHECK_EQ(updatedSpy.count(), 1);
}

TEST_CASE("Multiple state updates are possible")
{
  Q_ASSERT(testObject != nullptr);

  QSignalSpy updatedSpy(testObject, &TestObject::updateStateRequested);
  request_update();
  request_update();
  CHECK(updatedSpy.wait());
  CHECK_EQ(updatedSpy.count(), 2);

  updatedSpy.clear();
  request_update();
  request_update();
  CHECK(updatedSpy.wait());
  CHECK_EQ(updatedSpy.count(), 2);
}

TEST_CASE("Requests across multiple threads are supported")
{
  Q_ASSERT(testObject != nullptr);

  QSignalSpy updatedSpy(testObject, &TestObject::updateStateRequested);
  request_on_multiple_threads();
  CHECK(updatedSpy.wait());
  CHECK_EQ(updatedSpy.count(), 100);
}
