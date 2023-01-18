// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QMargins>
#include <QtTest/QTest>

#include "cxx-qt-gen/qmargins_cxx.cxx.h"

class QMarginsTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto m = construct_qmargins();
    QCOMPARE(m.left(), 1);
    QCOMPARE(m.top(), 2);
    QCOMPARE(m.right(), 3);
    QCOMPARE(m.bottom(), 4);
  }

  void read()
  {
    const auto m = QMargins(1, 2, 3, 4);
    QVERIFY(read_qmargins(m));
  }

  void clone()
  {
    const auto m = QMargins(1, 2, 3, 4);
    const auto c = clone_qmargins(m);
    QCOMPARE(c.left(), 1);
    QCOMPARE(c.top(), 2);
    QCOMPARE(c.right(), 3);
    QCOMPARE(c.bottom(), 4);
  }
};
