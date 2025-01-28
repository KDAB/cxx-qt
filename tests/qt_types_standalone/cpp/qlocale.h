// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Nicolas Fella <nicolas.fella@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QLocale>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qlocale.cxx.h"

class QLocaleTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto m = construct_qlocale();
    QVERIFY(!m.name().isEmpty());
  }

  void clone()
  {
    const auto locale = QLocale(QStringLiteral("de_DE"));
    // const auto c = clone_qlocale(locale);
    QCOMPARE(locale.name(), QStringLiteral("de_DE"));
  }
};
