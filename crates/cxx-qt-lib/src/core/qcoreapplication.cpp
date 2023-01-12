// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qcoreapplication.h"

namespace rust {
namespace cxxqtlib1 {

ApplicationArgsData::ApplicationArgsData(const QVector<QByteArray>& args,
                                         QObject* parent)
  : QObject(parent)
  , m_ownedVector(args)
{
  // Construct our vector of char*
  for (auto& bytes : m_ownedVector) {
    m_vector.emplace_back(bytes.data());
    m_size += 1;
  }
}

char**
ApplicationArgsData::data()
{
  return m_vector.data();
}

int&
ApplicationArgsData::size()
{
  return m_size;
}

::std::unique_ptr<QCoreApplication>
qcoreapplicationNew(const QVector<QByteArray>& args)
{
  // Ensure that our QVector has the same lifetime as the QGuiApplication
  // by storing it inside a QObject that has QGuiApplication as it's parent
  auto argsData = new ApplicationArgsData(args);
  // Note that QGuiApplication uses a reference to an int for the size here
  // so we need to ensure that reference remains valid
  auto ptr =
    ::std::make_unique<QCoreApplication>(argsData->size(), argsData->data());
  Q_ASSERT(ptr != nullptr);
  argsData->setParent(ptr.get());

  return ptr;
}

}
}
