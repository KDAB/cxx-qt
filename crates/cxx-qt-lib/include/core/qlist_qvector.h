// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGlobal>

// In Qt 6 QList and QVector are the same, so we only need IsRelocatable defined
// once
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))

#include <QtCore/QList>

#include "rust/cxx.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

// This has static asserts in the cpp file to ensure this is valid.
template<typename T>
struct IsRelocatable<QList<T>> : ::std::true_type
{
};

} // namespace rust

#endif
