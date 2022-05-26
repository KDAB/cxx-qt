// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>

#include "cxx-qt-gen/include/energy_usage.cxxqt.h"
#include "energyusageproxymodel.h"
#include "sensor.h"

int
main(int argc, char* argv[])
{
  QGuiApplication app(argc, argv);

  QQmlApplicationEngine engine;

  const QUrl url(QStringLiteral("qrc:/main.qml"));
  QObject::connect(
    &engine,
    &QQmlApplicationEngine::objectCreated,
    &app,
    [url](QObject* obj, const QUrl& objUrl) {
      if (!obj && url == objUrl)
        QCoreApplication::exit(-1);
    },
    Qt::QueuedConnection);

  qmlRegisterType<cxx_qt::energy_usage::EnergyUsage>(
    "com.kdab.energy", 1, 0, "EnergyUsage");
  qmlRegisterType<EnergyUsageProxyModel>(
    "com.kdab.energy", 1, 0, "EnergyUsageProxyModel");
  qmlRegisterType<Sensor>("com.kdab.energy", 1, 0, "Sensor");

  engine.load(url);

  return app.exec();
}
