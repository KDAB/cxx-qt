// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QVector>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qvector.cxx.h"

class QVectorTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto v = construct_qvector_i32();
    QVERIFY(v.contains(1));
    QVERIFY(!v.contains(2));
    QVERIFY(v.contains(3));
    QCOMPARE(v.size(), 4);
  }

  void read()
  {
    auto v = QVector<::std::int32_t>();
    v.append(1);
    v.append(1);
    v.append(3);
    v.append(3);
    QVERIFY(read_qvector_i32(v));
  }

  void clone()
  {
    auto v = QVector<::std::int32_t>();
    v.append(1);
    v.append(1);
    v.append(3);
    v.append(3);
    const auto c = clone_qvector_i32(v);
    QVERIFY(c.contains(1));
    QVERIFY(!c.contains(2));
    QVERIFY(c.contains(3));
    QCOMPARE(c.size(), 4);
  }
};
