// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt/connection.h"

#include <type_traits>

// ::QMetaObject::Connection is the size of one pointer
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qobjectdefs.h?h=v5.15.6-lts-lgpl#n620
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qobjectdefs.h?h=v6.2.4#n444
static_assert(alignof(::QMetaObject::Connection) <= (alignof(::std::size_t)),
              "unexpectedly large ::QMetaObject::Connection alignment!");
static_assert(sizeof(::QMetaObject::Connection) == (sizeof(::std::size_t)),
              "unexpected ::QMetaObject::Connection size!");

static_assert(
  !::std::is_trivially_copy_assignable<::QMetaObject::Connection>::value);
static_assert(
  !::std::is_trivially_copy_constructible<::QMetaObject::Connection>::value);
static_assert(
  !::std::is_trivially_destructible<::QMetaObject::Connection>::value);

namespace rust {
namespace cxxqt1 {

::QMetaObject::Connection
qmetaobjectconnectionDefault()
{
  return ::QMetaObject::Connection();
}

bool
qmetaobjectconnectionDisconnect(const ::QMetaObject::Connection& connection)
{
  return ::QObject::disconnect(connection);
}

void
qmetaobjectconnectionDrop(::QMetaObject::Connection& connection)
{
  connection.~QMetaObjectConnection();
}

}
}
