// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QColor>
#include <QtTest/QTest>

#include "cxx-qt-gen/qcolor_cxx.cxx.h"

class QColorTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    QCOMPARE(construct_qcolor(ColorTest::Rgb_Red),
             QColor(Qt::GlobalColor::red));
    QCOMPARE(construct_qcolor(ColorTest::Rgb_Green),
             QColor(Qt::GlobalColor::green));
    QCOMPARE(construct_qcolor(ColorTest::Rgb_Blue),
             QColor(Qt::GlobalColor::blue));
    QCOMPARE(construct_qcolor(ColorTest::Rgb_Transparent),
             QColor(Qt::GlobalColor::transparent));
  }

  void read()
  {
    QVERIFY(read_qcolor(QColor(255, 0, 0, 255), ColorTest::Rgb_Red));
    QVERIFY(read_qcolor(QColor(0, 255, 0, 255), ColorTest::Rgb_Green));
    QVERIFY(read_qcolor(QColor(0, 0, 255, 255), ColorTest::Rgb_Blue));
    QVERIFY(read_qcolor(QColor(0, 0, 0, 0), ColorTest::Rgb_Transparent));

    QVERIFY(read_qcolor(QColor(Qt::red), ColorTest::Rgb_Red));
    QVERIFY(read_qcolor(QColor(Qt::green), ColorTest::Rgb_Green));
    QVERIFY(read_qcolor(QColor(Qt::blue), ColorTest::Rgb_Blue));
    QVERIFY(read_qcolor(QColor(Qt::transparent), ColorTest::Rgb_Transparent));
  }

  void clone()
  {
    const auto color = QColor(255, 0, 0, 255);
    const auto c = clone_qcolor(color);
    QCOMPARE(c, Qt::GlobalColor::red);
  }
};
