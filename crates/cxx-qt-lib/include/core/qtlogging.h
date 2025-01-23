// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include "rust/cxx.h"
#include <QDebug>
#include <QtCore/qlogging.h>

QMessageLogContext
construct_qmessagelogcontext(const char* fileName,
                             int lineNumber,
                             const char* functionName,
                             const char* categoryName);

int
qmessagelogcontext_line(const QMessageLogContext& context);

const char*
qmessagelogcontext_file(const QMessageLogContext& context);

const char*
qmessagelogcontext_function(const QMessageLogContext& context);

const char*
qmessagelogcontext_category(const QMessageLogContext& context);

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<QMessageLogContext> : ::std::true_type
{};

} // namespace rust
