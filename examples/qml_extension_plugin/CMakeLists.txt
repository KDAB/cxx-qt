# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(example_qml_extension_plugin)
set(APP_NAME ${PROJECT_NAME})

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner QuickTest Test)
endif()
if(Qt6_FOUND)
    set(QT_VERSION_MAJOR 6)
else()
    find_package(Qt5 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner QuickTest Test REQUIRED)
    set(QT_VERSION_MAJOR 5)
endif()
get_target_property(QMAKE Qt${QT_VERSION_MAJOR}::qmake IMPORTED_LOCATION)

add_subdirectory(plugin)
add_subdirectory(tests)

add_executable(${APP_NAME} cpp/main.cpp qml/qml.qrc)
target_link_libraries(${APP_NAME} PRIVATE
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
    Qt${QT_VERSION_MAJOR}::Qml
    Qt${QT_VERSION_MAJOR}::QuickControls2
    Qt${QT_VERSION_MAJOR}::Test
)
qt_import_qml_plugins(${APP_NAME})

add_dependencies(${APP_NAME} core_qmlplugin)
