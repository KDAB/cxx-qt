// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QCommandLineParser>

#include "rust/cxx.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<QCommandLineParser> : ::std::true_type
{
};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {
using QCommandLineParserOptionsAfterPositionalArgumentsMode = QCommandLineParser::OptionsAfterPositionalArgumentsMode;
using QCommandLineParserSingleDashWordOptionMode = QCommandLineParser::SingleDashWordOptionMode;

QString
qcommandlineparserValue(const QCommandLineParser& parser,
                        const QString& optionName);

QStringList
qcommandlineparserValues(const QCommandLineParser& parser,
                         const QString& optionName);
}
}
