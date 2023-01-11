// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qbytearray.h"

#include "assertion_utils.h"

// The layout has changed between Qt 5 and Qt 6
//
// Qt5 QByteArray has one pointer as a member
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/text/qbytearray.h?h=v5.15.6-lts-lgpl#n470
//
// Qt6 QByteArray has one member, which contains two pointers and a size_t
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/text/qbytearray.h?h=v6.2.4#n91
// DataPointer is then a QByteArrayData, which is a QArrayDataPointer<char>
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qarraydatapointer.h?h=v6.2.4#n390
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
assert_alignment_and_size(QByteArray,
                          alignof(::std::size_t),
                          sizeof(::std::size_t[3]));
#else
assert_alignment_and_size(QByteArray,
                          alignof(::std::size_t),
                          sizeof(::std::size_t));
#endif

static_assert(!::std::is_trivially_copy_assignable<QByteArray>::value);
static_assert(!::std::is_trivially_copy_constructible<QByteArray>::value);

static_assert(!::std::is_trivially_destructible<QByteArray>::value);

static_assert(QTypeInfo<QByteArray>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

QByteArray
qbytearrayFromSliceU8(::rust::Slice<const ::std::uint8_t> slice)
{
  // Note that rust::Slice here is borrowed
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  return QByteArray(reinterpret_cast<const char*>(slice.data()),
                    static_cast<qsizetype>(slice.size()));
#else
  return QByteArray(reinterpret_cast<const char*>(slice.data()),
                    static_cast<int>(slice.size()));
#endif
}

::rust::Vec<::std::uint8_t>
qbytearrayToVecU8(const QByteArray& byteArray)
{
  ::rust::Vec<::std::uint8_t> vec;
  std::copy(byteArray.cbegin(), byteArray.cend(), ::std::back_inserter(vec));
  return vec;
}

QByteArray
qbytearrayFromRawData(::rust::Slice<const ::std::uint8_t> slice)
{
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  return QByteArray::fromRawData(reinterpret_cast<const char*>(slice.data()),
                                 static_cast<qsizetype>(slice.size()));
#else
  return QByteArray::fromRawData(reinterpret_cast<const char*>(slice.data()),
                                 static_cast<int>(slice.size()));
#endif
}

::rust::Slice<::std::uint8_t>
qbytearrayAsMutSlice(QByteArray& byteArray)
{
  return ::rust::Slice<::std::uint8_t>(
    reinterpret_cast<std::uint8_t*>(byteArray.data()),
    static_cast<::std::size_t>(byteArray.size()));
}

::rust::Slice<const ::std::uint8_t>
qbytearrayAsSlice(const QByteArray& byteArray)
{
  return ::rust::Slice<const ::std::uint8_t>(
    reinterpret_cast<const std::uint8_t*>(byteArray.data()),
    static_cast<::std::size_t>(byteArray.size()));
}

void
qbytearrayAppend(QByteArray& byteArray, ::std::uint8_t ch)
{
  byteArray.append(static_cast<char>(ch));
}

void
qbytearrayFill(QByteArray& byteArray, ::std::uint8_t ch, ::rust::isize size)
{
  Q_ASSERT(size >= -1);
  // Qt 5 has an int Qt 6 has a qsizetype
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  byteArray.fill(static_cast<char>(ch), static_cast<qsizetype>(size));
#else
  byteArray.fill(static_cast<char>(ch), static_cast<int>(size));
#endif
}

void
qbytearrayInsert(QByteArray& byteArray, ::rust::isize pos, ::std::uint8_t ch)
{
  Q_ASSERT(pos >= 0);
  // Qt 5 has an int Qt 6 has a qsizetype
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  byteArray.insert(static_cast<qsizetype>(pos), static_cast<char>(ch));
#else
  byteArray.insert(static_cast<int>(pos), static_cast<char>(ch));
#endif
}

::rust::isize
qbytearrayLen(const QByteArray& byteArray)
{
  // In Qt 5 the type was int now it is qsizetype, so we need to ensure the type
  // is the same for CXX
  return static_cast<::rust::isize>(byteArray.size());
}

void
qbytearrayRemove(QByteArray& byteArray, ::rust::isize pos, ::rust::isize len)
{
  Q_ASSERT(pos >= 0);
  Q_ASSERT(len >= 0);
  // Qt 5 has an int Qt 6 has a qsizetype
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  byteArray.remove(static_cast<qsizetype>(pos), static_cast<qsizetype>(len));
#else
  byteArray.remove(static_cast<int>(pos), static_cast<int>(len));
#endif
}

void
qbytearrayReserve(QByteArray& byteArray, ::rust::isize size)
{
  Q_ASSERT(size >= 0);
  // Qt has an int Qt 6 has a qsizetype
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  byteArray.reserve(static_cast<qsizetype>(size));
#else
  byteArray.reserve(static_cast<int>(size));
#endif
}

void
qbytearrayResize(QByteArray& byteArray, ::rust::isize size)
{
  Q_ASSERT(size >= 0);
  // Qt has an int Qt 6 has a qsizetype
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
  byteArray.resize(static_cast<qsizetype>(size));
#else
  byteArray.resize(static_cast<int>(size));
#endif
}

}
}
