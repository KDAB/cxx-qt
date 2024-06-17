


function(cxxqt_import_qml_module target)
  cmake_parse_arguments(QML_MODULE "" "URI;EXPORT_DIR;SOURCE_CRATE" "" ${ARGN})

  # Note: This needs to match the URI conversion in cxx-qt-build
  string(REPLACE "." "_" plugin_name ${QML_MODULE_URI})

  # QML plugin - init targeth
  set_source_files_properties(
    "${QML_MODULE_EXPORT_DIR}/plugins/${plugin_name}_plugin_init.o"
    PROPERTIES GENERATED ON)
  add_library(${target} OBJECT IMPORTED)
  set_property(TARGET ${target} PROPERTY IMPORTED_OBJECTS
    "${QML_MODULE_EXPORT_DIR}/qml_minimal/plugins/${plugin_name}_plugin_init.o")
  target_link_libraries(${target} INTERFACE ${QML_MODULE_SOURCE_CRATE})
endfunction()
