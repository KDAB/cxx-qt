// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "qapplication.h"

#include <cxx-qt-lib/qcoreapplication.h>

::std::unique_ptr<QApplication>
qapplicationNew(const QVector<QByteArray>& args)
{
  // Ensure that our QVector has the same lifetime as the QGuiApplication
  // by storing it inside a QObject that has QGuiApplication as it's parent
  auto argsData = new ::rust::cxxqtlib1::ApplicationArgsData(args);
  // Note that QGuiApplication uses a reference to an int for the size here
  // so we need to ensure that reference remains valid
  auto ptr =
    ::std::make_unique<QApplication>(argsData->size(), argsData->data());
  Q_ASSERT(ptr != nullptr);
  argsData->setParent(ptr.get());

  return ptr;
}
