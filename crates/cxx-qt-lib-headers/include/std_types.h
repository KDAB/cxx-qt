// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <QtGlobal>

// For versions less than Qt 6 we need to manually register the std numerics
#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))
#include <QtCore/QMetaType>

#include <cstdint>

static const int i8 = qRegisterMetaType<::std::int8_t>("::std::int8_t");
static const int i16 = qRegisterMetaType<::std::int16_t>("::std::int16_t");
static const int i32 = qRegisterMetaType<::std::int32_t>("::std::int32_t");
static const int i64 = qRegisterMetaType<::std::int64_t>("::std::int64_t");

static const int u8 = qRegisterMetaType<::std::uint8_t>("::std::uint8_t");
static const int u16 = qRegisterMetaType<::std::uint16_t>("::std::uint16_t");
static const int u32 = qRegisterMetaType<::std::uint32_t>("::std::uint32_t");
static const int u64 = qRegisterMetaType<::std::uint64_t>("::std::uint64_t");
#endif
