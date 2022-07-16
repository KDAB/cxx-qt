# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
# SPDX-FileContributor: Be <be.0@gmx.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Debug")
endif()

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

option(QT_DEFAULT_MAJOR_VERSION "Major version of Qt to use (6 or 5)" 6)

if(WIN32 OR APPLE)
    option(VCPKG "Use vcpkg for dependencies" ON)
else()
    option(VCPKG "Use vcpkg for dependencies" OFF)
endif()

if(VCPKG)
    # Explicitly setting QT_DEFAULT_MAJOR_VERSION can fail on systems that have both Qt6
    # and Qt5 installed, so only set it automatically when using vcpkg.
    if((NOT QT_DEFAULT_MAJOR_VERSION EQUAL 5) AND (NOT QT_DEFAULT_MAJOR_VERSION EQUAL 6))
        set(QT_DEFAULT_MAJOR_VERSION 6)
    endif()

    set(VCPKG_MANIFEST_FEATURES "qt${QT_DEFAULT_MAJOR_VERSION}")

    include(InitializeVcpkg)

    # These are required for binary caching to work reliably across machines.
    set(VCPKG_FEATURE_FLAGS "-compilertracking")
    set(VCPKG_INSTALL_OPTIONS "--x-abi-tools-use-exact-versions")

    if(NOT DEFINED ENV{VCPKG_BINARY_SOURCES})
        if(WIN32)
            set(COMMAND_PREIFX "")
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
        set(GITHUB_PAT_READ_PACKAGES_SUFFIX U2OpRinygfPcWI110P7CJOKLpomyGh2VeZ60)
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

# Define doctest as a library that the examples can link to in order to run their internal tests
add_library(doctest INTERFACE)
target_include_directories(doctest INTERFACE ${CMAKE_SOURCE_DIR}/doctest)

# Create helper method which adds a valgrind test with the given binary
function(add_valgrind_test NAME_WITH_PREFIX BINARY WORKING_DIRECTORY)
    add_test(NAME ${NAME_WITH_PREFIX}_valgrind
        COMMAND ${MEMORYCHECK_COMMAND} ${MEMORYCHECK_COMMAND_OPTIONS} --suppressions=${MEMORYCHECK_SUPPRESSIONS_FILE} --gen-suppressions=all ${BINARY}
        WORKING_DIRECTORY "${WORKING_DIRECTORY}"
    )
endfunction()

add_subdirectory(book)
add_subdirectory(examples)

# Create helper method which adds relevent cargo tests for a given manifest
function(add_test_cargo TEST_NAME_PREFIX MANIFEST_PATH ADD_DOCTESTS)
    # Add cargo as a test
    add_test(NAME ${TEST_NAME_PREFIX}_cargo_tests COMMAND cargo test --all-targets --manifest-path ${MANIFEST_PATH})
    # Check if we should enable doc tests
    if (${ADD_DOCTESTS} STREQUAL "DOCTESTS_ON")
        # Add cargo docs as a test
        add_test(NAME ${TEST_NAME_PREFIX}_cargo_doc_tests COMMAND cargo test --doc --manifest-path ${MANIFEST_PATH})
    endif()
    # Add clippy as a test
    add_test(NAME ${TEST_NAME_PREFIX}_cargo_clippy COMMAND cargo clippy --all-targets --manifest-path ${MANIFEST_PATH} -- -D warnings)
    # Add rustfmt as a test
    add_test(NAME ${TEST_NAME_PREFIX}_cargo_fmt COMMAND cargo fmt --manifest-path ${MANIFEST_PATH} -- --check)
endfunction()

# Add cargo tests for all our manifests
#
# Note doctests are not supported on the staticlib in root
add_test_cargo(cxx_qt "${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt/Cargo.toml" DOCTESTS_ON)
add_test_cargo(cxx_qt_build "${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt-build/Cargo.toml" DOCTESTS_ON)
add_test_cargo(cxx_qt_gen "${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt-gen/Cargo.toml" DOCTESTS_ON)
add_test_cargo(cxx_qt_lib "${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt-lib/Cargo.toml" DOCTESTS_ON)
add_test_cargo(cxx_qt_cli "${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt-cli/Cargo.toml" DOCTESTS_OFF)

# Ensure test inputs and outputs are formatted
file(GLOB CXX_QT_GEN_TEST_INPUTS ${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt-gen/test_inputs/*.rs)
file(GLOB CXX_QT_GEN_TEST_OUTPUTS ${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt-gen/test_outputs/*.rs)
add_test(NAME cxx_qt_gen_test_inputs_gen COMMAND rustfmt --check ${CXX_QT_GEN_TEST_INPUTS})
add_test(NAME cxx_qt_gen_test_outputs_gen COMMAND rustfmt --check ${CXX_QT_GEN_TEST_OUTPUTS})

# QML example has add_test in it's CMakeLists, so just add the cargo tests here
add_test_cargo(demo_threading "${CMAKE_CURRENT_SOURCE_DIR}/examples/demo_threading/Cargo.toml" DOCTESTS_OFF)
add_test_cargo(qml_features "${CMAKE_CURRENT_SOURCE_DIR}/examples/qml_features/Cargo.toml" DOCTESTS_OFF)
add_test_cargo(qml_extension_plugin "${CMAKE_CURRENT_SOURCE_DIR}/examples/qml_extension_plugin/core/Cargo.toml" DOCTESTS_OFF)
add_test_cargo(qml_minimal "${CMAKE_CURRENT_SOURCE_DIR}/examples/qml_minimal/Cargo.toml" DOCTESTS_OFF)
add_test_cargo(qml_with_threaded_logic "${CMAKE_CURRENT_SOURCE_DIR}/examples/qml_with_threaded_logic/Cargo.toml" DOCTESTS_OFF)

# Add test which scans for all .cpp and .h files in this project and runs clang-format
add_test(NAME cpp_clang_format COMMAND "${CMAKE_CURRENT_SOURCE_DIR}/clang_format_check.sh" "${CMAKE_CURRENT_SOURCE_DIR}")

# Add test which checks all files have a valid license
add_test(NAME reuse_lint COMMAND reuse lint)

add_subdirectory(tests)
