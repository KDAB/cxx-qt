// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGlobal>

// Qt 5 has a different QList<T>
#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))

#include <QtCore/QList>
#include <QtTest/QTest>

#include "cxx-qt-gen/qt_5_list_cxx.cxx.h"

class Qt5ListTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto v = construct_qt5list_i32();
    QVERIFY(v.contains(1));
    QVERIFY(!v.contains(2));
    QVERIFY(v.contains(3));
    QCOMPARE(v.size(), 4);
  }

  void read()
  {
    auto v = QList<::std::int32_t>();
    v.append(1);
    v.append(1);
    v.append(3);
    v.append(3);
    QVERIFY(read_qt5list_i32(v));
  }

  void clone()
  {
    auto v = QList<::std::int32_t>();
    v.append(1);
    v.append(1);
    v.append(3);
    v.append(3);
    const auto c = clone_qt5list_i32(v);
    QVERIFY(c.contains(1));
    QVERIFY(!c.contains(2));
    QVERIFY(c.contains(3));
    QCOMPARE(c.size(), 4);
  }
};

#endif
