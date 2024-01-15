// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#ifdef CXX_QT_GUI_FEATURE
#include <cinttypes>

#include <QtGui/QPainterPath>

#include "rust/cxx.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<QPainterPath> : ::std::true_type
{
};

} // namespace rust
#endif
