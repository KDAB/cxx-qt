# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

project(example_qml_minimal_plugin)

# Rust always links against non-debug Windows runtime on *-msvc targets
# Note it is best to set this on the command line to ensure all targets are consistent
# https://github.com/corrosion-rs/corrosion/blob/master/doc/src/common_issues.md#linking-debug-cc-libraries-into-rust-fails-on-windows-msvc-targets
# https://github.com/rust-lang/rust/issues/39016
if (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
endif()

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CXXQT_QTCOMPONENTS Core Gui Qml QuickControls2 QuickTest Test)
if(NOT BUILD_WASM)
    set(CXXQT_QTCOMPONENTS ${CXXQT_QTCOMPONENTS} QmlImportScanner)
endif()

find_package(Qt6 COMPONENTS ${CXXQT_QTCOMPONENTS})
set(Qt "Qt6")
if(NOT Qt6_FOUND)
    message("Disabling example_qml_minimal_plugin, as Qt6 was not found, this example only works on Qt6.")
    return()
endif()

if (BUILD_WASM)
    message("Disabling example_qml_minimal_plugin, as it is not supported with WASM builds.")
    return()
endif()

if(MSVC)
    # Qt also needs to link against the non-debug version of the MSVC Runtime libraries.
    # Note: The Qt:: targets are ALIAS targets that do not support setting properties directly.
    # We therefore need to resolve the target names to either Qt5 or Qt6 directly.
    set_property(
        TARGET ${Qt}::Core ${Qt}::Gui ${Qt}::Qml ${Qt}::QuickControls2 ${Qt}::QuickTest ${Qt}::Test
        PROPERTY MAP_IMPORTED_CONFIG_DEBUG "RELEASE")
endif()


find_package(CxxQt QUIET)
if(NOT CxxQt_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        CxxQt
        GIT_REPOSITORY https://github.com/LeonMatthesKDAB/cxx-qt-cmake.git
        GIT_TAG 1266-qml-plugin
    )

    FetchContent_MakeAvailable(CxxQt)
endif()

# CXX-Qt (using Corrosion) creates a CMake target with the same name as the crate.
cxx_qt_import_crate(
    MANIFEST_PATH rust/Cargo.toml 
    CRATES qml_minimal_plugin
    LOCKED
    QT_MODULES Qt::Core Qt::Gui Qt::Qml Qt::QuickControls2
)

cxx_qt_import_qml_module(qml_minimal_plugin_qml_module
    URI "com.kdab.cxx_qt.demo"
    SOURCE_CRATE qml_minimal_plugin)

# Add the main.qml file via the qml.qrc, as Qt5 does not yet support qt_add_resources
add_executable(example_qml_minimal_plugin cpp/main.cpp qml/qml.qrc)

# Link to the qml module, which in turn links to the Rust qml_minimal library
target_link_libraries(example_qml_minimal_plugin PRIVATE Qt::Core Qt::Gui Qt::Qml Qt::QuickControls2)

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
        set(APP_TEST_NAME example_qml_minimal_plugin_${TEST_NAME}_test)
        add_executable(${APP_TEST_NAME} tests/${TEST_NAME}/tst_${TEST_NAME}.cpp)
        target_link_libraries(${APP_TEST_NAME} PRIVATE Qt::Core Qt::Gui Qt::Qml Qt::QuickControls2 Qt::QuickTest)

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
            example_qml_minimal_plugin_${TEST_NAME}_test
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

        # qmllint --max-warnings only exists in Qt 6
        if(Qt6_FOUND AND Qt6_VERSION VERSION_GREATER_EQUAL 6.8.0)
            find_program(QMLLINT_COMMAND qmllint PATHS "${QT_INSTALL_BINS}")
            if("${QMLLINT_COMMAND}" STREQUAL "QMLLINT_COMMAND-NOTFOUND")
                MESSAGE(STATUS "qmllint not found. Please install it. Skipping qmllint tests.")
            else()
                file(GLOB QMLLINT_QML_FILES ${CMAKE_CURRENT_SOURCE_DIR}/qml/*.qml)
                add_test(NAME example_qml_minimal_plugin_qmllint_check COMMAND ${QMLLINT_COMMAND} --max-warnings 0 -I ${CMAKE_CURRENT_BINARY_DIR}/cxxqt/qml_modules ${QMLLINT_QML_FILES})
            endif()
        endif()
    endfunction()

    add_qml_test(myobject)
endif()
