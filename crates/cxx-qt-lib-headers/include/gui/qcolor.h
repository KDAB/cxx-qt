// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#ifdef CXX_QT_GUI_FEATURE
#include <QtGui/QColor>

#include "rust/cxx.h"

// QColor still had copy & move constructors in Qt 5 but they were basically
// trivial.
#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))
template<>
struct rust::IsRelocatable<QColor> : ::std::true_type
{
};
#endif

#endif
