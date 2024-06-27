// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QPolygonF>
#include <QtTest/QTest>

#include "qt_types_standalone/qpolygonf.cxx.h"

class QPolygonFTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto m = construct_qpolygonf();
    QVERIFY(m.isEmpty());
  }
  void clone()
  {
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
    const auto m = QPolygonF(QList<QPoint>() << QPoint(1, 2) << QPoint(3, 4));
#else
    const auto m = QPolygonF(QVector<QPoint>() << QPoint(1, 2) << QPoint(3, 4));
#endif
    const auto c = clone_qpolygonf(m);
    QCOMPARE(c.toPolygon().point(0), QPoint(1, 2));
    QCOMPARE(c.toPolygon().point(1), QPoint(3, 4));
  }
};
