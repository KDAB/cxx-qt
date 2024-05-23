// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <QtGlobal>

#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))
#include <cstdint>

#include <QtCore/QMetaType>
#endif

void
cxx_qt_qt5_compat()
{
#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))
  // This registers std numbers as a type for use in QML

  qRegisterMetaType<::std::int8_t>("::std::int8_t");
  qRegisterMetaType<::std::int16_t>("::std::int16_t");
  qRegisterMetaType<::std::int32_t>("::std::int32_t");
  qRegisterMetaType<::std::int64_t>("::std::int64_t");

  qRegisterMetaType<::std::uint8_t>("::std::uint8_t");
  qRegisterMetaType<::std::uint16_t>("::std::uint16_t");
  qRegisterMetaType<::std::uint32_t>("::std::uint32_t");
  qRegisterMetaType<::std::uint64_t>("::std::uint64_t");
#endif
}
