# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(example_basic_cxx_qt_qml_plugin)
set(APP_NAME ${PROJECT_NAME})

# TODO: Add a helper function to our CMake module which automatically
# handles some of this boilerplate for a "typical" Qt application
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Gui Qml QuickControls2 QuickTest Test REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Qml QuickControls2 QuickTest Test REQUIRED)

#
# QML plugin
#

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../../cmake")
include(CxxQt)

set(
    RUST_SOURCES
    lib.rs
    sub.rs
)

set(QML_IMPORT_DIR ${CMAKE_CURRENT_BINARY_DIR}/qml)
# TODO: generate this path and name from the build.rs then our cmake helpers can read this from .txt
set(PLUGIN_OUTPUT_DIR ${QML_IMPORT_DIR}/com/kdab/cxx_qt/demo)

# Generate the C++ code
cxx_qt_generate_cpp("${RUST_SOURCES}" GEN_SOURCES)

# Define our executable with our QML resource files
add_library(${APP_NAME}_qmlplugin SHARED "${GEN_SOURCES}")
# Set directory for our library
set_target_properties(${APP_NAME}_qmlplugin PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${PLUGIN_OUTPUT_DIR}
)

# Include generated sources
cxx_qt_include(${APP_NAME}_qmlplugin)
# Link to generated rust library
cxx_qt_link_rustlib(${APP_NAME}_qmlplugin)

target_link_libraries(${APP_NAME}_qmlplugin Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Qml Qt${QT_VERSION_MAJOR}::QuickControls2 Qt${QT_VERSION_MAJOR}::Test)

configure_file(${CMAKE_CURRENT_SOURCE_DIR}/target/cxx-qt-gen/plugin/qmldir ${PLUGIN_OUTPUT_DIR}/qmldir COPYONLY)

#
# App
#

set(
    CPP_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp
)

set(
    RESOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/qml.qrc
)

add_executable(${APP_NAME} "${CPP_SOURCES}" "${RESOURCES}")
target_link_libraries(${APP_NAME} ${APP_NAME}_qmlplugin Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Qml Qt${QT_VERSION_MAJOR}::QuickControls2 Qt${QT_VERSION_MAJOR}::Test)

#
# Unit test
#

find_program(QMLTESTRUNNER_BIN qmltestrunner)
if(QMLTESTRUNNER_BIN)
    message(STATUS "qmltestrunner found at: ${QMLTESTRUNNER_BIN}")
else()
    message(CRITICAL "qmltestrunner not found: tests disabled")
endif()

add_test(NAME ${APP_NAME}_test
    COMMAND ${QMLTESTRUNNER_BIN} -import ${QML_IMPORT_DIR} -input ${CMAKE_CURRENT_SOURCE_DIR}/src/tests
    WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
)

# TODO: test needs to depend on the app ? which depends on the plugin?
