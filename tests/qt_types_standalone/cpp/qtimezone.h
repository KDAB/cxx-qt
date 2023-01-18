// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QTimeZone>
#include <QtTest/QTest>

#include "cxx-qt-gen/qtimezone_cxx.cxx.h"

class QTimeZoneTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    auto t = construct_qtimezone();
    QCOMPARE(t.id(), QByteArrayLiteral("Europe/London"));
    // TODO: why do we need this?
    t.~QTimeZone();
  }

  void read()
  {
    const auto t = QTimeZone(QByteArrayLiteral("Europe/London"));
    QVERIFY(read_qtimezone(t));
  }

  void clone()
  {
    const auto t = QTimeZone(QByteArrayLiteral("Europe/London"));
    auto c = clone_qtimezone(t);
    QCOMPARE(c.id(), QByteArrayLiteral("Europe/London"));
    // TODO: why do we need this?
    c.~QTimeZone();
  }
};
