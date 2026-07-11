// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

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
