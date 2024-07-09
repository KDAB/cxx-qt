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

    set(CPP_TEST_NAME ${NAME_WITH_PREFIX}_cpp_tests)
    add_test(NAME ${CPP_TEST_NAME} COMMAND $<TARGET_FILE:${TARGET_NAME}>)

    # Unfortunately due to the static linking in our CI we can't load the
    # offscreen plugin, so just leave it at the default.
    if (NOT APPLE)
       set_tests_properties(
          ${CPP_TEST_NAME}
          PROPERTIES
          ENVIRONMENT "QT_QPA_PLATFORM=offscreen"
          )
    endif()
    set_tests_properties(
       ${CPP_TEST_NAME}
       PROPERTIES
       ENVIRONMENT_MODIFICATION "${RUNTIME_ENV}"
    )

    if (NOT WIN32)
       if (COMMAND add_valgrind_test)
          # Add valgrind test
          add_valgrind_test(
              ${NAME_WITH_PREFIX} $<TARGET_FILE:${TARGET_NAME}>
              ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}
          )
       else()
	  MESSAGE(STATUS "add_valgrind_test is defined in the top level of CXX-Qt. It will not executed")
       endif()
    endif()
endfunction()

# Add tests for all the acceptance tests
add_acceptance_tests(basic_cxx_only)
add_acceptance_tests(basic_cxx_qt)
add_acceptance_tests(qt_types_standalone)
