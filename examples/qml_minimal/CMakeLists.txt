# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# ANCHOR: book_cmake_setup
cmake_minimum_required(VERSION 3.24)

project(example_qml_minimal)

# Rust always links against non-debug Windows runtime on *-msvc targets
# Note it is best to set this on the command line to ensure all targets are consistent
# https://github.com/corrosion-rs/corrosion/blob/master/doc/src/common_issues.md#linking-debug-cc-libraries-into-rust-fails-on-windows-msvc-targets
# https://github.com/rust-lang/rust/issues/39016
if (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
endif()

set(CMAKE_AUTOMOC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner REQUIRED)
endif()
# ANCHOR_END: book_cmake_setup

# ANCHOR: book_cmake_find_cxx_qt_start
find_package(CxxQt QUIET)
if(NOT CxxQt_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        CxxQt
        GIT_REPOSITORY https://github.com/kdab/cxx-qt-cmake.git
        # ANCHOR_END: book_cmake_find_cxx_qt_start
        GIT_TAG main
        # ANCHOR: book_cmake_find_cxx_qt_end
    )

    FetchContent_MakeAvailable(CxxQt)
endif()
# ANCHOR_END: book_cmake_find_cxx_qt_end

# ANCHOR: book_cmake_use_cxx_qt
# CXX-Qt (using Corrosion) creates a CMake target with the same name as the crate.
cxx_qt_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES qml_minimal)
target_link_libraries(qml_minimal INTERFACE Qt::Core Qt::Gui Qt::Qml Qt::QuickControls2)

cxx_qt_import_qml_module(qml_minimal_qml_module
    URI "com.kdab.cxx_qt.demo"
    SOURCE_CRATE qml_minimal)
# ANCHOR_END: book_cmake_use_cxx_qt

# ANCHOR: book_cmake_executable
# Define the executable with the C++ source
add_executable(example_qml_minimal cpp/main.cpp)

# Link to the qml module, which in turn links to the Rust qml_minimal library
target_link_libraries(example_qml_minimal PRIVATE qml_minimal_qml_module)

# If we are using a statically linked Qt then we need to import any qml plugins
qt_import_qml_plugins(example_qml_minimal)
# ANCHOR_END: book_cmake_executable

if(BUILD_TESTING)
    #
    # Unit test
    #

    if(TARGET Qt6::Core)
        find_package(Qt6 COMPONENTS QuickTest Test REQUIRED)
    else()
        find_package(Qt5 COMPONENTS QuickTest Test REQUIRED)
    endif()

    function(add_qml_test TEST_NAME)
        set(APP_TEST_NAME example_qml_minimal_${TEST_NAME}_test)
        add_executable(${APP_TEST_NAME} tests/${TEST_NAME}/tst_${TEST_NAME}.cpp)
        target_link_libraries(${APP_TEST_NAME} PRIVATE qml_minimal_qml_module Qt::QuickTest)
        qt_import_qml_plugins(${APP_TEST_NAME})

        set(TEST_CMD
            $<TARGET_FILE:${APP_TEST_NAME}> -input
            ${CMAKE_CURRENT_SOURCE_DIR}/tests/${TEST_NAME}/tst_${TEST_NAME}.qml
        )

        add_test(
            NAME ${APP_TEST_NAME}
            COMMAND ${TEST_CMD}
        )
        # Unfortunately due to the static linking in our CI on macOS we can't load the
        # offscreen plugin, so just leave it at the default.
        if (NOT APPLE)
            set_tests_properties(${APP_TEST_NAME} PROPERTIES ENVIRONMENT "QT_QPA_PLATFORM=offscreen")
        endif()
        # RUNTIME_ENV comes from the CMakeLists.txt at the root of this repository.
        set_tests_properties(
            example_qml_minimal_${TEST_NAME}_test
            PROPERTIES
            ENVIRONMENT_MODIFICATION "${RUNTIME_ENV}"
        )

        if (COMMAND add_valgrind_test)
            add_valgrind_test(
                ${APP_TEST_NAME} "${TEST_CMD}" ${CMAKE_CURRENT_BINARY_DIR}
            )
        else()
            MESSAGE(STATUS "add_valgrind_test is defined in the top level of CXX-Qt. It will not executed")
        endif()
    endfunction()

    add_qml_test(myobject)
endif()
