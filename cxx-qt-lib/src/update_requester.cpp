// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/include/update_requester.h"

#include <QtCore/QMetaObject>

namespace rust {
namespace cxxqtlib1 {

UpdateRequester::UpdateRequester(QPointer<QObject> obj, const char* method)
  : m_method(method)
  , m_obj(obj)
{
}

bool
UpdateRequester::requestUpdate() const
{
  if (m_obj == nullptr) {
    return false;
  }

  return QMetaObject::invokeMethod(m_obj, m_method, Qt::QueuedConnection);
}

} // namespace cxxqtlib1
} // namespace rust
