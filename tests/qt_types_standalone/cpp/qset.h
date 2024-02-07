// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QSet>
#include <QtTest/QTest>

#include "cxx-qt-gen/qset.cxx.h"

class QSetTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto s = construct_qset_i32();
    QVERIFY(s.contains(1));
    QVERIFY(!s.contains(2));
    QVERIFY(s.contains(3));
    QCOMPARE(s.size(), 2);
  }

  void read()
  {
    auto s = QSet<::std::int32_t>();
    s.insert(1);
    s.insert(1);
    s.insert(3);
    s.insert(3);
    QVERIFY(read_qset_i32(s));
  }

  void clone()
  {
    auto s = QSet<::std::int32_t>();
    s.insert(1);
    s.insert(1);
    s.insert(3);
    s.insert(3);
    const auto c = clone_qset_i32(s);
    QVERIFY(c.contains(1));
    QVERIFY(!c.contains(2));
    QVERIFY(c.contains(3));
    QCOMPARE(c.size(), 2);
  }
};
