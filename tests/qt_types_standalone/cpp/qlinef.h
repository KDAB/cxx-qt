// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QLineF>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qlinef.cxx.h"

class QLineFTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto m = construct_qlinef();
    QCOMPARE(m.x1(), 1.0);
    QCOMPARE(m.y1(), 2.0);
    QCOMPARE(m.x2(), 3.0);
    QCOMPARE(m.y2(), 4.0);
  }

  void read()
  {
    const auto m = QLineF(QPointF(1.0, 2.0), QPointF(3.0, 4.0));
    QVERIFY(read_qlinef(m));
  }

  void clone()
  {
    const auto m = QLineF(QPointF(1.0, 2.0), QPointF(3.0, 4.0));
    const auto c = clone_qlinef(m);
    QCOMPARE(c.x1(), 1.0);
    QCOMPARE(c.y1(), 2.0);
    QCOMPARE(c.x2(), 3.0);
    QCOMPARE(c.y2(), 4.0);
  }
};
