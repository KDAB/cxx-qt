// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QSize>
#include <QtTest/QTest>

#include "cxx-qt-gen/qsize_cxx.cxx.h"

class QSizeTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto s = construct_qsize();
    QCOMPARE(s.width(), 1);
    QCOMPARE(s.height(), 4);
  }

  void read()
  {
    const auto s = QSize(1, 4);
    QVERIFY(read_qsize(s));
  }

  void clone()
  {
    const auto s = QSize(1, 4);
    const auto c = clone_qsize(s);
    QCOMPARE(c.width(), 1);
    QCOMPARE(c.height(), 4);
  }
};
