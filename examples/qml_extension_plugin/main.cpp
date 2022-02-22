// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtCore/QDir>
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>

int
main(int argc, char* argv[])
{
  QGuiApplication app(argc, argv);

  // ANCHOR: book_extension_plugin_register
  QQmlApplicationEngine engine;
  // Add qml dir in runtime folder to QML import paths
  engine.addImportPath(QDir(QCoreApplication::applicationDirPath())
                         .filePath(QStringLiteral("qml")));
  // ANCHOR_END: book_extension_plugin_register

  const QUrl url(QStringLiteral("qrc:/qml/main.qml"));
  QObject::connect(
    &engine,
    &QQmlApplicationEngine::objectCreated,
    &app,
    [url](QObject* obj, const QUrl& objUrl) {
      if (!obj && url == objUrl)
        QCoreApplication::exit(-1);
    },
    Qt::QueuedConnection);

  engine.load(url);

  return app.exec();
}
