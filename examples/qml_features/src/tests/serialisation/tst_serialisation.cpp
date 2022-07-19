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

#include "cxx-qt-gen/include/serialisation.cxxqt.h"

class Setup : public QObject
{
  Q_OBJECT

public:
  Setup()
  {
    qmlRegisterType<cxx_qt::serialisation::Serialisation>(
      "com.kdab.cxx_qt.demo", 1, 0, "Serialisation");
  }
};

QUICK_TEST_MAIN_WITH_SETUP(serialisation, Setup)

#include "tst_serialisation.moc"
