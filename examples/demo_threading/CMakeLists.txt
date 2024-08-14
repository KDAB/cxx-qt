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

if(BUILD_WASM)
    # Ensure Rust build for the correct target
    set(Rust_CARGO_TARGET wasm32-unknown-emscripten)
    set(THREADS_PREFER_PTHREAD_FLAG ON)
    find_package(Threads REQUIRED)
endif()

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CXXQT_QTCOMPONENTS Core Gui Qml QuickControls2)
if(NOT BUILD_WASM)
    set(CXXQT_QTCOMPONENTS ${CXXQT_QTCOMPONENTS} QmlImportScanner)
endif()

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS ${CXXQT_QTCOMPONENTS})
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS ${CXXQT_QTCOMPONENTS} REQUIRED)
endif()

find_package(CxxQt QUIET)
if(NOT CxxQt_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        CxxQt
        GIT_REPOSITORY https://github.com/kdab/cxx-qt-cmake.git
        GIT_TAG main
    )

    FetchContent_MakeAvailable(CxxQt)
endif()

set(CRATE cxx_qt_demo_threading)
cxx_qt_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})
cxx_qt_import_qml_module(${CRATE}_qml
    URI "com.kdab.energy"
    SOURCE_CRATE ${CRATE})

# Qt Graphical Effects imports changed in Qt 6 so provide proxies
if(Qt5_FOUND)
    set(QML_COMPAT_RESOURCES qml/compat/compat_qt5.qrc)
else()
    set(QML_COMPAT_RESOURCES qml/compat/compat_qt6.qrc)
endif()

set(DEMO_THREADING_SOURCES
    cpp/helpers/energyusageproxymodel.h
    cpp/helpers/energyusageproxymodel.cpp
    cpp/main.cpp
    cpp/helpers/sensor.h
    cpp/helpers/sensor.cpp
    images/images.qrc
    ${QML_COMPAT_RESOURCES}
)

if(BUILD_WASM)
    # Currently need to use qt_add_executable
    # for WASM builds, otherwise there is no
    # HTML output.
    #
    # TODO: Figure out how to configure such that
    #       we can use add_executable for WASM
    qt_add_executable(${APP_NAME} ${DEMO_THREADING_SOURCES})
else()
    add_executable(${APP_NAME} ${DEMO_THREADING_SOURCES})
endif()

target_link_libraries(${APP_NAME}
    PRIVATE
    ${CRATE}_qml
    Qt::Core
    Qt::Gui
    Qt::Qml
    Qt::QuickControls2)

qt_import_qml_plugins(${APP_NAME})
