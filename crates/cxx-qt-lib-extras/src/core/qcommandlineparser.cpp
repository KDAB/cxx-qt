// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib-extras/include/qcommandlineparser.h"

#include <cxx-qt-lib/assertion_utils.h>

// QCommandLineParser has a single pointer as it's member
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qcommandlineparser.h?h=v5.15.6-lts-lgpl#n107
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qcommandlineparser.h?h=v6.2.4#n109
assert_alignment_and_size(QCommandLineParser, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QCommandLineParser>::value);

namespace rust {
namespace cxxqtlib1 {
QString
qcommandlineparserValue(const QCommandLineParser& parser,
                        const QString& optionName)
{
  return parser.value(optionName);
}

QStringList
qcommandlineparserValues(const QCommandLineParser& parser,
                         const QString& optionName)
{
  return parser.values(optionName);
}

bool
qcommandlineparserIsSetFromQString(const QCommandLineParser& parser,
                                   const QString& optionName)
{
  return parser.isSet(optionName);
}

}
}
