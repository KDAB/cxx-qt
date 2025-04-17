# SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

project(example_qml_multi_crates)

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
    set(Qt "Qt6")
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS ${CXXQT_QTCOMPONENTS} REQUIRED)
    set(Qt "Qt5")
endif()

if(MSVC)
    # Qt also needs to link against the non-debug version of the MSVC Runtime libraries, see the previous comment.
    # Note: The Qt:: targets are ALIAS targets that do not support setting properties directly.
    # We therefore need to resolve the target names to either Qt5 or Qt6 directly.
    set_property(
        TARGET ${Qt}::Core ${Qt}::Gui ${Qt}::Qml ${Qt}::QuickControls2 ${Qt}::QuickTest ${Qt}::Test ${Qt}::Network
        PROPERTY MAP_IMPORTED_CONFIG_DEBUG "RELEASE")
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

cxx_qt_import_crate(MANIFEST_PATH rust/main/Cargo.toml
    CRATES qml_multi_crates
    CRATE_TYPES staticlib
    LOCKED
    QT_MODULES Qt::Core Qt::Gui Qt::Qml Qt::QuickControls2 Qt::Network
)

cxx_qt_import_qml_module(qml_multi_crates_main
    URI "com.kdab.cxx_qt.demo"
    SOURCE_CRATE qml_multi_crates)

cxx_qt_import_qml_module(qml_multi_crates_sub1
    URI "com.kdab.cxx_qt.demo.sub1"
    SOURCE_CRATE qml_multi_crates)

cxx_qt_import_qml_module(qml_multi_crates_sub2
    URI "com.kdab.cxx_qt.demo.sub2"
    SOURCE_CRATE qml_multi_crates)

# Define the executable with the C++ source
if(BUILD_WASM)
    # Currently need to use qt_add_executable
    # for WASM builds, otherwise there is no
    # HTML output.
    #
    # TODO: Figure out how to configure such that
    #       we can use add_executable for WASM
    qt_add_executable(example_qml_multi_crates cpp/main.cpp)
else()
    add_executable(example_qml_multi_crates cpp/main.cpp)
endif()

# Link to the qml module, which in turn links to the Rust qml_multi_crates library
target_link_libraries(example_qml_multi_crates PRIVATE Qt::Core Qt::Gui Qt::Qml qml_multi_crates_main qml_multi_crates_sub1 qml_multi_crates_sub2)

# If we are using a statically linked Qt then we need to import any qml plugins
qt_import_qml_plugins(example_qml_multi_crates)
