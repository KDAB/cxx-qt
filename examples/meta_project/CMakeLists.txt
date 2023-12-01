# SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

project(example_meta_project)
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

find_package(Corrosion QUIET)
if(NOT Corrosion_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.4.2
    )

    FetchContent_MakeAvailable(Corrosion)
endif()

set(CRATE qml-meta-project)
corrosion_import_crate(MANIFEST_PATH rust/main/Cargo.toml CRATES ${CRATE} FLAGS -j1)
set(CXXQT_EXPORT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxxqt")
corrosion_set_env_vars(${CRATE}
    "CXXQT_EXPORT_DIR=${CXXQT_EXPORT_DIR}"
    "QMAKE=${QMAKE}"
)
add_library(${APP_NAME}_lib INTERFACE)
target_include_directories(${APP_NAME}_lib INTERFACE "${CXXQT_EXPORT_DIR}/${CRATE}")
target_link_libraries(${APP_NAME}_lib INTERFACE
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,${CRATE}-static>"
    Qt::Core
    Qt::Gui
    Qt::Qml
    Qt::QuickControls2
)

add_executable(${APP_NAME}
    cpp/main.cpp
)

target_include_directories(${APP_NAME} PRIVATE cpp)
target_link_libraries(${APP_NAME} PRIVATE ${APP_NAME}_lib)
qt_import_qml_plugins(${APP_NAME})
