# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(example_qml_features)
set(APP_NAME ${PROJECT_NAME})

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner QuickTest Test)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner QuickTest Test REQUIRED)
endif()
get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)

find_package(Corrosion QUIET)
if(NOT Corrosion_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.3.1
    )

    FetchContent_MakeAvailable(Corrosion)
endif()

set(CRATE qml-features)
corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})
set(CXXQT_EXPORT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxxqt")
corrosion_set_env_vars(${CRATE}
    "CXXQT_EXPORT_DIR=${CXXQT_EXPORT_DIR}"
    "QMAKE=${QMAKE}"
)
target_include_directories(${CRATE} INTERFACE "${CXXQT_EXPORT_DIR}/${CRATE}")
target_link_libraries(${CRATE} INTERFACE
    Qt::Core
    Qt::Gui
    Qt::Qml
    Qt::QuickControls2
)

add_executable(${APP_NAME}
    cpp/custom_object.h
    cpp/custom_object.cpp
    cpp/main.cpp
    cpp/qabstractlistmodelcxx.h
    qml/qml.qrc
)

target_include_directories(${APP_NAME} PRIVATE cpp)
target_link_libraries(${APP_NAME} PRIVATE ${CRATE})
qt_import_qml_plugins(${APP_NAME})

#
# Unit test
#

# Copy QML unit test to build folder
# this is so that our test executable can find the QML files in the same folder
function(copy_qml_test TEST_NAME)
    configure_file(${CMAKE_CURRENT_SOURCE_DIR}/tests/tst_${TEST_NAME}.qml ${CMAKE_CURRENT_BINARY_DIR}/tests/tst_${TEST_NAME}.qml COPYONLY)
endfunction()

copy_qml_test(containers)
copy_qml_test(custom_base_class)
copy_qml_test(invokables)
copy_qml_test(multiple_qobjects)
copy_qml_test(properties)
copy_qml_test(serialisation)
copy_qml_test(signals)
copy_qml_test(threading)
copy_qml_test(types)

add_executable(${APP_NAME}_test
    cpp/custom_object.h
    cpp/custom_object.cpp
    cpp/qabstractlistmodelcxx.h
    tests/main.cpp
)
target_include_directories(${APP_NAME}_test PRIVATE cpp)
target_link_libraries(${APP_NAME}_test PRIVATE
    ${CRATE}
    Qt::Test
    Qt::QuickTest
)
qt_import_qml_plugins(${APP_NAME}_test)

if(WIN32)
    set(EXE_SUFFIX ".exe")
else()
    set(EXE_SUFFIX "")
endif()

add_test(NAME ${APP_NAME}_test COMMAND ${CMAKE_CURRENT_BINARY_DIR}/tests/${APP_NAME}_test${EXE_SUFFIX} WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/tests)

add_valgrind_test(${APP_NAME}_test ${CMAKE_CURRENT_BINARY_DIR}/tests/${APP_NAME}_test${EXE_SUFFIX} ${CMAKE_CURRENT_BINARY_DIR}/tests)

set_target_properties(${APP_NAME}_test PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/tests
)
