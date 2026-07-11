// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QCommandLineParser>

namespace rust {
namespace cxxqtlib1 {
using QCommandLineParserOptionsAfterPositionalArgumentsMode =
  QCommandLineParser::OptionsAfterPositionalArgumentsMode;
using QCommandLineParserSingleDashWordOptionMode =
  QCommandLineParser::SingleDashWordOptionMode;

void
qcommandlineparserAddHelpOption(QCommandLineParser& parser,
                                QCommandLineOption* uninit);

void
qcommandlineparserAddVersionOption(QCommandLineParser& parser,
                                   QCommandLineOption* uninit);

QString
qcommandlineparserValue(const QCommandLineParser& parser,
                        const QString& optionName);

QStringList
qcommandlineparserValues(const QCommandLineParser& parser,
                         const QString& optionName);

bool
qcommandlineparserIsSetFromQString(const QCommandLineParser& parser,
                                   const QString& optionName);
} // namespace cxxqtlib1
} // namespace rust
