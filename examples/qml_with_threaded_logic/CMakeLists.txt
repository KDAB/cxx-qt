# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(example_qml_with_threaded_logic)
set(APP_NAME ${PROJECT_NAME})

# TODO: Add a helper function to our CMake module which automatically
# handles some of this boilerplate for a "typical" Qt application
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Gui Qml QuickControls2 QuickTest Test REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Qml QuickControls2 QuickTest Test REQUIRED)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../../cmake")
include(CxxQt)

set(
    CPP_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp
)

set(
    RESOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/qml.qrc
)

# Generate the C++ code
cxx_qt_generate_cpp(GEN_SOURCES)
# Define our executable with our QML resource files
add_executable(${APP_NAME} "${CPP_SOURCES}" "${GEN_SOURCES}" "${RESOURCES}")
# Include generated sources
cxx_qt_include(${APP_NAME})
# Link to generated rust library
cxx_qt_link_rustlib(${APP_NAME})
target_link_libraries(${APP_NAME} Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Qml Qt${QT_VERSION_MAJOR}::QuickControls2 Qt${QT_VERSION_MAJOR}::Test)

#
# Unit test
#

function(add_qml_test TEST_NAME)
    # Copy QML unit test to build folder
    # this is so that our tst_${TEST_NAME} executable can find the QML files
    configure_file(${CMAKE_CURRENT_SOURCE_DIR}/src/tests/${TEST_NAME}/tst_${TEST_NAME}.qml ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}/tst_${TEST_NAME}.qml COPYONLY)

    cxx_qt_cmake(${APP_NAME}_${TEST_NAME}_test "${CMAKE_CURRENT_SOURCE_DIR}/src/tests/${TEST_NAME}/tst_${TEST_NAME}.cpp")
    target_link_libraries(${APP_NAME}_${TEST_NAME}_test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Qml Qt${QT_VERSION_MAJOR}::QuickTest)

    add_test(NAME ${APP_NAME}_${TEST_NAME}_test COMMAND ${APP_NAME}_${TEST_NAME}_test WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME})

    add_valgrind_test(${APP_NAME}_${TEST_NAME}_test ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}/${APP_NAME}_${TEST_NAME}_test ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME})

    set_target_properties(${APP_NAME}_${TEST_NAME}_test PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}
    )
endfunction()

add_qml_test(website)
