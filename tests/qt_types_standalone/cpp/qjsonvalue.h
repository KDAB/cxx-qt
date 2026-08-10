// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QJsonArray>
#include <QtCore/QJsonObject>
#include <QtCore/QJsonValue>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qjsonvalue.cxx.h"

class QJsonValueTest : public QObject
{
  Q_OBJECT

private:
  static QJsonArray array()
  {
    return QJsonArray({ QJsonValue(QStringLiteral("first")),
                        QJsonValue(QStringLiteral("second")) });
  }

  static QJsonObject object()
  {
    return QJsonObject(
      { { QStringLiteral("key"), QJsonValue(QStringLiteral("value")) } });
  }

private Q_SLOTS:
  void construct()
  {
    QCOMPARE(construct_qjsonvalue_default(), QJsonValue(QJsonValue::Null));
    QCOMPARE(construct_qjsonvalue_bool(true), QJsonValue(true));
    QCOMPARE(construct_qjsonvalue_double(1.23), QJsonValue(1.23));
    QCOMPARE(construct_qjsonvalue_int(123), QJsonValue(qint64(123)));
    QCOMPARE(construct_qjsonvalue_string(QStringLiteral("Rust string")),
             QJsonValue(QStringLiteral("Rust string")));
    QCOMPARE(construct_qjsonvalue_array(array()), QJsonValue(array()));
    QCOMPARE(construct_qjsonvalue_object(object()), QJsonValue(object()));
  }

  void read()
  {
    QVERIFY(read_qjsonvalue_bool(QJsonValue(true), true));
    QVERIFY(read_qjsonvalue_double(QJsonValue(4.56), 4.56));
    QVERIFY(read_qjsonvalue_int(QJsonValue(456), 456));
    QVERIFY(read_qjsonvalue_string(QJsonValue(QStringLiteral("C++ string")),
                                   QStringLiteral("C++ string")));
    QVERIFY(read_qjsonvalue_array(QJsonValue(array()), array()));
    QVERIFY(read_qjsonvalue_object(QJsonValue(object()), object()));
    QVERIFY(read_qjsonvalue_null(QJsonValue(QJsonValue::Null)));
    QVERIFY(read_qjsonvalue_undefined(QJsonValue(QJsonValue::Undefined)));
  }

  void clone()
  {
    const auto v = QJsonValue(object());
    const auto c = clone_qjsonvalue(v);
    QCOMPARE(c, v);
  }

  void handleChange() { QVERIFY(can_handle_qjsonvalue_change()); }
};
