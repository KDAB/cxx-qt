// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qmetaobjectconnection.h"

namespace rust {
namespace cxxqtlib1 {

void
qmetaobjectconnectionDisconnect(const QMetaObject::Connection& conn)
{
  QObject::disconnect(conn);
}

}
}
