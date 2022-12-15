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
                          alignof(std::size_t),
                          sizeof(std::size_t[3]));
#else
assert_alignment_and_size(QByteArray,
                          alignof(std::size_t),
                          sizeof(std::size_t));
#endif

static_assert(!std::is_trivially_copy_assignable<QByteArray>::value);
static_assert(!std::is_trivially_copy_constructible<QByteArray>::value);

static_assert(!std::is_trivially_destructible<QByteArray>::value);

static_assert(QTypeInfo<QByteArray>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

QByteArray
qbytearrayFromRustString(rust::Str string)
{
  // Note that rust::Str here is borrowed
  return QByteArray(string.data(), string.size());
}

rust::String
qbytearrayToRustString(const QByteArray& byteArray)
{
  return rust::String(byteArray.constData(), byteArray.size());
}

}
}
