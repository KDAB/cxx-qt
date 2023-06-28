# SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

project(example_meta_project)

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

set(CXXQT_QTCOMPONENTS Core Gui Qml QuickControls2 QuickTest Test)
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

cxx_qt_import_crate(MANIFEST_PATH rust/main/Cargo.toml CRATES qml_meta_project)
target_link_libraries(qml_meta_project INTERFACE Qt::Core Qt::Gui Qt::Qml Qt::QuickControls2)

cxx_qt_import_qml_module(qml_meta_project_main
    URI "com.kdab.cxx_qt.demo"
    SOURCE_CRATE qml_meta_project)

cxx_qt_import_qml_module(qml_meta_project_sub1
    URI "com.kdab.cxx_qt.demo.sub1"
    SOURCE_CRATE qml_meta_project)

cxx_qt_import_qml_module(qml_meta_project_sub2
    URI "com.kdab.cxx_qt.demo.sub2"
    SOURCE_CRATE qml_meta_project)

# Define the executable with the C++ source
if(BUILD_WASM)
    # Currently need to use qt_add_executable
    # for WASM builds, otherwise there is no
    # HTML output.
    #
    # TODO: Figure out how to configure such that
    #       we can use add_executable for WASM
    qt_add_executable(example_meta_project cpp/main.cpp qml/qml.qrc)
else()
    add_executable(example_meta_project cpp/main.cpp qml/qml.qrc)
endif()

# Link to the qml module, which in turn links to the Rust qml_meta_project library
target_link_libraries(example_meta_project PRIVATE Qt::Core Qt::Gui Qt::Qml qml_meta_project_main qml_meta_project_sub1 qml_meta_project_sub2)

# If we are using a statically linked Qt then we need to import any qml plugins
qt_import_qml_plugins(example_meta_project)
