// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QCoreApplication>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qcoreapplication.cxx.h"

class QCoreApplicationTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto app = construct_qcoreapplication();
    QVERIFY(app != nullptr);
    QCOMPARE(app->applicationName(), QStringLiteral("kdab"));
  }

  void read()
  {
    std::vector<char*> args;
    std::string path = "/path";
    args.push_back(path.data());
    auto argc = static_cast<int>(args.size());

    QCoreApplication app(argc, args.data());
    app.setApplicationName(QStringLiteral("kdab"));
    QVERIFY(read_qcoreapplication(app));
  }
};
