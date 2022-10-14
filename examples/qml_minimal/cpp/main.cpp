// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_main_cpp
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>

// ANCHOR: book_cpp_include
#include "cxx-qt-gen/my_object.cxxqt.h"
// ANCHOR_END: book_cpp_include

int
main(int argc, char* argv[])
{
  QGuiApplication app(argc, argv);

  QQmlApplicationEngine engine;

  // ANCHOR: book_qml_url
  const QUrl url(QStringLiteral("qrc:/main.qml"));
  // ANCHOR_END: book_qml_url
  QObject::connect(
    &engine,
    &QQmlApplicationEngine::objectCreated,
    &app,
    [url](QObject* obj, const QUrl& objUrl) {
      if (!obj && url == objUrl)
        QCoreApplication::exit(-1);
    },
    Qt::QueuedConnection);

  // ANCHOR: book_qml_register
  qmlRegisterType<MyObject>("com.kdab.cxx_qt.demo", 1, 0, "MyObject");
  // ANCHOR_END: book_qml_register

  engine.load(url);

  return app.exec();
}
// ANCHOR_END: book_main_cpp
