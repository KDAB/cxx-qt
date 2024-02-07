// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QObject>
#include <QtTest/QSignalSpy>
#include <QtTest/QTest>
#include <qobjectdefs.h>

#include "cxx-qt-gen/qmetaobjectconnection.cxx.h"

class MyObject : public QObject
{
  Q_OBJECT
public:
  void trigger() { Q_EMIT mySignal(); }

Q_SIGNALS:
  void mySignal();
  void anotherSignal();
};

class QMetaObjectConnectionTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void drop()
  {
    auto obj = MyObject();
    auto handle = QObject::connect(
      &obj, &MyObject::mySignal, &obj, &MyObject::anotherSignal);

    QSignalSpy mySignalSpy(&obj, &MyObject::mySignal);
    QSignalSpy anotherSignalSpy(&obj, &MyObject::anotherSignal);
    QCOMPARE(mySignalSpy.count(), 0);
    QCOMPARE(anotherSignalSpy.count(), 0);

    obj.trigger();
    QCOMPARE(mySignalSpy.count(), 1);
    QCOMPARE(anotherSignalSpy.count(), 1);

    qmetaobjectconnection_drop(std::move(handle));

    obj.trigger();
    QCOMPARE(mySignalSpy.count(), 2);
    QCOMPARE(anotherSignalSpy.count(), 1);
  }

  void release()
  {
    auto obj = MyObject();
    auto handle = QObject::connect(
      &obj, &MyObject::mySignal, &obj, &MyObject::anotherSignal);

    QSignalSpy mySignalSpy(&obj, &MyObject::mySignal);
    QSignalSpy anotherSignalSpy(&obj, &MyObject::anotherSignal);
    QCOMPARE(mySignalSpy.count(), 0);
    QCOMPARE(anotherSignalSpy.count(), 0);

    obj.trigger();
    QCOMPARE(mySignalSpy.count(), 1);
    QCOMPARE(anotherSignalSpy.count(), 1);

    qmetaobjectconnection_release(std::move(handle));

    obj.trigger();
    QCOMPARE(mySignalSpy.count(), 2);
    QCOMPARE(anotherSignalSpy.count(), 2);
  }
};
