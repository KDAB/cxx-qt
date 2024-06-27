// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QByteArray>
#include <QtTest/QTest>

#include "qt_types_standalone/qbytearray.cxx.h"

class QByteArrayTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    QFETCH(bool, slice);
    const auto s = construct_qbytearray(slice);
    QCOMPARE(s, QByteArrayLiteral("String constructed by Rust"));
  }

  void construct_data()
  {
    QTest::addColumn<bool>("slice");

    QTest::newRow("From &str") << true;
    QTest::newRow("From String") << false;
  }

  void read()
  {
    const auto s = QByteArrayLiteral("String constructed by C++");
    QVERIFY(read_qbytearray(s));
  }

  void clone()
  {
    auto s = QByteArrayLiteral("String constructed by C++");
    const auto c = clone_qbytearray(s);
    QCOMPARE(c, QByteArrayLiteral("String constructed by C++"));
  }

  void modify_rust() { QVERIFY(can_handle_qbytearray_change()); }

  void modify_updates_cpp()
  {
    auto s = QByteArrayLiteral("String constructed by C++");
    modify_qbytearray(s);
    QCOMPARE(s, QByteArrayLiteral("Updated string value"));
  }

  void can_use_as_slice_cpp() { QVERIFY(can_use_as_slice()); }
};
