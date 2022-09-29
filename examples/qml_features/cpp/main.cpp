// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>

#include "cxx-qt-gen/include/custom_base.cxxqt.h"
#include "cxx-qt-gen/include/my_object.cxxqt.h"
#include "cxx-qt-gen/include/serialisation.cxxqt.h"
#include "cxx-qt-gen/include/struct_properties.cxxqt.h"
#include "cxx-qt-gen/include/threading_website.cxxqt.h"
#include "cxx-qt-gen/include/types.cxxqt.h"

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

  qmlRegisterType<CustomBase>("com.kdab.cxx_qt.demo", 1, 0, "CustomBase");
  qmlRegisterType<StructProperties>(
    "com.kdab.cxx_qt.demo", 1, 0, "StructProperties");
  qmlRegisterType<MyObject>("com.kdab.cxx_qt.demo", 1, 0, "MyObject");
  qmlRegisterType<Serialisation>("com.kdab.cxx_qt.demo", 1, 0, "Serialisation");
  // ANCHOR: book_namespace_register
  qmlRegisterType<cxx_qt::website::ThreadingWebsite>(
    "com.kdab.cxx_qt.demo", 1, 0, "ThreadingWebsite");
  // ANCHOR_END: book_namespace_register
  qmlRegisterType<Types>("com.kdab.cxx_qt.demo", 1, 0, "Types");

  engine.load(url);

  return app.exec();
}
