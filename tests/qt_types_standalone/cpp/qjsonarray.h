// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QJsonArray>
#include <QtCore/QJsonValue>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qjsonarray.cxx.h"

class QJsonArrayTest : public QObject
{
  Q_OBJECT

private:
  static QJsonArray array()
  {
    return QJsonArray({ QJsonValue(QStringLiteral("first")),
                        QJsonValue(QStringLiteral("second")) });
  }

private Q_SLOTS:
  void construct()
  {
    QCOMPARE(construct_qjsonarray_default(), QJsonArray());
    QVERIFY(construct_qjsonarray_default().empty());

    const auto a =
      construct_qjsonarray(QStringLiteral("first"), QStringLiteral("second"));
    QCOMPARE(a.size(), 2);
    QCOMPARE(a, array());
  }

  void read()
  {
    QVERIFY(read_qjsonarray(
      array(), QStringLiteral("first"), QStringLiteral("second")));
  }

  void clone()
  {
    const auto a = array();
    const auto c = clone_qjsonarray(a);
    QCOMPARE(c, a);
  }

  void iterate()
  {
    const auto a = array();
    QCOMPARE(copy_qjsonarray_by_iterating(a), a);
  }

  void handleChange() { QVERIFY(can_handle_qjsonarray_change()); }
};
