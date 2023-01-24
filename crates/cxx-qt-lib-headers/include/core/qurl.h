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

template<>
struct rust::IsRelocatable<QUrl> : ::std::true_type
{
};

namespace rust {
namespace cxxqtlib1 {

QUrl
qurlInitFromString(::rust::Str string);
::rust::String
qurlToRustString(const QUrl& url);

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
QUrl
qurlFromLocalFile(const QString& localFile);
QString
qurlFromPercentEncoding(const QByteArray& input);
QUrl
qurlFromUserInput(const QString& userInput, const QString& workingDirectory);
QString
qurlHost(const QUrl& url);
QStringList
qurlIdnWhitelist();
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
void
qurlSetIdnWhitelist(const QStringList& list);
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
QByteArray
qurlToPercentEncoding(const QString& input,
                      const QByteArray& exclude,
                      const QByteArray& include);
QString
qurlToQString(const QUrl& url);
QString
qurlUserInfo(const QUrl& url);
QString
qurlUserName(const QUrl& url);

}
}
