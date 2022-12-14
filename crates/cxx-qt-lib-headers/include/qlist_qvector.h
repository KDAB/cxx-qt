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

// This has static asserts in the cpp file to ensure this is valid.
template<typename T>
struct rust::IsRelocatable<QList<T>> : std::true_type
{
};

#endif
