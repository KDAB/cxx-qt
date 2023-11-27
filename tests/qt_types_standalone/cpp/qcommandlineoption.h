// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QCommandLineOption>
#include <QtTest/QTest>

#include "cxx-qt-gen/qline_cxx.cxx.h"

class QCommandLineOptionTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto m = construct_qcommandlineoption();
    QCOMPARE(m.x1(), 1);
    QCOMPARE(m.y1(), 2);
    QCOMPARE(m.x2(), 3);
    QCOMPARE(m.y2(), 4);
  }

  void read()
  {
    const auto m = QCommandLineOption(QStringLiteral("foo"));
    QVERIFY(read_qcommandlineoption(m));
  }

  void clone()
  {
    const auto m = QCommandLineOption(QStringLiteral("foo"));
    const auto c = clone_qcommandlineoption(m);
    QCOMPARE(c.names(), QStringList());
  }
};
