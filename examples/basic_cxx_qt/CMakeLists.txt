# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(example_basic_cxx_qt)
set(APP_NAME ${PROJECT_NAME})

# TODO: Add a helper function to our CMake module which automatically
# handles some of this boilerplate for a "typical" Qt application
set(CMAKE_AUTOMOC ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Test REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Test REQUIRED)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../../cmake")
include(CxxQt)

set(
    RUST_SOURCES
    lib.rs
)

set(
    CPP_SOURCES
    src/main.cpp
)

cxx_qt_cmake(${APP_NAME} ${RUST_SOURCES} ${CPP_SOURCES})
target_link_libraries(${APP_NAME} doctest Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Test)
