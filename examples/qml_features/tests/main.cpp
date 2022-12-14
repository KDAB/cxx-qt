// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtCore/QtGlobal>
#include <QtQml/QQmlEngine>
#include <QtQuickTest/quicktest.h>

#include "cxx-qt-gen/custom_base_class.cxxqt.h"
#include "cxx-qt-gen/multiple_qobjects.cxxqt.h"
#include "cxx-qt-gen/rust_containers.cxxqt.h"
#include "cxx-qt-gen/rust_invokables.cxxqt.h"
#include "cxx-qt-gen/rust_properties.cxxqt.h"
#include "cxx-qt-gen/rust_signals.cxxqt.h"
#include "cxx-qt-gen/serialisation.cxxqt.h"
#include "cxx-qt-gen/threading_website.cxxqt.h"
#include "cxx-qt-gen/types.cxxqt.h"

class Setup : public QObject
{
  Q_OBJECT

public:
  Setup()
  {
    qmlRegisterType<CustomBaseClass>(
      "com.kdab.cxx_qt.demo", 1, 0, "CustomBaseClass");
    qmlRegisterType<FirstObject>("com.kdab.cxx_qt.demo", 1, 0, "FirstObject");
    qmlRegisterType<SecondObject>("com.kdab.cxx_qt.demo", 1, 0, "SecondObject");
    qmlRegisterType<RustContainers>(
      "com.kdab.cxx_qt.demo", 1, 0, "RustContainers");
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
