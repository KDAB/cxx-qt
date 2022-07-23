// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>

#include "data_struct_properties.cxxqt.h"
#include "my_object.cxxqt.h"
#include "serialisation.cxxqt.h"
#include "sub_object.cxxqt.h"
#include "types.cxxqt.h"

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

  qmlRegisterType<cxx_qt::data_struct_properties::DataStructProperties>(
    "com.kdab.cxx_qt.demo", 1, 0, "DataStructProperties");
  qmlRegisterType<cxx_qt::my_object::MyObject>(
    "com.kdab.cxx_qt.demo", 1, 0, "MyObject");
  qmlRegisterType<cxx_qt::serialisation::Serialisation>(
    "com.kdab.cxx_qt.demo", 1, 0, "Serialisation");
  qmlRegisterType<cxx_qt::sub_object::SubObject>(
    "com.kdab.cxx_qt.demo", 1, 0, "SubObject");
  qmlRegisterType<cxx_qt::types::Types>("com.kdab.cxx_qt.demo", 1, 0, "Types");

  engine.load(url);

  return app.exec();
}
