// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QMetaType>

#include "rust/cxx.h"

using QMetaTypeType = QMetaType::Type;

namespace rust {

template<>
struct IsRelocatable<QMetaType> : ::std::true_type
{};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {
int
qmetatypeId(const QMetaType& metaType);
bool
qmetatypeIsRegistered(const QMetaType& metaType);
bool
qmetatypeIsValid(const QMetaType& metaType);

QMetaType
qmetatypeFromName(::rust::Slice<const ::std::uint8_t> typeName);

inline bool (*qmetatypeCanConvert)(QMetaType,
                                   QMetaType) = QMetaType::canConvert;
}
}
