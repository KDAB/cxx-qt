// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QPainterPath>
#include <QtTest/QTest>

#include "cxx-qt-gen/qpainterpath.cxx.h"

class QPainterPathTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto p = construct_qpainterpath();
    QVERIFY(p.isEmpty());
  }

  void clone()
  {
    auto p = QPainterPath();
    p.addEllipse(12, 5, 7, 9);
    const auto c = clone_qpainterpath(p);
    QVERIFY(!c.isEmpty());
    QCOMPARE(c.elementCount(), p.elementCount());
  }
};
