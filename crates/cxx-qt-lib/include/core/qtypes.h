// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cstdint>

#include <QtCore/Qt>

#include "rust/cxx.h"

namespace rust {
namespace cxxqtlib1 {

::qint64
qint64FromI64(::std::int64_t value);
::std::int64_t
qint64IntoI64(::qint64 value);

::qintptr
qintptrFromIsize(::rust::isize value);
::rust::isize
qintptrIntoIsize(qintptr value);

::quint64
quint64FromU64(::std::uint64_t value);
::std::uint64_t
quint64IntoU64(::quint64 value);

::quintptr
quintptrFromUsize(::rust::usize value);
::rust::usize
quintptrIntoUsize(quintptr value);

::qsizetype
qsizetypeFromIsize(::rust::isize value);
::rust::isize
qsizetypeIntoIsize(qsizetype value);

}
}
