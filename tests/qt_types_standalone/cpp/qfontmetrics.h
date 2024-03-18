// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QFont>
#include <QtGui/QFontMetrics>
#include <QtTest/QTest>

#include "cxx-qt-gen/qfontmetrics.cxx.h"

class QFontMetricsTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
  }
  void clone()
  {
    QFont f;
    f.setBold(true);
    f.setPointSize(30);
    const auto m = QFontMetrics(f);

    const auto c = clone_qfontmetrics(m);
    QCOMPARE(m.ascent(), c.ascent());
    QCOMPARE(m.height(), c.height());
  }
};
