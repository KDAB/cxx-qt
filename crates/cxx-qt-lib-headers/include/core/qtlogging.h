// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QtLogging>
#include <QDebug>
#include "rust/cxx.h"

int qmessagelogcontext_line(const QMessageLogContext &context);

void qmessagelogcontext_set_line(QMessageLogContext &context, const int line);

const char *qmessagelogcontext_file(const QMessageLogContext &context);

void qmessagelogcontext_set_file(QMessageLogContext &context, const char *file);

const char *qmessagelogcontext_function(const QMessageLogContext &context);

void qmessagelogcontext_set_function(QMessageLogContext &context, const char *function);

const char *qmessagelogcontext_category(const QMessageLogContext &context);

void qmessagelogcontext_set_category(QMessageLogContext &context, const char *category);

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<QMessageLogContext> : ::std::true_type
{
};

} // namespace rust