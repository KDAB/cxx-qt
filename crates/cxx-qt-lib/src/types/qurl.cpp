// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/include/qurl.h"
#include "cxx-qt-lib/include/qstring.h"

#include "assertion_utils.h"

// QUrl has a single pointer as it's member
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/io/qurl.h?h=v5.15.6-lts-lgpl#n367
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/io/qurl.h?h=v6.2.4#n294
assert_alignment_and_size(QUrl, alignof(std::size_t), sizeof(std::size_t));

static_assert(!std::is_trivially_copy_assignable<QUrl>::value);
static_assert(!std::is_trivially_copy_constructible<QUrl>::value);

static_assert(!std::is_trivially_destructible<QUrl>::value);

namespace rust {
namespace cxxqtlib1 {

void
qurlDrop(QUrl& url)
{
  url.~QUrl();
}

QUrl
qurlInitDefault()
{
  return QUrl();
}

QUrl
qurlInitFromQString(const QString& string)
{
  return QUrl(string);
}

QUrl
qurlInitFromString(rust::Str string)
{
  // Note that rust::Str here is borrowed
  // and we convert back from UTF-8 to UTF-16
  return QUrl(qstringInitFromRustString(string));
}

QUrl
qurlInitFromQUrl(const QUrl& url)
{
  return QUrl(url);
}

QString
qurlToQString(const QUrl& url)
{
  return url.toString();
}

rust::String
qurlToRustString(const QUrl& url)
{
  // Note that this changes UTF-16 to UTF-8
  return qstringToRustString(url.toString());
}

}
}
