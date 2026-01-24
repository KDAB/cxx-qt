// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <QtGlobal>

// For versions less than Qt 6 we need to manually register the std numerics
#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))

#include <QtCore/QCoreApplication>
#include <QtCore/QMetaType>
#include <cstdint>

static void do_register_cxx_qt_core_types()
{
    // If we are using Qt 5 then register std numbers as a type for use in QML.
    //
    // See also:
    // https://github.com/rust-lang/rust/issues/108081
    // https://github.com/KDAB/cxx-qt/pull/598
    qRegisterMetaType<::std::int8_t>("::std::int8_t");
    qRegisterMetaType<::std::int16_t>("::std::int16_t");
    qRegisterMetaType<::std::int32_t>("::std::int32_t");
    qRegisterMetaType<::std::int64_t>("::std::int64_t");

    qRegisterMetaType<::std::uint8_t>("::std::uint8_t");
    qRegisterMetaType<::std::uint16_t>("::std::uint16_t");
    qRegisterMetaType<::std::uint32_t>("::std::uint32_t");
    qRegisterMetaType<::std::uint64_t>("::std::uint64_t");
}

// Use Q_COREAPP_STARTUP_FUNCTION to defer registration until QCoreApplication
// is created. This is Qt's recommended approach for type registration.
Q_COREAPP_STARTUP_FUNCTION(do_register_cxx_qt_core_types)

extern "C" bool
init_cxx_qt_core()
{
  // Registration is handled automatically via Q_COREAPP_STARTUP_FUNCTION
  // when QCoreApplication is constructed.
  return true;
}

#else

extern "C" bool
init_cxx_qt_core()
{
  // Only needed for Qt5
  return true;
}

#endif
