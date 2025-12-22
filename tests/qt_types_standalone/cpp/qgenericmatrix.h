// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QGenericMatrix>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qgenericmatrix.cxx.h"

class QGenericMatrixTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const QMatrix2x3 m = construct_qmatrix_ascending();
    QCOMPARE(m(2, 0), 4);
  }

  void set()
  {
    QMatrix2x3 m;
    set_qmatrix_value(m, 2, 0, 3);
    QCOMPARE(m(2, 0), 3);
  }
};
