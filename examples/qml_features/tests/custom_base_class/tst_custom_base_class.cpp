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

class Setup : public QObject
{
  Q_OBJECT

public:
  Setup()
  {
    qmlRegisterType<CustomBaseClass>("com.kdab.cxx_qt.demo", 1, 0, "CustomBaseClass");
  }
};

QUICK_TEST_MAIN_WITH_SETUP(custom_base_class, Setup)

#include "tst_custom_base_class.moc"
