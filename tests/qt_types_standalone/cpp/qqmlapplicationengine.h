// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QCoreApplication>
#include <QtQml/QQmlApplicationEngine>
#include <QtTest/QTest>

#include "cxx-qt-gen/qqmlapplicationengine_cxx.cxx.h"

class QQmlApplicationEngineTest : public QObject
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

    const auto engine = construct_qqmlapplicationengine();
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

    QQmlApplicationEngine engine;
    engine.setBaseUrl(QUrl(QStringLiteral("qrc:/kdab.qml")));
    QVERIFY(read_qqmlapplicationengine(engine));
  }
};
