# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
# ANCHOR: book_cmake_setup
cmake_minimum_required(VERSION 3.16)

project(example_qml_minimal)
set(APP_NAME ${PROJECT_NAME})

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner REQUIRED)
endif()
# ANCHOR_END: book_cmake_setup

# ANCHOR: book_cmake_find_qmake
# The path to the qmake executable path needs to be passed to the Rust
# library's build script to ensure it uses the same installation of Qt as CMake.
get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)
# ANCHOR_END: book_cmake_find_qmake

# ANCHOR: book_cmake_find_corrosion
find_package(Corrosion QUIET)
if(NOT Corrosion_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/Be-ing/corrosion.git
        GIT_TAG code_generators
    )

    FetchContent_MakeAvailable(Corrosion)
endif()
# ANCHOR_END: book_cmake_find_corrosion

# ANCHOR: book_cmake_use_corrosion
set(CRATE qml-minimal)
# Corrosion creates a CMake target with the same name as the crate.
corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})

# Add the generated headers to the include path
corrosion_add_target_generated_headers(${CRATE})

# Set the QMAKE environment variable to ensure the Rust library uses
# the same installation of Qt as CMake.
corrosion_set_env_vars(${CRATE} "QMAKE=${QMAKE}")

# Link the Rust INTERFACE library target to Qt. Do this on the library target
# rather than the main executable. This way, CMake targets besides the main
# executable which link the Rust library, for example tests, will also link Qt.
target_link_libraries(${CRATE} INTERFACE
    Qt::Core
    Qt::Gui
    Qt::Qml
    Qt::QuickControls2
)
# ANCHOR_END: book_cmake_use_corrosion

# ANCHOR: book_cmake_executable
# Define the executable with the C++ source and Qt resource files
add_executable(${APP_NAME} cpp/main.cpp qml/qml.qrc)

# Link to the Rust library
target_link_libraries(${APP_NAME} PRIVATE ${CRATE})
# ANCHOR_END: book_cmake_executable

#
# Unit test
#

if(TARGET Qt6::Core)
    find_package(Qt6 COMPONENTS QuickTest Test REQUIRED)
else()
    find_package(Qt5 COMPONENTS QuickTest Test REQUIRED)
endif()

function(add_qml_test TEST_NAME)
    # Copy QML unit test to build folder
    # this is so that our tst_${TEST_NAME} executable can find the QML files
    configure_file(${CMAKE_CURRENT_SOURCE_DIR}/tests/${TEST_NAME}/tst_${TEST_NAME}.qml ${CMAKE_CURRENT_BINARY_DIR}/${TEST_NAME}/tst_${TEST_NAME}.qml COPYONLY)

    add_executable("${APP_NAME}_${TEST_NAME}_test" "tests/${TEST_NAME}/tst_${TEST_NAME}.cpp")
    target_link_libraries(${APP_NAME}_${TEST_NAME}_test PRIVATE
        ${CRATE}
        Qt::QuickTest
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
