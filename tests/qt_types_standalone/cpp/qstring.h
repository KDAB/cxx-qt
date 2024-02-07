// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QString>
#include <QtTest/QTest>

#include "cxx-qt-gen/qstring.cxx.h"

class QStringTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    QFETCH(bool, slice);
    const auto s = construct_qstring(slice);
    QCOMPARE(s, QStringLiteral("String constructed by Rust"));
  }

  void construct_data()
  {
    QTest::addColumn<bool>("slice");

    QTest::newRow("From &str") << true;
    QTest::newRow("From String") << false;
  }

  void read()
  {
    const auto s = QStringLiteral("String constructed by C++");
    QVERIFY(read_qstring(s));
  }

  void clone()
  {
    auto s = QStringLiteral("String constructed by C++");
    const auto c = clone_qstring(s);
    QCOMPARE(c, QStringLiteral("String constructed by C++"));
  }

  void modify_rust() { QVERIFY(can_handle_qstring_change()); }

  void modify_updates_cpp()
  {
    auto s = QStringLiteral("String constructed by C++");
    modify_qstring(s);
    QCOMPARE(s, QStringLiteral("Updated string value"));
  }
};
