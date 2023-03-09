# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
# SPDX-FileContributor: Be <be.0@gmx.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Debug")
endif()

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

option(USE_QT5 "Use Qt5 even if Qt6 found" OFF)
if(WIN32 OR APPLE)
    option(VCPKG "Use vcpkg for dependencies" ON)
else()
    option(VCPKG "Use vcpkg for dependencies" OFF)
endif()

if(VCPKG)
    if(USE_QT5)
        set(VCPKG_MANIFEST_FEATURES "qt5")
    else()
        set(VCPKG_MANIFEST_FEATURES "qt6")
    endif()

    include(InitializeVcpkg)

    # These are required for binary caching to work reliably across machines.
    set(VCPKG_FEATURE_FLAGS "-compilertracking")
    set(VCPKG_INSTALL_OPTIONS "--x-abi-tools-use-exact-versions")

    if(NOT DEFINED ENV{VCPKG_BINARY_SOURCES})
        if(WIN32)
            set(COMMAND_PREFIX "")
            set(EXE_SUFFIX ".exe")
            set(SCRIPT_SUFFIX ".bat")
            set(DOTNET_RUNTIME "")
        else()
            set(COMMAND_PREFIX "./")
            set(EXE_SUFFIX "")
            set(SCRIPT_SUFFIX ".sh")
            set(DOTNET_RUNTIME "mono")
        endif()

        # vcpkg can download NuGet, so bootstrap vcpkg if the executable is not found.
        if(NOT EXISTS "${VCPKG_ROOT}/vcpkg${EXE_SUFFIX}")
        message(STATUS "Bootstrapping vcpkg")
        execute_process(
            COMMAND "${COMMAND_PREFIX}bootstrap-vcpkg${SCRIPT_SUFFIX}"
            WORKING_DIRECTORY ${VCPKG_ROOT}
        )
        endif()

        message(STATUS "Setting up vcpkg binary caching with read-only access to GitHub Packages NuGet source")

        execute_process(
            COMMAND "${COMMAND_PREFIX}vcpkg${EXE_SUFFIX}" fetch nuget
            WORKING_DIRECTORY ${VCPKG_ROOT}
            OUTPUT_VARIABLE NUGET_FETCH_OUTPUT
            OUTPUT_STRIP_TRAILING_WHITESPACE
        )
        string(REPLACE "\n" ";" NUGET_FETCH_OUTPUT "${NUGET_FETCH_OUTPUT}")
        list(POP_BACK NUGET_FETCH_OUTPUT NUGET_EXECUTABLE)

        # NuGet will fail with an error when trying to add a source with the same name
        # as one that already exists, so check that the NuGet source has not been added yet.
        execute_process(
            COMMAND ${DOTNET_RUNTIME} ${NUGET_EXECUTABLE} sources list
            OUTPUT_VARIABLE NUGET_SOURCES_LIST
        )
        string(FIND "${NUGET_SOURCES_LIST}" "cxx-qt-github-packages" SEARCH_RESULT)
        if(SEARCH_RESULT EQUAL -1)
        # GitHub will deactivate a personal access token that gets committed to the repository.
        # Hack around this by splitting up the PAT.
        # This is safe because this PAT only has read:packages permission.
        set(GITHUB_PAT_READ_PACKAGES_SUFFIX HvVJ7NF8sArqcyBnF45RXOgAT0Q1uL42CZkO)
        execute_process(
            COMMAND ${DOTNET_RUNTIME} ${NUGET_EXECUTABLE} sources add
            -name cxx-qt-github-packages
            -source https://nuget.pkg.github.com/KDAB/index.json
            -username KDAB
            -password ghp_${GITHUB_PAT_READ_PACKAGES_SUFFIX}
        )
        endif()

        set(ENV{VCPKG_BINARY_SOURCES} "clear;default,readwrite;nuget,cxx-qt-github-packages,read;")
    endif()
else()
    message(STATUS "Using dependencies from system without vcpkg")
endif()

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

if(UNIX AND NOT APPLE)
    add_compile_definitions(
        QT_STRICT_ITERATORS
    )
endif()

find_program(MEMORYCHECK_COMMAND valgrind)
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
    add_test(NAME ${NAME_WITH_PREFIX}_valgrind
        COMMAND ${MEMORYCHECK_COMMAND} ${MEMORYCHECK_COMMAND_OPTIONS} --suppressions=${MEMORYCHECK_SUPPRESSIONS_FILE} --gen-suppressions=all ${BINARY}
        WORKING_DIRECTORY "${WORKING_DIRECTORY}"
    )
endfunction()

add_subdirectory(book)
add_subdirectory(examples)

# QMAKE environment variable is needed by qt-build-utils to ensure that Cargo
# uses the same installation of Qt as CMake does.
if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Test)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Test REQUIRED)
endif()
get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)
set(CARGO_ENV "QMAKE=${QMAKE}")

# Create helper method which adds relevent cargo tests for a given manifest
function(add_test_cargo TEST_NAME_PREFIX MANIFEST_PATH ADD_DOCTESTS)
    # Add cargo as a test
    add_test(NAME ${TEST_NAME_PREFIX}_cargo_tests COMMAND cargo test --all-features --all-targets --manifest-path ${MANIFEST_PATH})
    set_property(TEST ${TEST_NAME_PREFIX}_cargo_tests PROPERTY ENVIRONMENT ${CARGO_ENV})
    # Check if we should enable doc tests
    if (${ADD_DOCTESTS} STREQUAL "DOCTESTS_ON")
        # Add cargo docs as a test
        add_test(NAME ${TEST_NAME_PREFIX}_cargo_doc_tests COMMAND cargo test --all-features --doc --manifest-path ${MANIFEST_PATH})
        set_property(TEST ${TEST_NAME_PREFIX}_cargo_doc_tests PROPERTY ENVIRONMENT ${CARGO_ENV})
    endif()
    # Add clippy as a test
    add_test(NAME ${TEST_NAME_PREFIX}_cargo_clippy COMMAND cargo clippy --all-features --all-targets --manifest-path ${MANIFEST_PATH} -- -D warnings)
    set_property(TEST ${TEST_NAME_PREFIX}_cargo_clippy PROPERTY ENVIRONMENT ${CARGO_ENV})
    # Add rustfmt as a test
    add_test(NAME ${TEST_NAME_PREFIX}_cargo_fmt COMMAND cargo fmt --manifest-path ${MANIFEST_PATH} -- --check)
    set_property(TEST ${TEST_NAME_PREFIX}_cargo_fmt PROPERTY ENVIRONMENT ${CARGO_ENV})
endfunction()

# Add cargo tests for all our manifests
#
# Note doctests are not supported on the staticlib in root
add_test_cargo(cxx_qt "${CMAKE_CURRENT_SOURCE_DIR}/crates/cxx-qt/Cargo.toml" DOCTESTS_ON)
add_test_cargo(cxx_qt_build "${CMAKE_CURRENT_SOURCE_DIR}/crates/cxx-qt-build/Cargo.toml" DOCTESTS_ON)
add_test_cargo(cxx_qt_gen "${CMAKE_CURRENT_SOURCE_DIR}/crates/cxx-qt-gen/Cargo.toml" DOCTESTS_ON)
add_test_cargo(cxx_qt_lib "${CMAKE_CURRENT_SOURCE_DIR}/crates/cxx-qt-lib/Cargo.toml" DOCTESTS_ON)
add_test_cargo(cxx_qt_lib_headers "${CMAKE_CURRENT_SOURCE_DIR}/crates/cxx-qt-lib-headers/Cargo.toml" DOCTESTS_ON)

# Ensure test inputs and outputs are formatted
file(GLOB CXX_QT_GEN_TEST_INPUTS ${CMAKE_CURRENT_SOURCE_DIR}/crates/cxx-qt-gen/test_inputs/*.rs)
file(GLOB CXX_QT_GEN_TEST_OUTPUTS ${CMAKE_CURRENT_SOURCE_DIR}/crates/cxx-qt-gen/test_outputs/*.rs)
add_test(NAME cxx_qt_gen_test_inputs_gen COMMAND rustfmt --check ${CXX_QT_GEN_TEST_INPUTS})
add_test(NAME cxx_qt_gen_test_outputs_gen COMMAND rustfmt --check ${CXX_QT_GEN_TEST_OUTPUTS})

# QML example has add_test in it's CMakeLists, so just add the cargo tests here
add_test_cargo(qt-build-utils "${CMAKE_CURRENT_SOURCE_DIR}/crates/qt-build-utils/Cargo.toml" DOCTESTS_ON)
add_test_cargo(demo_threading "${CMAKE_CURRENT_SOURCE_DIR}/examples/demo_threading/rust/Cargo.toml" DOCTESTS_OFF)
add_test_cargo(qml_features "${CMAKE_CURRENT_SOURCE_DIR}/examples/qml_features/rust/Cargo.toml" DOCTESTS_OFF)
add_test_cargo(qml_extension_plugin "${CMAKE_CURRENT_SOURCE_DIR}/examples/qml_extension_plugin/plugin/rust/Cargo.toml" DOCTESTS_OFF)
add_test_cargo(qml_minimal "${CMAKE_CURRENT_SOURCE_DIR}/examples/qml_minimal/rust/Cargo.toml" DOCTESTS_OFF)

# Add test which scans for all .cpp and .h files in this project and runs clang-format
add_test(NAME cpp_clang_format COMMAND "${CMAKE_CURRENT_SOURCE_DIR}/clang_format_check.sh" "${CMAKE_CURRENT_SOURCE_DIR}")

# Add test which checks all files have a valid license
add_test(NAME reuse_lint COMMAND reuse lint)

add_subdirectory(tests)
