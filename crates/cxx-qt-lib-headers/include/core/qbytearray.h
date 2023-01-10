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
struct rust::IsRelocatable<QByteArray> : ::std::true_type
{
};

namespace rust {
namespace cxxqtlib1 {

QByteArray
qbytearrayFromSliceU8(::rust::Slice<const ::std::uint8_t> slice);
::rust::Vec<::std::uint8_t>
qbytearrayToVecU8(const QByteArray& byteArray);

::rust::Slice<::std::uint8_t>
qbytearrayAsMutSlice(QByteArray& byteArray);
QByteArray
qbytearrayFromRawData(::rust::Slice<const ::std::uint8_t> slice);
::rust::Slice<const ::std::uint8_t>
qbytearrayAsSlice(const QByteArray& byteArray);

void
qbytearrayAppend(QByteArray& byteArray, ::std::uint8_t ch);
void
qbytearrayFill(QByteArray& byteArray, ::std::uint8_t ch, ::rust::isize size);
void
qbytearrayInsert(QByteArray& byteArray, ::rust::isize pos, ::std::uint8_t ch);
::rust::isize
qbytearrayLen(const QByteArray& byteArray);
void
qbytearrayRemove(QByteArray& byteArray, ::rust::isize pos, ::rust::isize len);
void
qbytearrayReserve(QByteArray& byteArray, ::rust::isize size);
void
qbytearrayResize(QByteArray& byteArray, ::rust::isize size);

}
}
