// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtCore/QtGlobal>
#include <QtQml/QQmlEngine>
#include <QtQuickTest/quicktest.h>

#include "qml-features/custom_base_class.cxxqt.h"
#include "qml-features/rust_invokables.cxxqt.h"
#include "qml-features/rust_properties.cxxqt.h"
#include "qml-features/rust_signals.cxxqt.h"
#include "qml-features/serialisation.cxxqt.h"
#include "qml-features/threading_website.cxxqt.h"
#include "qml-features/types.cxxqt.h"

class Setup : public QObject
{
  Q_OBJECT

public:
  Setup()
  {
    qmlRegisterType<CustomBaseClass>(
      "com.kdab.cxx_qt.demo", 1, 0, "CustomBaseClass");
    qmlRegisterType<RustInvokables>(
      "com.kdab.cxx_qt.demo", 1, 0, "RustInvokables");
    qmlRegisterType<RustProperties>(
      "com.kdab.cxx_qt.demo", 1, 0, "RustProperties");
    qmlRegisterType<RustSignals>("com.kdab.cxx_qt.demo", 1, 0, "RustSignals");
    qmlRegisterType<Serialisation>(
      "com.kdab.cxx_qt.demo", 1, 0, "Serialisation");
    qmlRegisterType<cxx_qt::website::ThreadingWebsite>(
      "com.kdab.cxx_qt.demo", 1, 0, "ThreadingWebsite");
    qmlRegisterType<Types>("com.kdab.cxx_qt.demo", 1, 0, "Types");
  }
};

QUICK_TEST_MAIN_WITH_SETUP(main, Setup)

#include "main.moc"
