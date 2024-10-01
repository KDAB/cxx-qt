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

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<QByteArray> : ::std::true_type
{};

} // namespace rust

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
qbytearrayPrepend(QByteArray& byteArray, ::std::uint8_t ch);
void
qbytearrayRemove(QByteArray& byteArray, ::rust::isize pos, ::rust::isize len);
void
qbytearrayReserve(QByteArray& byteArray, ::rust::isize size);
void
qbytearrayResize(QByteArray& byteArray, ::rust::isize size);
// If Q_COMPILER_REF_QUALIFIERS is set the definition of these is
// T method() const& which CXX doesn't bind it.
QByteArray
qbytearraySimplified(const QByteArray& byteArray);
QByteArray
qbytearrayToLower(const QByteArray& byteArray);
QByteArray
qbytearrayToUpper(const QByteArray& byteArray);
QByteArray
qbytearrayTrimmed(const QByteArray& byteArray);
}
}
