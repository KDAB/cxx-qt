// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QLine>
#include <QtTest/QTest>

#include "qt_types_standalone/qline.cxx.h"

class QLineTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto m = construct_qline();
    QCOMPARE(m.x1(), 1);
    QCOMPARE(m.y1(), 2);
    QCOMPARE(m.x2(), 3);
    QCOMPARE(m.y2(), 4);
  }

  void read()
  {
    const auto m = QLine(QPoint(1, 2), QPoint(3, 4));
    QVERIFY(read_qline(m));
  }

  void clone()
  {
    const auto m = QLine(QPoint(1, 2), QPoint(3, 4));
    const auto c = clone_qline(m);
    QCOMPARE(c.x1(), 1);
    QCOMPARE(c.y1(), 2);
    QCOMPARE(c.x2(), 3);
    QCOMPARE(c.y2(), 4);
  }
};
