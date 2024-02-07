// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QSizeF>
#include <QtTest/QTest>

#include "cxx-qt-gen/qsizef.cxx.h"

class QSizeFTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto s = construct_qsizef();
    QCOMPARE(s.width(), 1.23);
    QCOMPARE(s.height(), 4.56);
  }

  void read()
  {
    const auto s = QSizeF(1.23, 4.56);
    QVERIFY(read_qsizef(s));
  }

  void clone()
  {
    const auto s = QSizeF(1.23, 4.56);
    const auto c = clone_qsizef(s);
    QCOMPARE(c.width(), 1.23);
    QCOMPARE(c.height(), 4.56);
  }
};
