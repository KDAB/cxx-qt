# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(cxx_qt)

# Define doctest as a library that the examples can link to in order to run their internal tests
add_library(doctest INTERFACE)
target_include_directories(doctest INTERFACE ${CMAKE_SOURCE_DIR}/doctest)

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
add_test_cargo(clang_format "${CMAKE_CURRENT_SOURCE_DIR}/clang-format/Cargo.toml" DOCTESTS_ON)
add_test_cargo(cxx_qt "${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt/Cargo.toml" DOCTESTS_ON)
add_test_cargo(cxx_qt_build "${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt-build/Cargo.toml" DOCTESTS_ON)
add_test_cargo(cxx_qt_gen "${CMAKE_CURRENT_SOURCE_DIR}/cxx-qt-gen/Cargo.toml" DOCTESTS_ON)

# Create helper method which adds relevent tests for the given example
function(add_example_tests EXAMPLE_NAME)
    set(NAME_WITH_PREFIX example_${EXAMPLE_NAME})

    # Add all the normal tests used on the other modules
    add_test_cargo(${NAME_WITH_PREFIX} "${CMAKE_CURRENT_SOURCE_DIR}/examples/${EXAMPLE_NAME}/Cargo.toml" DOCTESTS_OFF)
    # The example executable itself is a test that needs to be run
    add_test(NAME ${NAME_WITH_PREFIX}_cpp_tests COMMAND ${NAME_WITH_PREFIX})
endfunction()

# Add tests for all the examples
# TODO: we probably want to move tests of examples into the CMakeLists under the examples folder
add_example_tests(basic_cxx_only)

# Enable testing
enable_testing()
