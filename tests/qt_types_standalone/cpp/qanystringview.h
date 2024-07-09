// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QAnyStringView>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qanystringview.cxx.h"

class QAnyStringViewTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto s = construct_qanystringview("String constructed by Rust");
    QCOMPARE(s, QByteArrayLiteral("String constructed by Rust"));
  }

  void construct_qstring()
  {
    const auto s = construct_qanystringview_qstring(
      QStringLiteral("String constructed by Rust"));
    QCOMPARE(s, QByteArrayLiteral("String constructed by Rust"));
  }

  void clone()
  {
    const auto l = QAnyStringView("Test");
    const auto c = clone_qanystringview(l);
    QCOMPARE(c, l);
  }
};
