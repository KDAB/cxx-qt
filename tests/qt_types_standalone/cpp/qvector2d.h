// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QVector2D>
#include <QtTest/QTest>

#include "qt_types_standalone/qvector2d.cxx.h"

class QVector2DTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto v = construct_qvector2d();
    QCOMPARE(v.x(), 1.23f);
    QCOMPARE(v.y(), 4.56f);
  }

  void read()
  {
    const auto v = QVector2D(1.23f, 4.56f);
    QVERIFY(read_qvector2d(v));
  }

  void clone()
  {
    const auto v = QVector2D(1.23f, 4.56f);
    const auto c = clone_qvector2d(v);
    QCOMPARE(c.x(), 1.23f);
    QCOMPARE(c.y(), 4.56f);
  }
};
