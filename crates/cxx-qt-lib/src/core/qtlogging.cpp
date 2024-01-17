// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qtlogging.h"

#include <cxx-qt-lib/assertion_utils.h>

// QMessageLogContext has three "const char*" members for line, category, etc
// https://codebrowser.dev/qt5/qtbase/src/corelib/global/qlogging.h.html#QMessageLogContext
assert_alignment_and_size(QMessageLogContext, {
  int version;
  int line;
  const char* file;
  const char* function;
  const char* category;
});

static_assert(!::std::is_trivially_copy_assignable<QMessageLogContext>::value);
static_assert(
  !::std::is_trivially_copy_constructible<QMessageLogContext>::value);
static_assert(::std::is_trivially_destructible<QMessageLogContext>::value);

QMessageLogContext
construct_qmessagelogcontext(const char* fileName,
                             int lineNumber,
                             const char* functionName,
                             const char* categoryName)
{
  return QMessageLogContext(fileName, lineNumber, functionName, categoryName);
}

int
qmessagelogcontext_line(const QMessageLogContext& context)
{
  return context.line;
}

const char*
qmessagelogcontext_file(const QMessageLogContext& context)
{
  return context.file;
}

const char*
qmessagelogcontext_function(const QMessageLogContext& context)
{
  return context.function;
}

const char*
qmessagelogcontext_category(const QMessageLogContext& context)
{
  return context.category;
}
