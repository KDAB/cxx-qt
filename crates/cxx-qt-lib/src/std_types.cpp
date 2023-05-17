// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <QtGlobal>

// For versions less than Qt 6 we need to manually register the std numerics
#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))
#include <QtCore/QMetaType>

#include <cstdint>

namespace rust {
namespace cxxqtlib1 {

// Ensure that std int types are registered
// so that they can be used with QML in Qt 5

static const int register_i8 =
  qRegisterMetaType<::std::int8_t>("::std::int8_t");
static const int register_i16 =
  qRegisterMetaType<::std::int16_t>("::std::int16_t");
static const int register_i32 =
  qRegisterMetaType<::std::int32_t>("::std::int32_t");
static const int register_i64 =
  qRegisterMetaType<::std::int64_t>("::std::int64_t");

static const int register_u8 =
  qRegisterMetaType<::std::uint8_t>("::std::uint8_t");
static const int register_u16 =
  qRegisterMetaType<::std::uint16_t>("::std::uint16_t");
static const int register_u32 =
  qRegisterMetaType<::std::uint32_t>("::std::uint32_t");
static const int register_u64 =
  qRegisterMetaType<::std::uint64_t>("::std::uint64_t");

}
}
#endif
