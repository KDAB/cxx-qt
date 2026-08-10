// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QJsonObject>
#include <QtCore/QJsonValue>
#include <QtCore/QStringList>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qjsonobject.cxx.h"

class QJsonObjectTest : public QObject
{
  Q_OBJECT

private:
  static QJsonObject object()
  {
    return QJsonObject(
      { { QStringLiteral("key"), QJsonValue(QStringLiteral("value")) } });
  }

private Q_SLOTS:
  void construct()
  {
    QCOMPARE(construct_qjsonobject_default(), QJsonObject());

    const auto o =
      construct_qjsonobject(QStringLiteral("key"), QStringLiteral("value"));
    QCOMPARE(o.size(), 1);
    QCOMPARE(o, object());
  }

  void read()
  {
    QVERIFY(read_qjsonobject(
      object(), QStringLiteral("key"), QStringLiteral("value")));
  }

  void clone()
  {
    const auto o = object();
    const auto c = clone_qjsonobject(o);
    QCOMPARE(c, o);
  }

  void keys()
  {
    QCOMPARE(keys_qjsonobject(object()), QStringList(QStringLiteral("key")));
  }

  void handleChange() { QVERIFY(can_handle_qjsonobject_change()); }
};
