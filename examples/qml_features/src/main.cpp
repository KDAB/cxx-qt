// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>

#include "cxx-qt-gen/include/data_struct_properties.h"
#include "cxx-qt-gen/include/handler_property_change.h"
#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/include/serialisation.h"
#include "cxx-qt-gen/include/sub_object.h"
#include "cxx-qt-gen/include/types.h"

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

  qmlRegisterType<
    custom_namespace::data_struct_properties::DataStructProperties>(
    "com.kdab.cxx_qt.demo", 1, 0, "DataStructProperties");
  qmlRegisterType<
    custom_namespace::handler_property_change::HandlerPropertyChange>(
    "com.kdab.cxx_qt.demo", 1, 0, "HandlerPropertyChange");
  qmlRegisterType<custom_namespace::my_object::MyObject>(
    "com.kdab.cxx_qt.demo", 1, 0, "MyObject");
  qmlRegisterType<custom_namespace::serialisation::Serialisation>(
    "com.kdab.cxx_qt.demo", 1, 0, "Serialisation");
  qmlRegisterType<custom_namespace::sub_object::SubObject>(
    "com.kdab.cxx_qt.demo", 1, 0, "SubObject");
  qmlRegisterType<custom_namespace::types::Types>(
    "com.kdab.cxx_qt.demo", 1, 0, "Types");

  engine.load(url);

  return app.exec();
}
