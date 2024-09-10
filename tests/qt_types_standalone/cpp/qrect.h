// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QRect>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qrect.cxx.h"

class QRectTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto r = construct_qrect();
    QCOMPARE(r.x(), 1);
    QCOMPARE(r.y(), 4);
    QCOMPARE(r.width(), 2);
    QCOMPARE(r.height(), 8);
  }

  void read()
  {
    const auto r = QRect(1, 4, 2, 8);
    QVERIFY(read_qrect(r));
  }

  void clone()
  {
    const auto r = QRect(1, 4, 2, 8);
    const auto c = clone_qrect(r);
    QCOMPARE(c.x(), 1);
    QCOMPARE(c.y(), 4);
    QCOMPARE(c.width(), 2);
    QCOMPARE(c.height(), 8);
  }
};
