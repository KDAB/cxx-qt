# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(cxx_qt)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

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

add_subdirectory(examples)

# Create helper method which adds relevent cargo tests for a given manifest
function(add_test_cargo TEST_NAME_PREFIX MANIFEST_PATH ADD_DOCTESTS)
    # Add cargo as a test
    add_test(NAME ${TEST_NAME_PREFIX}_cargo_tests COMMAND cargo test --all-targets --all-features --manifest-path ${MANIFEST_PATH})
    # Check if we should enable doc tests
    if (${ADD_DOCTESTS} STREQUAL "DOCTESTS_ON")
        # Add cargo docs as a test
        add_test(NAME ${TEST_NAME_PREFIX}_cargo_doc_tests COMMAND cargo test --doc --manifest-path ${MANIFEST_PATH})
    endif()
    # Add clippy as a test
    add_test(NAME ${TEST_NAME_PREFIX}_cargo_clippy COMMAND cargo clippy --all-targets --all-features --manifest-path ${MANIFEST_PATH} -- -D warnings)
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

# Ensure test inputs and outputs are formatted
file(GLOB CXX_QT_GEN_TEST_INPUTS ${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt-gen/test_inputs/*.rs)
file(GLOB CXX_QT_GEN_TEST_OUTPUTS ${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt-gen/test_outputs/*.rs)
add_test(NAME cxx_qt_gen_test_inputs_gen COMMAND rustfmt --check ${CXX_QT_GEN_TEST_INPUTS})
add_test(NAME cxx_qt_gen_test_outputs_gen COMMAND rustfmt --check ${CXX_QT_GEN_TEST_OUTPUTS})

# Create helper method which adds relevent tests for the given example
function(add_example_tests EXAMPLE_NAME)
    set(NAME_WITH_PREFIX example_${EXAMPLE_NAME})

    # Add all the normal tests used on the other modules
    add_test_cargo(${NAME_WITH_PREFIX} "${CMAKE_CURRENT_SOURCE_DIR}/examples/${EXAMPLE_NAME}/Cargo.toml" DOCTESTS_OFF)

    # The example executable itself is a test that needs to be run
    add_test(NAME ${NAME_WITH_PREFIX}_cpp_tests COMMAND ${NAME_WITH_PREFIX})

    # Add valgrind test
    add_valgrind_test(${NAME_WITH_PREFIX} "${CMAKE_CURRENT_BINARY_DIR}/examples/${EXAMPLE_NAME}/example_${EXAMPLE_NAME}" "${CMAKE_CURRENT_BINARY_DIR}/examples/${EXAMPLE_NAME}")
endfunction()

# Add tests for all the examples
# TODO: we probably want to move tests of examples into the CMakeLists under the examples folder
add_example_tests(basic_cxx_only)
add_example_tests(qt_types_standalone)
add_example_tests(update_requester_standalone)
add_example_tests(basic_cxx_qt)
# QML example has add_test in it's CMakeLists, so just add the cargo tests here
add_test_cargo(basic_cxx_qt_qml "${CMAKE_CURRENT_SOURCE_DIR}/examples/basic_cxx_qt_qml/Cargo.toml" DOCTESTS_OFF)
add_test_cargo(basic_cxx_qt_qml_plugin "${CMAKE_CURRENT_SOURCE_DIR}/examples/basic_cxx_qt_qml_plugin/Cargo.toml" DOCTESTS_OFF)
add_test_cargo(qml_with_threaded_logic "${CMAKE_CURRENT_SOURCE_DIR}/examples/qml_with_threaded_logic/Cargo.toml" DOCTESTS_OFF)

# Add test which scans for all .cpp and .h files in this project and runs clang-format
add_test(NAME cpp_clang_format COMMAND "${CMAKE_CURRENT_SOURCE_DIR}/clang_format_check.sh" "${CMAKE_CURRENT_SOURCE_DIR}")

# Add test which checks all files have a valid license
add_test(NAME reuse_lint COMMAND reuse lint)
