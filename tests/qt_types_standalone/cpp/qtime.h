// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QTime>
#include <QtTest/QTest>

#include "qt_types_standalone/qtime.cxx.h"

class QTimeTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto t = construct_qtime();
    QCOMPARE(t.hour(), 1);
    QCOMPARE(t.minute(), 2);
    QCOMPARE(t.second(), 3);
    QCOMPARE(t.msec(), 4);
  }

  void read()
  {
    const auto t = QTime(1, 2, 3, 4);
    QVERIFY(read_qtime(t));
  }

  void clone()
  {
    const auto t = QTime(1, 2, 3, 4);
    const auto c = clone_qtime(t);
    QCOMPARE(c.hour(), 1);
    QCOMPARE(c.minute(), 2);
    QCOMPARE(c.second(), 3);
    QCOMPARE(c.msec(), 4);
  }
};
