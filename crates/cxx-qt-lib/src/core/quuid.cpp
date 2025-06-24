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

static_assert(QTypeInfo<QUuid>::isRelocatable, "QUuid must be relocatable!");

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
quuidCreateUuidV3(const QUuid& ns, ::rust::Slice<const ::std::uint8_t> slice)
{
  return QUuid::createUuidV3(ns, byteView(slice));
}

QUuid
quuidCreateUuid()
{
  return QUuid::createUuid();
}

QUuid
quuidCreateUuidV5(const QUuid& ns, ::rust::Slice<const ::std::uint8_t> slice)
{
  return QUuid::createUuidV5(ns, byteView(slice));
}

QString
quuidToString(const QUuid& uuid)
{
  return uuid.toString();
}

QUuid
quuidFromString(const QString& string)
{
  return QUuid::fromString(string);
}

QUuid
quuidFromStr(rust::Str string)
{
#if QT_VERSION >= QT_VERSION_CHECK(6, 8, 0)
  return QUuid::fromString(QAnyStringView(string.data(), string.length()));
#else
  return QUuid::fromString(QString::fromLatin1(string.data(), string.length()));
#endif
}

QUuid
quuidFromRfc4122(const QByteArray& bytes)
{
  return QUuid::fromRfc4122(bytes);
}

QUuid::Variant
quuidVariant(const QUuid& uuid)
{
  return uuid.variant();
}

QUuid::Version
quuidVersion(const QUuid& uuid)
{
  return uuid.version();
}

}
}
