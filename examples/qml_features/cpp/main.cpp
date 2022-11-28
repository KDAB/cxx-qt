// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>

#include "cxx-qt-gen/custom_base_class.cxxqt.h"
#include "cxx-qt-gen/multiple_qobjects.cxxqt.h"
#include "cxx-qt-gen/nested_qobjects.cxxqt.h"
#include "cxx-qt-gen/rust_containers.cxxqt.h"
#include "cxx-qt-gen/rust_invokables.cxxqt.h"
#include "cxx-qt-gen/rust_properties.cxxqt.h"
#include "cxx-qt-gen/rust_signals.cxxqt.h"
#include "cxx-qt-gen/serialisation.cxxqt.h"
#include "cxx-qt-gen/threading_website.cxxqt.h"
#include "cxx-qt-gen/types.cxxqt.h"

#include "custom_object.h"

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

  qRegisterMetaType<CustomStruct>("CustomStruct");
  qmlRegisterType<CustomObject>("com.kdab.cxx_qt.demo", 1, 0, "CustomObject");
  qmlRegisterType<RustContainers>(
    "com.kdab.cxx_qt.demo", 1, 0, "RustContainers");
  qmlRegisterType<CustomBaseClass>(
    "com.kdab.cxx_qt.demo", 1, 0, "CustomBaseClass");
  qmlRegisterType<FirstObject>("com.kdab.cxx_qt.demo", 1, 0, "FirstObject");
  qmlRegisterType<SecondObject>("com.kdab.cxx_qt.demo", 1, 0, "SecondObject");
  qmlRegisterType<RustInvokables>(
    "com.kdab.cxx_qt.demo", 1, 0, "RustInvokables");
  qmlRegisterType<RustProperties>(
    "com.kdab.cxx_qt.demo", 1, 0, "RustProperties");
  qmlRegisterType<RustSignals>("com.kdab.cxx_qt.demo", 1, 0, "RustSignals");
  qmlRegisterType<Serialisation>("com.kdab.cxx_qt.demo", 1, 0, "Serialisation");
  // ANCHOR: book_namespace_register
  qmlRegisterType<cxx_qt::website::ThreadingWebsite>(
    "com.kdab.cxx_qt.demo", 1, 0, "ThreadingWebsite");
  // ANCHOR_END: book_namespace_register
  qmlRegisterType<Types>("com.kdab.cxx_qt.demo", 1, 0, "Types");
  qmlRegisterType<InnerObject>("com.kdab.cxx_qt.demo", 1, 0, "InnerObject");
  qmlRegisterType<OuterObject>("com.kdab.cxx_qt.demo", 1, 0, "OuterObject");

  engine.load(url);

  return app.exec();
}
