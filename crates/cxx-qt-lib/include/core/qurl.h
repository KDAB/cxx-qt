// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QByteArray>
#include <QtCore/QString>
#include <QtCore/QStringList>
#include <QtCore/QUrl>

#include "rust/cxx.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<QUrl> : ::std::true_type
{};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {

// Bitwise enums don't work well with Rust and CXX, so lets just use the
// defaults for now
QString
qurlAuthority(const QUrl& url);
QString
qurlFileName(const QUrl& url);
QString
qurlFragment(const QUrl& url);
QUrl
qurlFromEncoded(const QByteArray& input);
QUrl (*qurlFromLocalFile)(const QString&) = QUrl::fromLocalFile;
QString (*qurlFromPercentEncoding)(const QByteArray&) =
  QUrl::fromPercentEncoding;
QUrl
qurlFromUserInput(const QString& userInput, const QString& workingDictionary);
QString
qurlHost(const QUrl& url);
QStringList (*qurlIdnWhitelist)() = QUrl::idnWhitelist;
QString
qurlPath(const QUrl& url);
QString
qurlPassword(const QUrl& url);
QString
qurlQuery(const QUrl& url);
void
qurlSetAuthority(QUrl& url, const QString& authority);
void
qurlSetFragment(QUrl& url, const QString& fragment);
void
qurlSetHost(QUrl& url, const QString& host);
void (*qurlSetIdnWhitelist)(const QStringList&) = QUrl::setIdnWhitelist;
void
qurlSetPassword(QUrl& url, const QString& password);
void
qurlSetPath(QUrl& url, const QString& path);
void
qurlSetQuery(QUrl& url, const QString& query);
void
qurlSetScheme(QUrl& url, const QString& scheme);
void
qurlSetUrl(QUrl& url, const QString& newUrl);
void
qurlSetUserInfo(QUrl& url, const QString& userInfo);
void
qurlSetUserName(QUrl& url, const QString& userName);
QString
qurlToDisplayString(const QUrl& url);
QByteArray
qurlToEncoded(const QUrl& url);
QString
qurlToQString(const QUrl& url);
QByteArray (*qurlToPercentEncoding)(const QString&,
                                    const QByteArray&,
                                    const QByteArray&) =
  QUrl::toPercentEncoding;
QString
qurlUserInfo(const QUrl& url);
QString
qurlUserName(const QUrl& url);

}
}
