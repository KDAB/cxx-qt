// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qstring.h"

// The layout has changed between Qt 5 and Qt 6
//
// Qt5 QString has one pointer as a member
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/text/qstring.h?h=v5.15.6-lts-lgpl#n979
//
// Qt6 QString has one member, which contains two pointers and a size_t
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/text/qstring.h?h=v6.2.4#n1094
// DataPointer is then a QStringPrivate, which is a QArrayDataPointer<char16_t>
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qarraydatapointer.h?h=v6.2.4#n390
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
static_assert(alignof(QString) <= alignof(std::size_t[3]),
              "unexpectedly large QString alignment");
static_assert(sizeof(QString) == sizeof(std::size_t[3]),
              "unexpected QString size");
#else
static_assert(alignof(QString) <= alignof(std::size_t),
              "unexpectedly large QString alignment");
static_assert(sizeof(QString) == sizeof(std::size_t),
              "unexpected QString size");
#endif

static_assert(!std::is_trivially_copy_assignable<QString>::value);
static_assert(!std::is_trivially_copy_constructible<QString>::value);

static_assert(!std::is_trivially_destructible<QString>::value);

namespace rust {
namespace cxxqtlib1 {

void
qstringDrop(QString& string)
{
  string.~QString();
}

QString
qstringInitDefault()
{
  return QString();
}

QString
qstringInitFromRustString(rust::Str string)
{
  // Note that rust::Str here is borrowed
  // and we convert back from UTF-8 to UTF-16
  return QString::fromUtf8(string.data(), string.size());
}

QString
qstringInitFromQString(const QString& string)
{
  return QString(string);
}

rust::String
qstringToRustString(const QString& string)
{
  // Note that this changes UTF-16 to UTF-8
  const auto byteArray = string.toUtf8();
  return rust::String(byteArray.constData(), byteArray.size());
}

}
}
