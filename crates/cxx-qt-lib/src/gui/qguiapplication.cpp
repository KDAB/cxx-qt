// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qguiapplication.h"

#include "cxx-qt-lib/qcoreapplication.h"

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QGuiApplication>
qguiapplicationNew(const QVector<QByteArray>& args)
{
  // Ensure that our QVector has the same lifetime as the QGuiApplication
  // by storing it inside a QObject that has QGuiApplication as it's parent
  auto argsData = new ApplicationArgsData(args);
  // Note that QGuiApplication uses a reference to an int for the size here
  // so we need to ensure that reference remains valid
  auto ptr =
    ::std::make_unique<QGuiApplication>(argsData->size(), argsData->data());
  Q_ASSERT(ptr != nullptr);
  argsData->setParent(ptr.get());

  return ptr;
}

}
}
