// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
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

::qsizetype
qsizetypeFromIsize(::rust::isize value);
::rust::isize
qsizetypeIntoIsize(qsizetype value);

}
}
