// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QRectF>
#include <QtTest/QTest>

#include "qt-types-standalone/qrectf_cxx.cxx.h"

class QRectFTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto r = construct_qrectf();
    QCOMPARE(r.x(), 1.23);
    QCOMPARE(r.y(), 4.56);
    QCOMPARE(r.width(), 2.46);
    QCOMPARE(r.height(), 9.12);
  }

  void read()
  {
    const auto r = QRectF(1.23, 4.56, 2.46, 9.12);
    QVERIFY(read_qrectf(r));
  }

  void clone()
  {
    const auto r = QRectF(1.23, 4.56, 2.46, 9.12);
    const auto c = clone_qrectf(r);
    QCOMPARE(c.x(), 1.23);
    QCOMPARE(c.y(), 4.56);
    QCOMPARE(c.width(), 2.46);
    QCOMPARE(c.height(), 9.12);
  }
};
