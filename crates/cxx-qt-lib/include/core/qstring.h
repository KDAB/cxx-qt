// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QByteArray>
#include <QtCore/QStringList>

#include "rust/cxx.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<QString> : ::std::true_type
{};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {

QString
qstringInitFromRustString(::rust::Str string);

::rust::Slice<const ::std::uint16_t>
qstringAsSlice(const QString& string);

::rust::Slice<const QChar>
qstringAsChars(const QString& string);

QChar
qstringAt(const QString& string, ::rust::isize position);

QString
qstringArg(const QString& string, const QString& a);
::rust::isize
qstringIndexOf(const QString& string,
               const QString& str,
               ::rust::isize from,
               Qt::CaseSensitivity cs);
QString&
qstringInsert(QString& string, ::rust::isize pos, const QString& str);
QString
qstringLeft(const QString& string, ::rust::isize n);
::rust::isize
qstringLen(const QString& string);
QString
qstringMid(const QString& string, ::rust::isize position, ::rust::isize n);
QString
qstringRight(const QString& string, ::rust::isize n);
QStringList
qstringSplit(const QString& string,
             const QString& sep,
             Qt::SplitBehaviorFlags behavior,
             Qt::CaseSensitivity cs);

// If Q_COMPILER_REF_QUALIFIERS is set the definition of these is
// T method() const& which CXX doesn't bind it.
QString
qstringSimplified(const QString& string);
QByteArray
qstringToLatin1(const QString& string);
QByteArray
qstringToLocal8Bit(const QString& string);
QString
qstringToLower(const QString& string);
QString
qstringToUpper(const QString& string);
QByteArray
qstringToUtf8(const QString& string);
QString
qstringTrimmed(const QString& string);

}
}
