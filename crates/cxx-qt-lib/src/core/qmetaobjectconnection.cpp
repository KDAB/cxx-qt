// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qmetaobjectconnection.h"

#include "../assertion_utils.h"

// ::QMetaObject::Connection is the size of one pointer
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qobjectdefs.h?h=v5.15.6-lts-lgpl#n620
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qobjectdefs.h?h=v6.2.4#n444
assert_alignment_and_size(::QMetaObject::Connection,
                          alignof(::std::size_t),
                          sizeof(::std::size_t));

static_assert(
  !::std::is_trivially_destructible<::QMetaObject::Connection>::value);

namespace rust {
namespace cxxqtlib1 {

void
qmetaobjectconnectionDisconnect(const ::QMetaObject::Connection& conn)
{
  ::QObject::disconnect(conn);
}

}
}
