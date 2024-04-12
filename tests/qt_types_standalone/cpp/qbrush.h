// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QBrush>
#include <QtTest/QTest>

#include "cxx-qt-gen/qbrush.cxx.h"

class QBrushTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto p = construct_qbrush();
    QCOMPARE(p.color(), QColor(Qt::red));
  }

  void clone()
  {
    const auto p = construct_qbrush();
    const auto c = clone_qbrush(p);
    QCOMPARE(c.color(), QColor(Qt::red));
  }
};
