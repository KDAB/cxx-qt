// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QUuid>

#include "rust/cxx.h"

using QUuidStringFormat = QUuid::StringFormat;
using QUuidVariant = QUuid::Variant;
using QUuidVersion = QUuid::Version;

namespace rust {
namespace cxxqtlib1 {
QUuid
quuidCreateUuidV3(const QUuid& ns, ::rust::Slice<const ::std::uint8_t> slice);

QUuid
quuidCreateUuid();

QUuid
quuidCreateUuidV5(const QUuid& ns, ::rust::Slice<const ::std::uint8_t> slice);

QUuid
quuidFromString(const QString& string);

QUuid
quuidFromStr(rust::Str string);

QUuid
quuidFromRfc4122(const QByteArray& bytes);

}
}
