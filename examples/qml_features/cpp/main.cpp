// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>

#include <cxx-qt/init.h>

#include "custom_object.h"
#include "external_qobject.h"

int
main(int argc, char* argv[])
{
  cxx_qt::init();

  QGuiApplication app(argc, argv);

  QQmlApplicationEngine engine;

  const QUrl url(
    QStringLiteral("qrc:/qt/qml/com/kdab/cxx_qt/demo/qml/main.qml"));
  QObject::connect(
    &engine,
    &QQmlApplicationEngine::objectCreated,
    &app,
    [url](QObject* obj, const QUrl& objUrl) {
      if (!obj && url == objUrl)
        QCoreApplication::exit(-1);
    },
    Qt::QueuedConnection);

  qRegisterMetaType<CustomStruct>("CustomStruct");
  // Note the _cpp at the end of the URI. If qmlRegisterMetatype is used here
  // with the same URI as used by the QML elements generated from Rust, none of
  // the elements generated by Rust will be available to the QML engine.
  qmlRegisterType<CustomObject>(
    "com.kdab.cxx_qt.demo_cpp", 1, 0, "CustomObject");
  qmlRegisterType<ExternalQObject>(
    "com.kdab.cxx_qt.demo_cpp", 1, 0, "ExternalQObject");

  engine.load(url);

  return app.exec();
}
