# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
# SPDX-FileContributor: Be <be.0@gmx.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

option(USE_QT5 "Use Qt5 even if Qt6 found" OFF)

option(BUILD_TESTING "Build the tests for CXX-Qt" ON)

project(cxx_qt)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(CompilerCaching)

# Enable extra Qt definitions for all projects
add_compile_definitions(
    QT_NO_CAST_FROM_ASCII
    QT_NO_CAST_TO_ASCII
    QT_NO_CAST_FROM_BYTEARRAY
    QT_NO_URL_CAST_FROM_STRING
    QT_NO_NARROWING_CONVERSIONS_IN_CONNECT
    QT_NO_FOREACH
    QT_NO_JAVA_STYLE_ITERATORS
    QT_NO_KEYWORDS
    QT_USE_QSTRINGBUILDER
)

if(BUILD_WASM)
    # TODO: fix wasm test failures
    set(BUILD_TESTING OFF)
    # Ensure Rust build for the correct target
    set(Rust_CARGO_TARGET wasm32-unknown-emscripten)
    set(THREADS_PREFER_PTHREAD_FLAG ON)
    find_package(Threads REQUIRED)
endif()

# QMAKE environment variable is needed by qt-build-utils to ensure that Cargo
# uses the same installation of Qt as CMake does.
if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Test QuickControls2)
endif()
if(NOT Qt6_FOUND)
    if(BUILD_WASM)
        message(FATAL_ERROR 
            "CXX-Qt for WebAssembly only currently supports Qt 6 builds."
        )
    endif()
    find_package(Qt5 5.15 COMPONENTS Core Gui Test REQUIRED)
endif()

if (NOT Qt5_FOUND)
    add_definitions(-DQT_NO_CONTEXTLESS_CONNECT)
endif()

if(UNIX AND NOT APPLE)
    if (Qt5_FOUND)
        add_compile_definitions(
            QT_STRICT_ITERATORS
        )
    endif()
endif()

find_program(MEMORYCHECK_COMMAND valgrind)
if(NOT "${MEMORYCHECK_COMMAND}" STREQUAL "MEMORYCHECK_COMMAND-NOTFOUND")
    if (NOT WIN32)
       MESSAGE(STATUS "Valgrind found! Tests based on valgrind must be executed.")
    endif()
endif()

# Set our extra command options for valgrind
# TODO: we need to come up with a better way to suppress "possibly lost" errors.
# Suppression file doesn't work because there is a ton of mangled names that won't remain stable.
set(MEMORYCHECK_COMMAND_OPTIONS --error-exitcode=1 --errors-for-leak-kinds=definite --leak-check=full --trace-children=yes --track-origins=yes --show-possibly-lost=no)
# A suppressions file which silences errors from other libs like QtCore
set(MEMORYCHECK_SUPPRESSIONS_FILE "${CMAKE_SOURCE_DIR}/valgrind_suppressions.txt")

# Enable testing (this needs to be called before subdirs are added to detect tests in them)
enable_testing()

# Create helper method which adds a valgrind test with the given binary
function(add_valgrind_test NAME_WITH_PREFIX BINARY WORKING_DIRECTORY)
    if("${MEMORYCHECK_COMMAND}" STREQUAL "MEMORYCHECK_COMMAND-NOTFOUND")
       if (NOT WIN32)
           MESSAGE(STATUS "valgrind not found. Please install it")
       endif()
    else()
       add_test(NAME ${NAME_WITH_PREFIX}_valgrind
           COMMAND ${MEMORYCHECK_COMMAND} ${MEMORYCHECK_COMMAND_OPTIONS} --suppressions=${MEMORYCHECK_SUPPRESSIONS_FILE} --gen-suppressions=all ${BINARY}
           WORKING_DIRECTORY "${WORKING_DIRECTORY}"
       )
    endif()
endfunction()

get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)
set(CARGO_ENV "QMAKE=set:${QMAKE}")
set(RUNTIME_ENV "")

# On windows, Qt dll needs to be in the PATH for the tests to run
if(CMAKE_SYSTEM_NAME STREQUAL "Windows" AND BUILD_TESTING)
    execute_process(
        COMMAND ${QMAKE} -query QT_INSTALL_BINS
        OUTPUT_VARIABLE QT_INSTALL_BINS
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )

    execute_process(
        COMMAND ${QMAKE} -query QT_INSTALL_PLUGINS
        OUTPUT_VARIABLE QT_INSTALL_PLUGINS
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )

    execute_process(
        COMMAND ${QMAKE} -query QT_INSTALL_QML
        OUTPUT_VARIABLE QT_INSTALL_QML
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )

    list(
        APPEND
        RUNTIME_ENV
        "PATH=path_list_append:${QT_INSTALL_BINS}"
        "QT_PLUGIN_PATH=path_list_append:${QT_INSTALL_PLUGINS}"
        "QML_IMPORT_PATH=path_list_append:${QT_INSTALL_QML}"
        "QML2_IMPORT_PATH=path_list_append:${QT_INSTALL_QML}"
    )
    list(APPEND CARGO_ENV ${RUNTIME_ENV})
endif()

# Same logic as in Corrosion.cmake
if(CMAKE_VS_PLATFORM_NAME)
    set(BUILD_DIR "${CMAKE_VS_PLATFORM_NAME}/$<CONFIG>")
elseif(CMAKE_CONFIGURATION_TYPES)
    set(BUILD_DIR "$<CONFIG>")
else()
    set(BUILD_DIR .)
endif()

# Set the target dir to the same that Corrosion uses to reuse build artifacts
# from the main build.
set(CARGO_TARGET_DIR "${CMAKE_BINARY_DIR}/${BUILD_DIR}/cargo/build")

if(BUILD_TESTING)
    # Add CMake tests for `cargo test/clippy/fmt/doc`.
    add_test(NAME cargo_tests COMMAND cargo test --locked --release --all-features --target-dir ${CARGO_TARGET_DIR})
    add_test(NAME cargo_doc COMMAND cargo doc --locked --release --all-features --target-dir ${CARGO_TARGET_DIR})
    add_test(NAME cargo_clippy COMMAND cargo clippy --locked --release --all-features --target-dir ${CARGO_TARGET_DIR} -- -D warnings)

    set_tests_properties(cargo_tests cargo_clippy PROPERTIES
        ENVIRONMENT_MODIFICATION "${CARGO_ENV}"
    )
    set_tests_properties(cargo_doc PROPERTIES
        ENVIRONMENT_MODIFICATION "${CARGO_ENV};RUSTDOCFLAGS=set:--deny=warnings"
    )
    
    # Ensure test inputs and outputs are formatted
    file(GLOB CXX_QT_GEN_TEST_INPUTS ${CMAKE_CURRENT_SOURCE_DIR}/crates/cxx-qt-gen/test_inputs/*.rs)
    file(GLOB CXX_QT_GEN_TEST_OUTPUTS ${CMAKE_CURRENT_SOURCE_DIR}/crates/cxx-qt-gen/test_outputs/*.rs)
    add_test(NAME cxx_qt_gen_test_inputs_gen COMMAND rustfmt --check ${CXX_QT_GEN_TEST_INPUTS})
    add_test(NAME cxx_qt_gen_test_outputs_gen COMMAND rustfmt --check ${CXX_QT_GEN_TEST_OUTPUTS})
    
    # Add test which checks that a build rerun doesn't recompile and uses caches instead
    add_test(NAME cargo_build_rerun COMMAND "${CMAKE_CURRENT_SOURCE_DIR}/scripts/check_cargo_build_rerun.sh" "${CMAKE_CURRENT_SOURCE_DIR}" "${CARGO_TARGET_DIR}")
    
    # Ensure that cargo_build_rerun doesn't run while we are already building
    set_tests_properties(cargo_build_rerun PROPERTIES RUN_SERIAL TRUE)

    # As we do not have cargo_without_cmake under cmake anymore
    # instead manually have a test for qmllint here
    #
    # qmllint --max-warnings only exists in Qt 6
    if(Qt6_FOUND AND Qt6_VERSION VERSION_GREATER_EQUAL 6.8.0)
        find_program(QMLLINT_COMMAND qmllint PATHS "${QT_INSTALL_BINS}")
        if("${QMLLINT_COMMAND}" STREQUAL "QMLLINT_COMMAND-NOTFOUND")
            MESSAGE(STATUS "qmllint not found. Please install it. Skipping qmllint tests.")
        else()
            file(GLOB QMLLINT_QML_FILES ${CMAKE_CURRENT_SOURCE_DIR}/examples/cargo_without_cmake/qml/*.qml)
            add_test(NAME example_cargo_without_cmake_qmllint_check COMMAND ${QMLLINT_COMMAND} --max-warnings 0 -I ${CARGO_TARGET_DIR}/cxxqt/qml_modules ${QMLLINT_QML_FILES})

            # Add a fixture to so that this test can depend on cargo build being run before
            add_test(NAME example_cargo_without_cmake_qmllint_check_fixture COMMAND cargo build -p qml-minimal-no-cmake --locked --release --all-features --target-dir ${CARGO_TARGET_DIR})
            set_tests_properties(example_cargo_without_cmake_qmllint_check_fixture PROPERTIES FIXTURES_SETUP fixture_cargo_without_cmake_build)
            set_tests_properties(example_cargo_without_cmake_qmllint_check PROPERTIES FIXTURES_REQUIRED fixture_cargo_without_cmake_build)

            # Ensure that other tests are not running while building
            set_tests_properties(example_cargo_without_cmake_qmllint_check_fixture PROPERTIES RUN_SERIAL TRUE)
        endif()
    endif()

endif()

if(CMAKE_RUSTC_WRAPPER)
    list(APPEND CARGO_ENV "RUSTC_WRAPPER=set:${CMAKE_RUSTC_WRAPPER}")
endif()

add_subdirectory(book)
add_subdirectory(examples)

if(BUILD_TESTING)
    add_subdirectory(tests)
endif()
