# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../../cmake")

# TODO: figure out what the actual hard minimum is

# TODO: Add a helper function to our CMake module which automatically
# handles some of this boilerplate for a "typical" Qt application

# ANCHOR: book_tutorial_cmake_full
cmake_minimum_required(VERSION 3.16)

project(example_qml_minimal)
set(APP_NAME ${PROJECT_NAME})

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner REQUIRED)
find_package(CxxQt${QT_VERSION_MAJOR} QUIET)

# ANCHOR: book_cmake_generation
# ANCHOR: book_tutorial_cmake_diff_1
include(CxxQt)

# Generate C++ code from Rust using Cargo in the current folder
cxx_qt_generate_cpp(GEN_SOURCES)
# ANCHOR_END: book_tutorial_cmake_diff_1

# Define our sources
set(
    CPP_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp
)

set(
    RESOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/qml.qrc
)

# Define our executable with our C++ source, generated sources, and QML resource files
add_executable(${APP_NAME} "${CPP_SOURCES}" "${GEN_SOURCES}" "${RESOURCES}")

# ANCHOR: book_tutorial_cmake_diff_2
# Include generated sources
cxx_qt_include(${APP_NAME})

# Link to generated rust library
cxx_qt_link_rustlib(${APP_NAME})
# ANCHOR_END: book_tutorial_cmake_diff_2

# Link to Qt in the normal way
target_link_libraries(${APP_NAME} PRIVATE
    CxxQt${QT_VERSION_MAJOR}::cxxqt
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
    Qt${QT_VERSION_MAJOR}::Qml
    Qt${QT_VERSION_MAJOR}::QuickControls2
)
qt_import_qml_plugins(${APP_NAME})
# ANCHOR_END: book_cmake_generation
# ANCHOR_END: book_tutorial_cmake_full

#
# Unit test
#

find_package(QT NAMES Qt6 Qt5 COMPONENTS QuickTest Test REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS QuickTest Test REQUIRED)

function(add_qml_test TEST_NAME)
    # Copy QML unit test to build folder
    # this is so that our tst_${TEST_NAME} executable can find the QML files
    configure_file(${CMAKE_CURRENT_SOURCE_DIR}/src/tests/${TEST_NAME}/tst_${TEST_NAME}.qml ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}/tst_${TEST_NAME}.qml COPYONLY)

    cxx_qt_cmake(${APP_NAME}_${TEST_NAME}_test "${CMAKE_CURRENT_SOURCE_DIR}/src/tests/${TEST_NAME}/tst_${TEST_NAME}.cpp")
    target_link_libraries(${APP_NAME}_${TEST_NAME}_test PRIVATE
        CxxQt${QT_VERSION_MAJOR}::cxxqt
        Qt${QT_VERSION_MAJOR}::Core
        Qt${QT_VERSION_MAJOR}::Gui
        Qt${QT_VERSION_MAJOR}::Qml
        Qt${QT_VERSION_MAJOR}::QuickTest
    )
    qt_import_qml_plugins(${APP_NAME}_${TEST_NAME}_test)

    if(WIN32)
        set(EXE_SUFFIX ".exe")
    else()
        set(EXE_SUFFIX "")
    endif()

    add_test(NAME ${APP_NAME}_${TEST_NAME}_test COMMAND ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}/${APP_NAME}_${TEST_NAME}_test${EXE_SUFFIX} WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME})

    add_valgrind_test(${APP_NAME}_${TEST_NAME}_test ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}/${APP_NAME}_${TEST_NAME}_test${EXE_SUFFIX} ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME})

    set_target_properties(${APP_NAME}_${TEST_NAME}_test PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}
    )
endfunction()

add_qml_test(myobject)
