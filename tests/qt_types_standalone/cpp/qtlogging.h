// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/qlogging.h>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qtlogging.cxx.h"

static QtMessageHandler originalHandler = nullptr;
static QString loggedMessage{};

class QtLoggingTest : public QObject
{
  Q_OBJECT

private:
  static void logAndStore(QtMsgType type,
                          const QMessageLogContext& context,
                          const QString& msg)
  {
    if (type == QtMsgType::QtInfoMsg) {
      loggedMessage = QStringLiteral("%1:%2 - %3")
                        .arg(QString::fromUtf8(context.file))
                        .arg(context.line)
                        .arg(msg);
    }
    if (originalHandler) {
      (*originalHandler)(type, context, msg);
    }
  }

private Q_SLOTS:
  void log()
  {
    originalHandler = qInstallMessageHandler(logAndStore);
    const QString expectedMessage = log_info(QStringLiteral("test message"));
    qInstallMessageHandler(originalHandler);
    QCOMPARE(loggedMessage, expectedMessage);
  }
};
