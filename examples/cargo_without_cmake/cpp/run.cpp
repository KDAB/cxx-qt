// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cargo_cpp_includes
#include <QtQml/QQmlApplicationEngine>

#include "cxx-qt-gen/my_object.cxxqt.h"

// ANCHOR_END: book_cargo_cpp_includes

// ANCHOR: book_cargo_run_cpp
// extern "C" is important for the linker to be able to link this
// function with Rust code.
extern "C" void
run_cpp()
{
  // ANCHOR_END: book_cargo_run_cpp

  // ANCHOR: book_cargo_run_qml
  // TODO: creating binding, needs to survive
  QQmlApplicationEngine* engine = new QQmlApplicationEngine();

  const QUrl url(QStringLiteral("qrc:/main.qml"));

  qmlRegisterType<MyObject>("com.kdab.cxx_qt.demo", 1, 0, "MyObject");

  // TODO: this explodes due to calling qApp->arguments()
  engine->load(url);
}
// ANCHOR_END: book_cargo_run_qml
