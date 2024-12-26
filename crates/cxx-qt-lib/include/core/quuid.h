// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QUuid>

#include "rust/cxx.h"

using QUuidVariant = QUuid::Variant;
using QUuidVersion = QUuid::Version;

namespace rust {
namespace cxxqtlib1 {
QUuid
quuidNewV3(const QUuid& ns, ::rust::Slice<const ::std::uint8_t> slice);

QUuid
quuidNewV4();

QUuid
quuidNewV5(const QUuid& ns, ::rust::Slice<const ::std::uint8_t> slice);

QString
quuidToString(const QUuid& uuid);

QUuid
quuidFromString(QAnyStringView string);
}
}
