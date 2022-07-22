# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

add_subdirectory(basic_cxx_qt)
add_subdirectory(basic_cxx_only)
add_subdirectory(qt_types_standalone)

# Create helper method which adds relevent tests for the given acceptance test
function(add_acceptance_tests TEST_NAME)
    set(NAME_WITH_PREFIX test_${TEST_NAME})

    # Add all the normal tests used on the other modules
    add_test_cargo(${NAME_WITH_PREFIX} "${CMAKE_CURRENT_SOURCE_DIR}/${TEST_NAME}/Cargo.toml" DOCTESTS_OFF)

    # The executable itself is a test that needs to be run
    if(WIN32)
        set(EXE_SUFFIX ".exe")
    else()
        set(EXE_SUFFIX "")
    endif()

    add_test(NAME ${NAME_WITH_PREFIX}_cpp_tests COMMAND "${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}/tests_${TEST_NAME}${EXE_SUFFIX}")

    # Add valgrind test
    add_valgrind_test(${NAME_WITH_PREFIX} "${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}/tests_${TEST_NAME}${EXE_SUFFIX}" "${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}")
endfunction()

# Add tests for all the acceptance tests
add_acceptance_tests(basic_cxx_only)
add_acceptance_tests(qt_types_standalone)
add_acceptance_tests(basic_cxx_qt)
