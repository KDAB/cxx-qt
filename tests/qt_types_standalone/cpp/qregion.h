// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QRegion>
#include <QtTest/QTest>

#include "cxx-qt-gen/qregion.cxx.h"

class QRegionTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto p = construct_qregion();
    QVERIFY(p.isEmpty());
  }

  void clone()
  {
    const auto p = QRegion(2, 4, 5, 3);
    const auto c = clone_qregion(p);
    QCOMPARE(p, c);
  }
};
