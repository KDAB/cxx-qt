# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../../../cmake")

# ANCHOR: book_cmake_generation
include(CxxQt)

set(QML_IMPORT_DIR ${CMAKE_CURRENT_BINARY_DIR}/../qml)
# TODO: generate this path and name from the build.rs then our cmake helpers can read this from .txt
set(PLUGIN_OUTPUT_DIR ${QML_IMPORT_DIR}/com/kdab/cxx_qt/demo)

# Generate C++ code from Rust using Cargo in the current folder
cxx_qt_generate_cpp(GEN_SOURCES)

# Define our QML plugin library
add_library(core_qmlplugin SHARED "${GEN_SOURCES}")
# Set directory for our library
set_target_properties(core_qmlplugin PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${PLUGIN_OUTPUT_DIR}
)

# Include generated sources
cxx_qt_include(core_qmlplugin)
# Link to generated rust library
cxx_qt_link_rustlib(core_qmlplugin)
# Link to Qt libraries
target_link_libraries(core_qmlplugin PRIVATE
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
    Qt${QT_VERSION_MAJOR}::Qml
    Qt${QT_VERSION_MAJOR}::QuickControls2
)

# Install the qmldir file into the plugin folder
configure_file(${CMAKE_CURRENT_SOURCE_DIR}/target/cxx-qt-gen/plugin/qmldir ${PLUGIN_OUTPUT_DIR}/qmldir COPYONLY)
# ANCHOR_END: book_cmake_generation