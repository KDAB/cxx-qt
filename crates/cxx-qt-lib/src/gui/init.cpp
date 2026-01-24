// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <cxx-qt-lib/qlist.h>
#include <cxx-qt-lib/qvector.h>

#include <QtCore/QCoreApplication>

static void do_register_gui_types()
{
    qRegisterMetaType<::QList_QColor>("QList_QColor");
    qRegisterMetaType<::QVector_QColor>("QVector_QColor");
}

// Use Q_COREAPP_STARTUP_FUNCTION to defer registration until QCoreApplication
// is created. This is Qt's recommended approach for type registration.
Q_COREAPP_STARTUP_FUNCTION(do_register_gui_types)

extern "C" bool
init_cxx_qt_lib_gui()
{
  // Registration is handled automatically via Q_COREAPP_STARTUP_FUNCTION
  // when QCoreApplication is constructed.
  return true;
}
