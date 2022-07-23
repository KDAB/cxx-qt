# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is

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

cxx_qt_create_target(
    TARGET ${APP_NAME}_cxxqt
    BRIDGE_MODULE_FILES src/lib.rs
    MANIFEST_PATH Cargo.toml
    CRATES qml-minimal
)

# ANCHOR: book_cmake_generation
# ANCHOR: book_tutorial_cmake_diff_1

# ANCHOR_END: book_tutorial_cmake_diff_1

# Define our executable with our C++ source, generated sources, and QML resource files
add_executable(${APP_NAME}
    src/main.cpp
    src/qml.qrc
)

# Link to Qt in the normal way
target_link_libraries(${APP_NAME} PRIVATE
    ${APP_NAME}_cxxqt
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

    set(TARGET ${APP_NAME}_${TEST_NAME}_test)
    add_executable(${TARGET} "${CMAKE_CURRENT_SOURCE_DIR}/src/tests/${TEST_NAME}/tst_${TEST_NAME}.cpp")
    target_link_libraries(${APP_NAME}_${TEST_NAME}_test PRIVATE
        ${APP_NAME}_cxxqt
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
