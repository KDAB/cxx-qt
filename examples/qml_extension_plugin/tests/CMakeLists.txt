# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

find_program(QMLTESTRUNNER_BIN qmltestrunner)
if(QMLTESTRUNNER_BIN)
    message(STATUS "qmltestrunner found at: ${QMLTESTRUNNER_BIN}")
else()
    message(CRITICAL "qmltestrunner not found: tests disabled")
endif()

set(test_cmd ${QMLTESTRUNNER_BIN} -import "${CMAKE_CURRENT_BINARY_DIR}/../qml" -input ${CMAKE_CURRENT_SOURCE_DIR})
add_test(NAME ${APP_NAME}_test
    COMMAND ${test_cmd}
    WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/../"
)

# TODO: can't use add_valgrind_test so cmake gets confused by the spaces in test_cmd, write manually for now
add_test(NAME ${APP_NAME}_test_valgrind
    COMMAND ${MEMORYCHECK_COMMAND} ${MEMORYCHECK_COMMAND_OPTIONS} --suppressions=${MEMORYCHECK_SUPPRESSIONS_FILE} --gen-suppressions=all ${test_cmd}
    WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"
)

# TODO: test needs to depend on the app ? which depends on the plugin?
