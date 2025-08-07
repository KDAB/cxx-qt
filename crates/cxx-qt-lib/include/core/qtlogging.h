// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QString>

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {
namespace cxxqtlib1 {
void
q_debug(const char* fileName, int lineNumber, const QString& message);

void
q_info(const char* fileName, int lineNumber, const QString& message);

void
q_warning(const char* fileName, int lineNumber, const QString& message);

void
q_critical(const char* fileName, int lineNumber, const QString& message);

void
q_fatal(const char* fileName, int lineNumber, const QString& message);

}
} // namespace rust
