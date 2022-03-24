# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(demo_threading)
set(APP_NAME ${PROJECT_NAME})

# TODO: Add a helper function to our CMake module which automatically
# handles some of this boilerplate for a "typical" Qt application
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Gui Qml QuickControls2 QuickTest Test REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Qml QuickControls2 QuickTest Test REQUIRED)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../../cmake")
include(CxxQt)

set(
    CPP_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp
)

set(
    RESOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/qml.qrc
)

# Generate the C++ code
cxx_qt_generate_cpp(GEN_SOURCES)

# Define our executable with our QML resource files
add_executable(${APP_NAME} "${CPP_SOURCES}" "${GEN_SOURCES}" "${RESOURCES}")

# Include generated sources
cxx_qt_include(${APP_NAME})

# Link to generated rust library
cxx_qt_link_rustlib(${APP_NAME})
target_link_libraries(${APP_NAME} PRIVATE
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
    Qt${QT_VERSION_MAJOR}::Qml
    Qt${QT_VERSION_MAJOR}::QuickControls2
    Qt${QT_VERSION_MAJOR}::Test
)
qt_import_qml_plugins(${APP_NAME})
