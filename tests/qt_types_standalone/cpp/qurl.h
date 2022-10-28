// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QUrl>
#include <QtTest/QTest>

#include "qt-types-standalone/qurl_cxx.cxx.h"

class QUrlTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    QCOMPARE(construct_qurl(QStringLiteral("https://kdab.com/")),
             QUrl(QStringLiteral("https://kdab.com/")));
    QCOMPARE(construct_qurl(QStringLiteral("https://github.com/KDAB/cxx-qt/")),
             QUrl(QStringLiteral("https://github.com/KDAB/cxx-qt/")));
  }

  void read()
  {
    QVERIFY(read_qurl(QUrl(QStringLiteral("https://github.com/KDAB/cxx-qt/")),
                      QStringLiteral("https://github.com/KDAB/cxx-qt/")));
    QVERIFY(read_qurl(QUrl(QStringLiteral("https://kdab.com")),
                      QStringLiteral("https://kdab.com")));
  }

  void clone()
  {
    const auto u = QUrl(QStringLiteral("https://kdab.com/"));
    const auto c = clone_qurl(u);
    QCOMPARE(c, u);
  }
};
