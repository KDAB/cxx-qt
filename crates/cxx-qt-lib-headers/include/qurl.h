// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QUrl>

#include "rust/cxx.h"

template<>
struct rust::IsRelocatable<QUrl> : std::true_type
{
};
static_assert(QTypeInfo<QUrl>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

void
qurlDrop(QUrl& url);
QUrl
qurlInitDefault();
QUrl
qurlInitFromQString(const QString& string);
QUrl
qurlInitFromString(rust::Str string);
QUrl
qurlInitFromQUrl(const QUrl& url);
QString
qurlToQString(const QUrl& url);
rust::String
qurlToRustString(const QUrl& url);

}
}
