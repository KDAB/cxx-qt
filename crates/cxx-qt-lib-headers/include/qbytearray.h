// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cstdint>

#include <QtCore/QByteArray>

#include "rust/cxx.h"

template<>
struct rust::IsRelocatable<QByteArray> : std::true_type
{
};

namespace rust {
namespace cxxqtlib1 {

QByteArray
qbytearrayFromSliceU8(::rust::Slice<const ::std::uint8_t> slice);
::rust::Vec<::std::uint8_t>
qbytearrayToVecU8(const QByteArray& byteArray);

QByteArray
qbytearrayFromRawData(::rust::Slice<const ::std::uint8_t> slice);
::rust::Slice<const ::std::uint8_t>
qbytearrayAsSlice(const QByteArray& byteArray);

}
}
