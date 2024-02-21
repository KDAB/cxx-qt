// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QPointF>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qpointf.cxx.h"

class QPointFTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto p = construct_qpointf();
    QCOMPARE(p.x(), 1.23);
    QCOMPARE(p.y(), 4.56);
  }

  void read()
  {
    const auto p = QPointF(1.23, 4.56);
    QVERIFY(read_qpointf(p));
  }

  void clone()
  {
    const auto p = QPointF(1.23, 4.56);
    const auto c = clone_qpointf(p);
    QCOMPARE(c.x(), 1.23);
    QCOMPARE(c.y(), 4.56);
  }
};
