# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(example_basic_cxx_only)
set(APP_NAME ${PROJECT_NAME})

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../../cmake")
include(CxxQt)

# TODO: at some point we want to transform cxx-qt-build into cxx-cmake-build
# to clarify that qt support is optional. At this point it would make more
# sense to never generate files with Qt code rather than to #ifdef it out using
# this flag as we currently do for cases where the application doesn't link to Qt.
add_compile_definitions(NO_QT)

set(
    RUST_SOURCES
    lib.rs
)

set(
    CPP_SOURCES
    src/main.cpp
)

cxx_qt_cmake(${APP_NAME} "${RUST_SOURCES}" "${CPP_SOURCES}")
target_link_libraries(${APP_NAME} doctest)