# SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

find_package(Corrosion QUIET)
if(NOT Corrosion_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.5.0
    )

    FetchContent_MakeAvailable(Corrosion)
endif()

function(cxxqt_import_crate)
  cmake_parse_arguments(IMPORT_CRATE "" "CXXQT_EXPORT_DIR;QMAKE" "" ${ARGN})

  corrosion_import_crate(IMPORTED_CRATES __cxxqt_imported_crates ${IMPORT_CRATE_UNPARSED_ARGUMENTS})

  message(STATUS "Found CXX-Qt crate(s): ${__cxxqt_imported_crates}")

  if (NOT DEFINED IMPORT_CRATE_CXXQT_EXPORT_DIR)
    set(IMPORT_CRATE_CXXQT_EXPORT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxxqt/")
  endif()
  message(VERBOSE "CXX-Qt EXPORT_DIR: ${IMPORT_CRATE_CXXQT_EXPORT_DIR}")

  if (NOT DEFINED IMPORT_CRATE_QMAKE)
    get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)
    if (NOT QMAKE STREQUAL "QMAKE-NOTFOUND")
      set(IMPORT_CRATE_QMAKE "${QMAKE}")
    else()
      message(FATAL_ERROR "cxxqt_import_crate: QMAKE is not defined and could not be queried from the Qt::qmake target!\nPlease use the QMAKE argument to specify the path to the qmake executable or use find_package(Qt) before calling this function.")
    endif()
  endif()

  foreach(CRATE ${__cxxqt_imported_crates})

    corrosion_set_env_vars(${CRATE}
      "CXXQT_EXPORT_DIR=${IMPORT_CRATE_CXXQT_EXPORT_DIR}"
      "QMAKE=${IMPORT_CRATE_QMAKE}"
      $<$<BOOL:${CMAKE_RUSTC_WRAPPER}>:RUSTC_WRAPPER=${CMAKE_RUSTC_WRAPPER}>)

    file(MAKE_DIRECTORY "${IMPORT_CRATE_CXXQT_EXPORT_DIR}/include/${CRATE}")
    target_include_directories(${CRATE} INTERFACE "${IMPORT_CRATE_CXXQT_EXPORT_DIR}/include/${CRATE}")

    set_target_properties(${CRATE}
      PROPERTIES
      CXXQT_EXPORT_DIR "${IMPORT_CRATE_CXXQT_EXPORT_DIR}")
  endforeach()

endfunction()


function(cxxqt_import_qml_module target)
  cmake_parse_arguments(QML_MODULE "" "URI;SOURCE_CRATE" "" ${ARGN})

  if (NOT DEFINED QML_MODULE_URI)
    message(FATAL_ERROR "cxxqt_import_qml_module: URI must be specified!")
  endif()

  if (NOT DEFINED QML_MODULE_SOURCE_CRATE)
    message(FATAL_ERROR "cxxqt_import_qml_module: SOURCE_CRATE must be specified!")
  endif()

  get_target_property(QML_MODULE_EXPORT_DIR ${QML_MODULE_SOURCE_CRATE} CXXQT_EXPORT_DIR)

  if (${QML_MODULE_EXPORT_DIR} STREQUAL "QML_MODULE_EXPORT_DIR-NOTFOUND")
    message(FATAL_ERROR "cxxqt_import_qml_module: SOURCE_CRATE must be a valid target that has been imported with cxxqt_import_crate!")
  endif()

  # Note: This needs to match the URI conversion in cxx-qt-build
  string(REPLACE "." "_" plugin_name ${QML_MODULE_URI})
  set(QML_MODULE_PLUGIN_DIR "${QML_MODULE_EXPORT_DIR}/plugins/${plugin_name}")
  file(MAKE_DIRECTORY ${QML_MODULE_PLUGIN_DIR})

  # QML plugin - init target
  set_source_files_properties(
    "${QML_MODULE_PLUGIN_DIR}/plugin_init.o"
    PROPERTIES GENERATED ON)
  add_library(${target} OBJECT IMPORTED)
  set_target_properties(${target}
    PROPERTIES
    IMPORTED_OBJECTS "${QML_MODULE_PLUGIN_DIR}/plugin_init.o")
  target_link_libraries(${target} INTERFACE ${QML_MODULE_SOURCE_CRATE})

  message(VERBOSE "Expecting CXX-Qt QML plugin: ${QML_MODULE_URI} in directory: ${QML_MODULE_PLUGIN_DIR}")
endfunction()
