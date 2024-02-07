// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QVector4D>
#include <QtTest/QTest>

#include "cxx-qt-gen/qvector4d.cxx.h"

class QVector4DTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto v = construct_qvector4d();
    QCOMPARE(v.w(), 1.47f);
    QCOMPARE(v.x(), 1.23f);
    QCOMPARE(v.y(), 4.56f);
    QCOMPARE(v.z(), 7.89f);
  }

  void read()
  {
    const auto v = QVector4D(1.23f, 4.56f, 7.89f, 1.47f);
    QVERIFY(read_qvector4d(v));
  }

  void clone()
  {
    const auto v = QVector4D(1.23f, 4.56f, 7.89f, 1.47f);
    const auto c = clone_qvector4d(v);
    QCOMPARE(c.w(), 1.47f);
    QCOMPARE(c.x(), 1.23f);
    QCOMPARE(c.y(), 4.56f);
    QCOMPARE(c.z(), 7.89f);
  }
};
