// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QFont>
#include <QtGui/QFontMetrics>
#include <QtGui/QGuiApplication>
#include <QtTest/QTest>

#include "cxx-qt-gen/qfontmetrics.cxx.h"

class QFontMetricsTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    std::vector<char*> args;
    std::string path = "/path";
    args.push_back(path.data());
    auto argc = static_cast<int>(args.size());

    // QFontDatabase needs a QGuiApplication first
    QGuiApplication app(argc, args.data());
    app.setApplicationName(QStringLiteral("kdab"));

    QFont f;
    const int pointSize = 40;
    f.setPointSize(pointSize);
    const auto m = constructor_qfontmetrics(f);
    QCOMPARE(m.ascent(), 57);
    QCOMPARE(m.height(), 73);
  }
  void clone()
  {
    std::vector<char*> args;
    std::string path = "/path";
    args.push_back(path.data());
    auto argc = static_cast<int>(args.size());

    // QFontDatabase needs a QGuiApplication first
    QGuiApplication app(argc, args.data());
    app.setApplicationName(QStringLiteral("kdab"));

    QFont f;
    f.setBold(true);
    f.setPointSize(30);
    const auto m = QFontMetrics(f);

    const auto c = clone_qfontmetrics(m);
    QCOMPARE(m.ascent(), c.ascent());
    QCOMPARE(m.height(), c.height());
  }
};
