// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QVector4D>
#include <QtTest/QTest>

#include "qt_types_standalone/qvector3d.cxx.h"

class QVector3DTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto v = construct_qvector3d();
    QCOMPARE(v.x(), 1.23f);
    QCOMPARE(v.y(), 4.56f);
    QCOMPARE(v.z(), 7.89f);
  }

  void read()
  {
    const auto v = QVector3D(1.23f, 4.56f, 7.89f);
    QVERIFY(read_qvector3d(v));
  }

  void clone()
  {
    const auto v = QVector3D(1.23f, 4.56f, 7.89f);
    const auto c = clone_qvector3d(v);
    QCOMPARE(c.x(), 1.23f);
    QCOMPARE(c.y(), 4.56f);
    QCOMPARE(c.z(), 7.89f);
  }
};
