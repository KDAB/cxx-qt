# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(tests_basic_cxx_only)
set(APP_NAME ${PROJECT_NAME})

# TODO: at some point we want to transform cxx-qt-build into cxx-cmake-build
# to clarify that qt support is optional. At this point it would make more
# sense to never generate files with Qt code rather than to #ifdef it out using
# this flag as we currently do for cases where the application doesn't link to Qt.
add_compile_definitions(NO_QT)

set(CXXQTBRIDGE_OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxx-qt-generated")
execute_process(
    COMMAND ${CXXQTBRIDGE_EXECUTABLE}
            --input src/lib.rs
            --output ${CXXQTBRIDGE_OUTPUT_DIR}
    OUTPUT_VARIABLE OUTPUT_FILES
    RESULT_VARIABLE CXXQTBRIDGE_RESULT
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)
if(NOT CXXQTBRIDGE_RESULT EQUAL 0)
    message(FATAL_ERROR "Failed to generate C++ files with cxxqtbridge from ${INPUT_FILE}")
endif()
separate_arguments(GENERATED_SOURCES NATIVE_COMMAND "${OUTPUT_FILES}")

add_executable(${APP_NAME}
    src/main.cpp
    ${GENERATED_SOURCES}
)
target_include_directories(${APP_NAME} PRIVATE
    include
    "${CXXQTBRIDGE_OUTPUT_DIR}/include"
)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)
target_link_libraries(${APP_NAME} PRIVATE basic-cxx-only)

target_include_directories(${APP_NAME} PRIVATE include)
target_link_libraries(${APP_NAME} PRIVATE doctest)
