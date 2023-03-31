# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set(APP_NAME "${PROJECT_NAME}_test")

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS QuickTest)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS QuickTest REQUIRED)
endif()
get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)

add_executable(${APP_NAME} main.cpp)
target_link_libraries(${APP_NAME} Qt::QuickTest)

add_dependencies(${APP_NAME} core_qmlplugin)

set(test_cmd
    "$<TARGET_FILE:${APP_NAME}>"
    -import
    "${CMAKE_CURRENT_BINARY_DIR}/../qml"
    -input
    "${CMAKE_CURRENT_SOURCE_DIR}"
    -plugins
    "$<TARGET_FILE_DIR:core_qmlplugin>"
)

add_test(
    NAME ${APP_NAME}
    COMMAND ${test_cmd}
    WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"
)

# RUNTIME_ENV comes from the CMakeLists.txt at the root of this repository.
set_tests_properties(
    ${APP_NAME} PROPERTIES ENVIRONMENT_MODIFICATION "${RUNTIME_ENV}"
)

# TODO: Remove valgrind_additional_suppressions, there is a currently unknown issue with Qt5,
# which we can't precisely suppress, so we need to suppress all conditional checks in valgrind.
if(Qt5_FOUND)
    set(ADDITIONAL_SUPPRESSIONS --suppressions=${CMAKE_CURRENT_SOURCE_DIR}/valgrind_additional_suppressions.txt)
else()
    set(ADDITIONAL_SUPPRESSIONS "")
endif()

# TODO: can't use add_valgrind_test so cmake gets confused by the spaces in test_cmd, write manually for now
#
add_test(NAME ${APP_NAME}_valgrind
    COMMAND ${MEMORYCHECK_COMMAND} ${MEMORYCHECK_COMMAND_OPTIONS} --suppressions=${MEMORYCHECK_SUPPRESSIONS_FILE} ${ADDITIONAL_SUPPRESSIONS} --gen-suppressions=all ${test_cmd}
    WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}"
)
