// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qstring.h"

#include <cxx-qt-lib/assertion_utils.h>

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
assert_alignment_and_size(QString, {
  ::std::size_t a0;
  ::std::size_t a1;
  ::std::size_t a2;
});
#else
assert_alignment_and_size(QString, { ::std::size_t a0; });
#endif

static_assert(!::std::is_trivially_copy_assignable<QString>::value);
static_assert(!::std::is_trivially_copy_constructible<QString>::value);

static_assert(!::std::is_trivially_destructible<QString>::value);

static_assert(QTypeInfo<QString>::isRelocatable);

// Qt 5 has an int Qt 6 has a qsizetype
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
constexpr qsizetype
castIndex(::rust::isize index) noexcept
{
  return static_cast<qsizetype>(index);
}
#else
constexpr int
castIndex(::rust::isize index) noexcept
{
  return static_cast<int>(index);
}
#endif

namespace rust {
namespace cxxqtlib1 {

QString
qstringInitFromRustString(::rust::Str string)
{
  // Note that rust::Str here is borrowed
  // and we convert back from UTF-8 to UTF-16
  return QString::fromUtf8(string.data(), string.size());
}

::rust::Slice<const QChar>
qstringAsChars(const QString& string)
{
  return ::rust::Slice<const QChar>(
    reinterpret_cast<const QChar*>(string.data()),
    static_cast<::std::size_t>(string.size()));
}

::rust::Slice<const ::std::uint16_t>
qstringAsSlice(const QString& string)
{
  return ::rust::Slice<const ::std::uint16_t>(
    reinterpret_cast<const std::uint16_t*>(string.data()),
    static_cast<::std::size_t>(string.size()));
}

QChar
qstringAt(const QString& string, ::rust::isize position)
{
  return string.at(castIndex(position));
}

QString
qstringArg(const QString& string, const QString& a)
{
  // CXX can't choose between arg overloads so use C++
  return string.arg(a);
}

::rust::isize
qstringIndexOf(const QString& string,
               QChar ch,
               ::rust::isize from,
               Qt::CaseSensitivity cs)
{
  return static_cast<::rust::isize>(string.indexOf(ch, castIndex(from), cs));
}
::rust::isize
qstringIndexOf(const QString& string,
               const QString& str,
               ::rust::isize from,
               Qt::CaseSensitivity cs)
{
  return static_cast<::rust::isize>(string.indexOf(str, castIndex(from), cs));
}

QString&
qstringInsert(QString& string, ::rust::isize pos, QChar ch)
{
  Q_ASSERT(pos >= 0);
  return string.insert(castIndex(pos), ch);
}
QString&
qstringInsert(QString& string, ::rust::isize pos, const QString& str)
{
  Q_ASSERT(pos >= 0);
  return string.insert(castIndex(pos), str);
}

QString
qstringLeft(const QString& string, ::rust::isize n)
{
  Q_ASSERT(n >= 0);
  return string.left(castIndex(n));
}

::rust::isize
qstringLen(const QString& string)
{
  // In Qt 5 the type was int now it is qsizetype, so we need to ensure the type
  // is the same for CXX
  return static_cast<::rust::isize>(string.size());
}

QString
qstringMid(const QString& string, ::rust::isize position, ::rust::isize n)
{
  Q_ASSERT(position >= 0);
  Q_ASSERT(n >= -1);
  return string.mid(castIndex(position), castIndex(n));
}

QString
qstringRight(const QString& string, ::rust::isize n)
{
  Q_ASSERT(n >= 0);
  return string.right(castIndex(n));
}

QStringList
qstringSplit(const QString& string,
             const QString& sep,
             Qt::SplitBehaviorFlags behavior,
             Qt::CaseSensitivity cs)
{
  return string.split(sep, behavior, cs);
}
QStringList
qstringSplit(const QString& string,
             QChar sep,
             Qt::SplitBehaviorFlags behavior,
             Qt::CaseSensitivity cs)
{
  return string.split(sep, behavior, cs);
}

QString
qstringSimplified(const QString& string)
{
  return string.simplified();
}

QByteArray
qstringToLatin1(const QString& string)
{
  return string.toLatin1();
}

QByteArray
qstringToLocal8Bit(const QString& string)
{
  return string.toLocal8Bit();
}

QString
qstringToLower(const QString& string)
{
  return string.toLower();
}

QString
qstringToUpper(const QString& string)
{
  return string.toUpper();
}

QByteArray
qstringToUtf8(const QString& string)
{
  return string.toUtf8();
}

QString
qstringTrimmed(const QString& string)
{
  return string.trimmed();
}

}
}
