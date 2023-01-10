// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QPersistentModelIndex>

#include "rust/cxx.h"

// This has static asserts in the cpp file to ensure this is valid.
template<>
struct rust::IsRelocatable<QPersistentModelIndex> : ::std::true_type
{
};
