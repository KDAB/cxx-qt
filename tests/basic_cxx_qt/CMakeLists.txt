# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

project(tests_basic_cxx_qt)
set(APP_NAME ${PROJECT_NAME})

# Rust always links against non-debug Windows runtime on *-msvc targets
# Note it is best to set this on the command line to ensure all targets are consistent
# https://github.com/corrosion-rs/corrosion/blob/master/doc/src/common_issues.md#linking-debug-cc-libraries-into-rust-fails-on-windows-msvc-targets
# https://github.com/rust-lang/rust/issues/39016
if (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
endif()

# TODO: Add a helper function to our CMake module which automatically
# handles some of this boilerplate for a "typical" Qt application
set(CMAKE_AUTOMOC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Qml Test)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Qml Test REQUIRED)
endif()

set(CRATE basic_cxx_qt)
cxx_qt_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})

add_executable(${APP_NAME} cpp/main.cpp)
target_link_libraries(${APP_NAME} PRIVATE
    ${CRATE}
    Qt::Core
    Qt::Gui
    Qt::Qml
    Qt::QuickControls2
    Qt::Test)
