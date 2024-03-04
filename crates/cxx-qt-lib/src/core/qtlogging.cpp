// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qtlogging.h"

#include "../assertion_utils.h"

// QMessageLogContext has three "const char*" members for line, category, etc
// https://codebrowser.dev/qt5/qtbase/src/corelib/global/qlogging.h.html#QMessageLogContext
assert_alignment_and_size(QMessageLogContext, alignof(intptr_t), sizeof(intptr_t) * 4);

static_assert(::std::is_trivially_copyable<QMessageLogContext>::value,
              "QMessageLogContext must be trivially copyable");

int qmessagelogcontext_line(const QMessageLogContext &context) {
  return context.line;
}

void qmessagelogcontext_set_line(QMessageLogContext &context, const int line) {
  context.line = line;
}

const char *qmessagelogcontext_file(const QMessageLogContext &context) {
  return context.file;
}

void qmessagelogcontext_set_file(QMessageLogContext &context, const char *file) {
  context.file = file;
}

const char *qmessagelogcontext_function(const QMessageLogContext &context) {
  return context.function;
}

void qmessagelogcontext_set_function(QMessageLogContext &context, const char *function) {
  context.function = function;
}

const char *qmessagelogcontext_category(const QMessageLogContext &context) {
  return context.category;
}

void qmessagelogcontext_set_category(QMessageLogContext &context, const char *category) {
  context.category = category;
}
