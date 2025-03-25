// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QtCore>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qflags.cxx.h"

class QFlagsTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto f = construct_qflags();
    QCOMPARE(f, Qt::MouseButtons(Qt::ForwardButton) | Qt::LeftButton);
  }

  void read()
  {
    const auto f = Qt::MouseButtons(Qt::ForwardButton) | Qt::LeftButton;
    QVERIFY(read_qflags(f));
  }

  void clone()
  {
    const auto f = Qt::MouseButtons(Qt::ForwardButton) | Qt::LeftButton;
    const auto c = clone_qflags(f);
    QCOMPARE(c, f);
  }

  void emptyQFlags()
  {
    const auto f = Qt::MouseButtons();
    QVERIFY(test_is_empty(f));
  }

  void addFlags()
  {
    const auto f = Qt::MouseButtons(Qt::ForwardButton) | Qt::LeftButton;
    const auto m2 = Qt::MouseButtons(Qt::LeftButton) | Qt::RightButton;
    const auto m3 = add_flags(f, m2);
    QCOMPARE(m3, f | Qt::RightButton);
  }
};
