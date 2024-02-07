// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QPoint>
#include <QtTest/QTest>

#include "cxx-qt-gen/qpoint.cxx.h"

class QPointTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto p = construct_qpoint();
    QCOMPARE(p.x(), 2);
    QCOMPARE(p.y(), 4);
  }

  void read()
  {
    const auto p = QPoint(2, 4);
    QVERIFY(read_qpoint(p));
  }

  void clone()
  {
    const auto p = QPoint(2, 4);
    const auto c = clone_qpoint(p);
    QCOMPARE(c.x(), 2);
    QCOMPARE(c.y(), 4);
  }
};
