// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QDateTime>
#include <QtTest/QTest>

#include "cxx-qt-gen/qdatetime.cxx.h"

class QDateTimeTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto dt =
      construct_qdatetime(QDate(2022, 1, 1), QTime(1, 2, 3, 4), QTimeZone(0));
    QCOMPARE(dt.date().year(), 2022);
    QCOMPARE(dt.date().month(), 1);
    QCOMPARE(dt.date().day(), 1);
    QCOMPARE(dt.time().hour(), 1);
    QCOMPARE(dt.time().minute(), 2);
    QCOMPARE(dt.time().second(), 3);
    QCOMPARE(dt.time().msec(), 4);
    QCOMPARE(dt.offsetFromUtc(), 0);
  }

  void read()
  {
    QVERIFY(
      read_qdatetime(QDateTime(QDate(2022, 1, 1), QTime(1, 2, 3, 4), Qt::UTC),
                     QDate(2022, 1, 1),
                     QTime(1, 2, 3, 4)));
  }

  void clone()
  {
    const auto dt = QDateTime(QDate(2022, 1, 1), QTime(1, 2, 3, 4));
    const auto c = clone_qdatetime(dt);
    QCOMPARE(c, dt);
  }
};
