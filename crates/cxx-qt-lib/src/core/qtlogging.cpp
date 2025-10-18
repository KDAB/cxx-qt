// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qtlogging.h"

#include <QtCore/qlogging.h>

namespace rust {
namespace cxxqtlib1 {

inline void
log(QtMsgType type,
    const char* fileName,
    int lineNumber,
    const QString& message)
{
  qt_message_output(
    type,
    QMessageLogContext(fileName, lineNumber, nullptr, "default"),
    message);
}

void
q_debug(const char* fileName, int lineNumber, const QString& message)
{
  log(QtMsgType::QtDebugMsg, fileName, lineNumber, message);
}

void
q_info(const char* fileName, int lineNumber, const QString& message)
{
  log(QtMsgType::QtInfoMsg, fileName, lineNumber, message);
}

void
q_warning(const char* fileName, int lineNumber, const QString& message)
{
  log(QtMsgType::QtWarningMsg, fileName, lineNumber, message);
}

void
q_critical(const char* fileName, int lineNumber, const QString& message)
{
  log(QtMsgType::QtCriticalMsg, fileName, lineNumber, message);
}

void
q_fatal(const char* fileName, int lineNumber, const QString& message)
{
  log(QtMsgType::QtFatalMsg, fileName, lineNumber, message);
}

}
}
