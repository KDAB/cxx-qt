// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QStringList>
#include <QtTest/QTest>

#include "qt_types_standalone/qstringlist.cxx.h"

class QStringListTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto l =
      construct_qstringlist(QStringLiteral("https://kdab.com/"),
                            QStringLiteral("https://github.com/KDAB/cxx-qt/"));
    QVERIFY(l.contains(QStringLiteral("https://github.com/KDAB/cxx-qt/")));
    QCOMPARE(l.size(), 2);
  }

  void read()
  {
    const auto l = QStringList()
                   << QStringLiteral("https://kdab.com/")
                   << QStringLiteral("https://github.com/KDAB/cxx-qt/");
    QVERIFY(read_qstringlist(l));
  }

  void clone()
  {
    const auto l = QStringList()
                   << QStringLiteral("https://kdab.com/")
                   << QStringLiteral("https://github.com/KDAB/cxx-qt/");
    const auto c = clone_qstringlist(l);
    QVERIFY(l.contains(QStringLiteral("https://github.com/KDAB/cxx-qt/")));
    QCOMPARE(l.size(), 2);
  }
};
