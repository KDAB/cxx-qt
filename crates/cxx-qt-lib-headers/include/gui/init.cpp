// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <cxx-qt-lib/qlist.h>
#include <cxx-qt-lib/qvector.h>

void
cxx_qt_lib_gui_init()
{
  // QList
  qRegisterMetaType<::QList_QColor>("QList_QColor");

  // QVector
  qRegisterMetaType<::QVector_QColor>("QVector_QColor");
}
