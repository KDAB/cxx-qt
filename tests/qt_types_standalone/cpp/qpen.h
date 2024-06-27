// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QPen>
#include <QtTest/QTest>

#include "qt_types_standalone/qpen.cxx.h"

class QPenTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto p = construct_qpen();
    QCOMPARE(p.style(), Qt::SolidLine);
    QCOMPARE(p.width(), 1);
  }

  void clone()
  {
    auto p = QPen(Qt::DashLine);
    p.setWidth(5);
    const auto c = clone_qpen(p);
    QCOMPARE(c.style(), Qt::DashLine);
    QCOMPARE(c.color(), Qt::black);
    QCOMPARE(c.width(), 5);
  }
};
