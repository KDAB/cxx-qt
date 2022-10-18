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
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner QuickTest Test REQUIRED)
endif()
get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)

add_subdirectory(plugin)
add_subdirectory(tests)

add_executable(${APP_NAME} cpp/main.cpp qml/qml.qrc)
target_link_libraries(${APP_NAME} PRIVATE
    Qt::Core
    Qt::Gui
    Qt::Qml
    Qt::QuickControls2
    Qt::Test
)
qt_import_qml_plugins(${APP_NAME})

add_dependencies(${APP_NAME} core_qmlplugin)
