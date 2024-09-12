# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

project(example_qml_features)
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

set(CRATE qml_features)
cxx_qt_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})
cxx_qt_import_qml_module(${CRATE}_qml
    URI "com.kdab.cxx_qt.demo"
    SOURCE_CRATE ${CRATE})

if(BUILD_WASM)
    # Currently need to use qt_add_executable
    # for WASM builds, otherwise there is no
    # HTML output.
    #
    # TODO: Figure out how to configure such that
    #       we can use add_executable for WASM
    qt_add_executable(${APP_NAME} cpp/main.cpp)
else()
    add_executable(${APP_NAME} cpp/main.cpp)
endif()

target_include_directories(${APP_NAME} PRIVATE cpp)
target_link_libraries(${APP_NAME}
    PRIVATE
    ${CRATE}_qml
    Qt::Core
    Qt::Gui
    Qt::Qml
    Qt::QuickControls2)

qt_import_qml_plugins(${APP_NAME})

if(BUILD_TESTING)
    #
    # Unit test
    #

    set(APP_TEST_NAME ${APP_NAME}_test)
    add_executable(${APP_TEST_NAME} tests/main.cpp)
    target_include_directories(${APP_TEST_NAME} PRIVATE cpp)
    target_link_libraries( ${APP_TEST_NAME}
        PRIVATE
        ${CRATE}_qml
        Qt::Test
        Qt::QuickTest
        Qt::Core
        Qt::Gui
        Qt::Qml
        Qt::QuickControls2
    )
    qt_import_qml_plugins(${APP_TEST_NAME})

    set(TEST_CMD
        $<TARGET_FILE:${APP_TEST_NAME}> -input ${CMAKE_CURRENT_SOURCE_DIR}/tests
    )

    add_test(
        NAME ${APP_TEST_NAME}
        COMMAND ${TEST_CMD}
    )
    # Unfortunately due to the static linking in our CI on macOS we can't load the
    # offscreen plugin, so just leave it at the default.
    if (NOT APPLE)
        set_tests_properties(${APP_TEST_NAME} PROPERTIES ENVIRONMENT "QT_QPA_PLATFORM=offscreen")
    endif()
    # RUNTIME_ENV comes from the CMakeLists.txt at the root of this repository.
    set_tests_properties(
        ${APP_TEST_NAME} PROPERTIES ENVIRONMENT_MODIFICATION "${RUNTIME_ENV}"
    )

    if (COMMAND add_valgrind_test)
        add_valgrind_test(
            ${APP_TEST_NAME} "${TEST_CMD}" ${CMAKE_CURRENT_BINARY_DIR}
        )
    else()
        MESSAGE(STATUS "add_valgrind_test is defined in the top level of CXX-Qt. It will not executed")
    endif()
endif()
