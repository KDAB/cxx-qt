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
    set(TARGET_NAME tests_${TEST_NAME})

    # Add all the normal tests used on the other modules
    add_test_cargo(
        ${NAME_WITH_PREFIX}
        "${CMAKE_CURRENT_SOURCE_DIR}/${TEST_NAME}/rust/Cargo.toml" DOCTESTS_OFF
    )

    set(CPP_TEST_NAME ${NAME_WITH_PREFIX}_cpp_tests)
    add_test(NAME ${CPP_TEST_NAME} COMMAND $<TARGET_FILE:${TARGET_NAME}>)

    set_tests_properties(
        ${CPP_TEST_NAME} PROPERTIES ENVIRONMENT_MODIFICATION "${RUNTIME_ENV}"
    )

    # Add valgrind test
    add_valgrind_test(
        ${NAME_WITH_PREFIX} $<TARGET_FILE:${TARGET_NAME}>
        ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}
    )
endfunction()

# Add tests for all the acceptance tests
add_acceptance_tests(basic_cxx_only)
add_acceptance_tests(basic_cxx_qt)
add_acceptance_tests(qt_types_standalone)
