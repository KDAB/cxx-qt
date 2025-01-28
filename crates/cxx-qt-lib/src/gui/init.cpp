// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <cxx-qt-lib/qlist.h>
#include <cxx-qt-lib/qvector.h>

extern "C" int
init_cxx_qt_lib_gui()
{
  static bool initialized = false;
  if (initialized) {
    return 42;
  }
  initialized = true;

  qRegisterMetaType<::QList_QColor>("QList_QColor");
  qRegisterMetaType<::QVector_QColor>("QVector_QColor");
  return 42;
}
