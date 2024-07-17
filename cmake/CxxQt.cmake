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

function(cxx_qt_import_crate)
  cmake_parse_arguments(IMPORT_CRATE "" "CXX_QT_EXPORT_DIR;QMAKE" "" ${ARGN})

  corrosion_import_crate(IMPORTED_CRATES __cxx_qt_imported_crates ${IMPORT_CRATE_UNPARSED_ARGUMENTS})

  message(STATUS "CXX-Qt Found crate(s): ${__cxx_qt_imported_crates}")

  if (NOT DEFINED IMPORT_CRATE_CXX_QT_EXPORT_DIR)
    set(IMPORT_CRATE_CXX_QT_EXPORT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxxqt/")
  endif()
  message(VERBOSE "CXX-Qt EXPORT_DIR: ${IMPORT_CRATE_CXX_QT_EXPORT_DIR}")

  if (NOT DEFINED IMPORT_CRATE_QMAKE)
    get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)
    if (NOT QMAKE STREQUAL "QMAKE-NOTFOUND")
      set(IMPORT_CRATE_QMAKE "${QMAKE}")
    else()
      message(FATAL_ERROR "cxx_qt_import_crate: QMAKE is not defined and could not be queried from the Qt::qmake target!\nPlease use the QMAKE argument to specify the path to the qmake executable or use find_package(Qt) before calling cxx_qt_import_crate.")
    endif()
  endif()

  foreach(CRATE ${__cxx_qt_imported_crates})
    corrosion_set_env_vars(${CRATE}
      # Tell cxx-qt-build where to export the data
      "CXX_QT_EXPORT_DIR=${IMPORT_CRATE_CXX_QT_EXPORT_DIR}"
      # Tell cxx-qt-build which crate to export
      "CXX_QT_EXPORT_CRATE_${CRATE}=1"
      "QMAKE=${IMPORT_CRATE_QMAKE}"
      $<$<BOOL:${CMAKE_RUSTC_WRAPPER}>:RUSTC_WRAPPER=${CMAKE_RUSTC_WRAPPER}>)

    file(MAKE_DIRECTORY "${IMPORT_CRATE_CXX_QT_EXPORT_DIR}/crates/${CRATE}/include/")
    target_include_directories(${CRATE} INTERFACE "${IMPORT_CRATE_CXX_QT_EXPORT_DIR}/crates/${CRATE}/include/")

    set_target_properties(${CRATE}
      PROPERTIES
      CXX_QT_EXPORT_DIR "${IMPORT_CRATE_CXX_QT_EXPORT_DIR}")

    # cxx-qt-build generates object files that need to be linked to the final target.
    # These are the static initializers that would be removed as an optimization if they're not referenced.
    # So add them to an object library instead.
    file(MAKE_DIRECTORY "${IMPORT_CRATE_CXX_QT_EXPORT_DIR}/crates/${CRATE}/")
    # When using the Ninja generator, we need to provide **some** way to generate the object file
    # Unfortunately I'm not able to tell corrosion that this obj file is indeed a byproduct, so
    # create a fake target for it.
    # This target doesn't need to do anything, because the file should already exist after building the crate.
    add_custom_target(${CRATE}_mock_initializers
      COMMAND ${CMAKE_COMMAND} -E true
      DEPENDS ${CRATE}
      BYPRODUCTS "${IMPORT_CRATE_CXX_QT_EXPORT_DIR}/crates/${CRATE}/initializers.o")

    add_library(${CRATE}_initializers OBJECT IMPORTED)
    set_target_properties(${CRATE}_initializers
      PROPERTIES
      IMPORTED_OBJECTS "${IMPORT_CRATE_CXX_QT_EXPORT_DIR}/crates/${CRATE}/initializers.o")
    # Note that we need to link using TARGET_OBJECTS, so that the object files are included **transitively**, otherwise
    # Only the linker flags from the object library would be included, but not the actual object files.
    # See also the "Linking Object Libraries" and "Linking Object Libraries via $<TARGET_OBJECTS>" sections:
    # https://cmake.org/cmake/help/latest/command/target_link_libraries.html
    target_link_libraries(${CRATE} INTERFACE ${CRATE}_initializers $<TARGET_OBJECTS:${CRATE}_initializers>)
  endforeach()

endfunction()


function(cxx_qt_import_qml_module target)
  cmake_parse_arguments(QML_MODULE "" "URI;SOURCE_CRATE" "" ${ARGN})

  if (NOT DEFINED QML_MODULE_URI)
    message(FATAL_ERROR "cxx_qt_import_qml_module: URI must be specified!")
  endif()

  if (NOT DEFINED QML_MODULE_SOURCE_CRATE)
    message(FATAL_ERROR "cxx_qt_import_qml_module: SOURCE_CRATE must be specified!")
  endif()

  get_target_property(QML_MODULE_EXPORT_DIR ${QML_MODULE_SOURCE_CRATE} CXX_QT_EXPORT_DIR)
  get_target_property(QML_MODULE_CRATE_TYPE ${QML_MODULE_SOURCE_CRATE} TYPE)

  if (${QML_MODULE_EXPORT_DIR} STREQUAL "QML_MODULE_EXPORT_DIR-NOTFOUND")
    message(FATAL_ERROR "cxx_qt_import_qml_module: SOURCE_CRATE must be a valid target that has been imported with cxx_qt_import_crate!")
  endif()

  # Note: This needs to match the URI conversion in cxx-qt-build
  string(REPLACE "." "_" module_name ${QML_MODULE_URI})
  set(QML_MODULE_DIR "${QML_MODULE_EXPORT_DIR}/qml_modules/${module_name}")
  file(MAKE_DIRECTORY ${QML_MODULE_DIR})

  # QML plugin - init target
  # When using the Ninja generator, we need to provide **some** way to generate the object file
  # Unfortunately I'm not able to tell corrosion that this obj file is indeed a byproduct, so
  # create a fake target for it.
  # This target doesn't need to do anything, because the file should already exist after building the crate.
  add_custom_target(${target}_mock_obj_output
    COMMAND ${CMAKE_COMMAND} -E true
    DEPENDS ${QML_MODULE_SOURCE_CRATE}
    BYPRODUCTS "${QML_MODULE_DIR}/plugin_init.o")

  add_library(${target} OBJECT IMPORTED)
  set_target_properties(${target}
    PROPERTIES
    IMPORTED_OBJECTS "${QML_MODULE_DIR}/plugin_init.o")
  target_link_libraries(${target} INTERFACE ${QML_MODULE_SOURCE_CRATE})

  message(VERBOSE "CXX-Qt Expects QML plugin: ${QML_MODULE_URI} in directory: ${QML_MODULE_DIR}")
endfunction()
