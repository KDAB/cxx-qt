# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

project(demo_threading)
set(APP_NAME ${PROJECT_NAME})

# Rust always links against non-debug Windows runtime on *-msvc targets
# Note it is best to set this on the command line to ensure all targets are consistent
# https://github.com/corrosion-rs/corrosion/blob/master/doc/src/common_issues.md#linking-debug-cc-libraries-into-rust-fails-on-windows-msvc-targets
# https://github.com/rust-lang/rust/issues/39016
if (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
endif()

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner REQUIRED)
endif()
get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)

find_package(Corrosion QUIET)
if(NOT Corrosion_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.5.0
    )

    FetchContent_MakeAvailable(Corrosion)
endif()

set(CRATE cxx_qt_demo_threading)
corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})
set(CXXQT_EXPORT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxxqt")
corrosion_set_env_vars(${CRATE}
    "CXXQT_EXPORT_DIR=${CXXQT_EXPORT_DIR}"
    "QMAKE=${QMAKE}"
    $<$<BOOL:${CMAKE_RUSTC_WRAPPER}>:RUSTC_WRAPPER=${CMAKE_RUSTC_WRAPPER}>
)

add_library(${APP_NAME}_lib INTERFACE)
target_include_directories(${APP_NAME}_lib INTERFACE "${CXXQT_EXPORT_DIR}/${CRATE}")
target_link_directories(${APP_NAME}_lib INTERFACE "${CXXQT_EXPORT_DIR}/${CRATE}")
target_link_libraries(${APP_NAME}_lib INTERFACE
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,${CRATE}-initializers>"
    ${CRATE}
    Qt::Core
    Qt::Gui
    Qt::Qml
    Qt::QuickControls2
)

# Qt Graphical Effects imports changed in Qt 6 so provide proxies
if(Qt5_FOUND)
    set(QML_COMPAT_RESOURCES qml/compat/compat_qt5.qrc)
else()
    set(QML_COMPAT_RESOURCES qml/compat/compat_qt6.qrc)
endif()

add_executable(${APP_NAME}
    cpp/helpers/energyusageproxymodel.h
    cpp/helpers/energyusageproxymodel.cpp
    cpp/main.cpp
    cpp/helpers/sensor.h
    cpp/helpers/sensor.cpp
    images/images.qrc
    ${QML_COMPAT_RESOURCES}
)
target_link_libraries(${APP_NAME} PRIVATE ${APP_NAME}_lib)
qt_import_qml_plugins(${APP_NAME})
