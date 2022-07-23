# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

function(cxx_qt_generate_cpp)
    cmake_parse_arguments(CXXQT "" "TARGET" "BRIDGE_MODULE_FILES" ${ARGN})

    set_property(DIRECTORY APPEND PROPERTY CMAKE_CONFIGURE_DEPENDS ${CXXQT_BRIDGE_MODULE_FILES})

    find_program(CXXQTBRIDGE_EXECUTABLE cxxqtbridge REQUIRED)
    set(CXXQTBRIDGE_OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxx-qt-generated")
    set(CXXQTBRIDGE_GENERATED_SOURCES "")
    foreach(INPUT_FILE IN LISTS CXXQT_BRIDGE_MODULE_FILES)
        execute_process(
            COMMAND ${CXXQTBRIDGE_EXECUTABLE}
                    --input ${INPUT_FILE}
                    --output ${CXXQTBRIDGE_OUTPUT_DIR}
            OUTPUT_VARIABLE OUTPUT_FILES
            RESULT_VARIABLE CXXQTBRIDGE_RESULT
            WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        )
        if(NOT CXXQTBRIDGE_RESULT EQUAL 0)
            message(FATAL_ERROR "Failed to generate C++ files with cxxqtbridge from ${INPUT_FILE}")
        endif()
        separate_arguments(OUTPUT_FILES NATIVE_COMMAND "${OUTPUT_FILES}")
        list(APPEND CXXQTBRIDGE_GENERATED_SOURCES ${OUTPUT_FILES})
    endforeach()

    add_library(${CXXQT_TARGET}_generated_cpp STATIC ${CXXQTBRIDGE_GENERATED_SOURCES})
    target_include_directories(${CXXQT_TARGET}_generated_cpp PUBLIC "${CXXQTBRIDGE_OUTPUT_DIR}/include")
    target_link_libraries(${CXXQT_TARGET}_generated_cpp PUBLIC CxxQt${QT_VERSION_MAJOR}::cxxqt)
endfunction()

function(cxx_qt_create_target)
    cmake_parse_arguments(CXXQT "" "TARGET;MANIFEST_PATH" "BRIDGE_MODULE_FILES;CRATES" ${ARGN})

    add_library(${CXXQT_TARGET} INTERFACE)

    cxx_qt_generate_cpp(
        TARGET ${CXXQT_TARGET}
        BRIDGE_MODULE_FILES ${CXXQT_BRIDGE_MODULE_FILES}
    )
    target_link_libraries(${CXXQT_TARGET} INTERFACE ${CXXQT_TARGET}_generated_cpp)

    # FIXME: cxx's build.rs fails without this.
    # https://github.com/dtolnay/cxx/issues/1020
    if(APPLE)
        set(ENV{SDKROOT} ${CMAKE_OSX_SYSROOT})
    endif()
    corrosion_import_crate(MANIFEST_PATH ${CXXQT_MANIFEST_PATH})
    target_link_libraries(${CXXQT_TARGET} INTERFACE ${CXXQT_CRATES})
endfunction()
