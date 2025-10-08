// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <cstdint>

#include <QtQml/QQmlEngine>

// This is similar to the builtins file in qtdeclarative
// https://code.qt.io/cgit/qt/qtdeclarative.git/tree/src/qml/qqmlbuiltins_p.h?h=v6.9.3
//
// We need this to be able to alias namespaced std numerics to types that QML
// understands.
//
// We do not need to build this file but the moc JSON output to give to
// qmltyperegistrar so that qmllint and qmlls can understand the types.
//
// If Qt ever registered qualified versions of the numerics this could be
// removed.
//
// qqmlbuiltins uses the following values for QML_USING so we should copy
// i8, u8 -> qint8, quint8
// i16, u16 -> short, ushort
// i32, u32 -> int, uint
// i64, u64 -> qlonglong, qulonglong

struct QQmlCxxQtStdInt8TForeign
{
  Q_GADGET
  QML_FOREIGN(::std::int8_t)
  QML_USING(qint8)
};
static_assert(sizeof(::std::int8_t) == sizeof(qint8));

struct QQmlCxxQtStdUInt8TForeign
{
  Q_GADGET
  QML_FOREIGN(::std::uint8_t)
  QML_USING(quint8)
};
static_assert(sizeof(::std::uint8_t) == sizeof(quint8));

struct QQmlCxxQtStdInt16TForeign
{
  Q_GADGET
  QML_FOREIGN(::std::int16_t)
  QML_USING(short)
};
static_assert(sizeof(::std::int16_t) == sizeof(short));

struct QQmlCxxQtStdUInt16TForeign
{
  Q_GADGET
  QML_FOREIGN(::std::uint16_t)
  QML_USING(ushort)
};
static_assert(sizeof(::std::uint16_t) == sizeof(ushort));

struct QQmlCxxQtStdInt32TForeign
{
  Q_GADGET
  QML_FOREIGN(::std::int32_t)
  QML_USING(int)
};
static_assert(sizeof(::std::int32_t) == sizeof(int));

struct QQmlCxxQtStdUInt32TForeign
{
  Q_GADGET
  QML_FOREIGN(::std::uint32_t)
  QML_USING(uint)
};
static_assert(sizeof(::std::uint32_t) == sizeof(uint));

struct QQmlCxxQtStdInt64TForeign
{
  Q_GADGET
  QML_FOREIGN(::std::int64_t)
  QML_USING(qlonglong)
};
static_assert(sizeof(::std::int64_t) == sizeof(qlonglong));

struct QQmlCxxQtStdUInt64TForeign
{
  Q_GADGET
  QML_FOREIGN(::std::uint64_t)
  QML_USING(qulonglong)
};
static_assert(sizeof(::std::uint64_t) == sizeof(qulonglong));
