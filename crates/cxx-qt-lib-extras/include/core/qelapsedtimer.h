// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QElapsedTimer>

#include "rust/cxx.h"

namespace rust {
namespace cxxqtlib1 {
::std::int64_t
qelapsedtimerRestart(QElapsedTimer& elapsedTimer);

}
}
