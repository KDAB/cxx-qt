// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <memory>

#include <QtCore/QObject>

#include "rust/cxx.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<::QMetaObject::Connection> : ::std::true_type
{
};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {

using QMetaObjectConnection = ::QMetaObject::Connection;

void
qmetaobjectconnectionDisconnect(const ::QMetaObject::Connection& conn);

}
}
