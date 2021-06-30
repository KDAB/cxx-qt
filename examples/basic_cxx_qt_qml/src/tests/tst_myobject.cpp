// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtCore/QtGlobal>
#include <QtQml/QQmlEngine>
#include <QtQuickTest/quicktest.h>

#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/include/sub_object.h"

class Setup : public QObject
{
  Q_OBJECT

public:
  Setup()
  {
    qmlRegisterType<MyObject>("com.kdab.cxx_qt.demo", 1, 0, "MyObject");
    qmlRegisterType<SubObject>("com.kdab.cxx_qt.demo", 1, 0, "SubObject");
  }
};

QUICK_TEST_MAIN_WITH_SETUP(myobject, Setup)

#include "tst_myobject.moc"
