// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QCoreApplication>
#include <QtQml/QQmlEngine>
#include <QtTest/QTest>

#include "cxx-qt-gen/qqmlengine_cxx.cxx.h"

class QQmlEngineTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    // QQmlEngine requires a QApplication
    std::vector<char*> args;
    std::string path = "/path";
    args.push_back(path.data());
    auto argc = static_cast<int>(args.size());
    QCoreApplication app(argc, args.data());

    const auto engine = construct_qqmlengine();
    QVERIFY(engine != nullptr);
    QCOMPARE(engine->baseUrl(), QUrl(QStringLiteral("qrc:/kdab.qml")));
  }

  void read()
  {
    // QQmlEngine requires a QApplication
    std::vector<char*> args;
    std::string path = "/path";
    args.push_back(path.data());
    auto argc = static_cast<int>(args.size());
    QCoreApplication app(argc, args.data());

    QQmlEngine engine;
    engine.setBaseUrl(QUrl(QStringLiteral("qrc:/kdab.qml")));
    QVERIFY(read_qqmlengine(engine));
  }
};
