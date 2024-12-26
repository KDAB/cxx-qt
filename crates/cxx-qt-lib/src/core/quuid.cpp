// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/quuid.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QUuid, {
  ::std::uint32_t data1;
  ::std::uint16_t data2;
  ::std::uint16_t data3;
  ::std::uint8_t data4[8];
});

static_assert(::std::is_trivially_copyable<QUuid>::value,
              "QUuid must be trivially copyable!");

#if QT_VERSION >= QT_VERSION_CHECK(6, 8, 0)
#define byteView(slice) QByteArrayView(slice.data(), slice.length())
#elif QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
#define byteView(slice)                                                        \
  QByteArray::fromRawData(reinterpret_cast<const char*>(slice.data()),         \
                          static_cast<qsizetype>(slice.size()))
#else
#define byteView(slice)                                                        \
  QByteArray::fromRawData(reinterpret_cast<const char*>(slice.data()),         \
                          static_cast<int>(slice.size()))
#endif

namespace rust {
namespace cxxqtlib1 {
QUuid
quuidNewV3(const QUuid& ns, ::rust::Slice<const ::std::uint8_t> slice)
{
  return QUuid::createUuidV3(ns, byteView(slice));
}

QUuid
quuidNewV4()
{
  return QUuid::createUuid();
}

QUuid
quuidNewV5(const QUuid& ns, ::rust::Slice<const ::std::uint8_t> slice)
{
  return QUuid::createUuidV5(ns, byteView(slice));
}

QString
quuidToString(const QUuid& uuid)
{
  return uuid.toString();
}

QUuid
quuidFromString(QAnyStringView string)
{
#if QT_VERSION >= QT_VERSION_CHECK(6, 8, 0)
  return QUuid::fromString(string); // we can use UTF8 strings directly
#else
  return QUuid::fromString(QString(string));
#endif
}

}
}
