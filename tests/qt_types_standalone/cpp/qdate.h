// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QDate>
#include <QtTest/QTest>

#include "qt-types-standalone/qdate_cxx.cxx.h"

class QDateTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto d = construct_qdate();
    QCOMPARE(d.year(), 2022);
    QCOMPARE(d.month(), 1);
    QCOMPARE(d.day(), 1);
  }

  void read()
  {
    const auto d = QDate(2022, 1, 1);
    QVERIFY(read_qdate(d));
  }

  void clone()
  {
    const auto d = QDate(2022, 1, 1);
    const auto c = clone_qdate(d);
    QCOMPARE(c.year(), 2022);
    QCOMPARE(c.month(), 1);
    QCOMPARE(c.day(), 1);
  }
};
