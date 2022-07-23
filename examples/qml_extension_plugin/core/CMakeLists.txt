# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# ANCHOR: book_cmake_generation
set(QML_IMPORT_DIR ${CMAKE_CURRENT_BINARY_DIR}/../qml)
set(PLUGIN_OUTPUT_DIR ${QML_IMPORT_DIR}/com/kdab/cxx_qt/demo)

cxx_qt_create_target(
    TARGET core_qmlplugin_cxxqt
    BRIDGE_MODULE_FILES src/lib.rs
    MANIFEST_PATH Cargo.toml
    CRATES qml-extension-plugin
)

# Define our QML plugin library
add_library(core_qmlplugin SHARED plugin.cpp)
# Set directory for our library
set_target_properties(core_qmlplugin PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${PLUGIN_OUTPUT_DIR}
)

target_link_libraries(core_qmlplugin PRIVATE
    core_qmlplugin_cxxqt
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
    Qt${QT_VERSION_MAJOR}::Qml
    Qt${QT_VERSION_MAJOR}::QuickControls2
)

# Install the qmldir file into the plugin folder
configure_file(${CMAKE_CURRENT_SOURCE_DIR}/qmldir ${PLUGIN_OUTPUT_DIR}/qmldir COPYONLY)
# ANCHOR_END: book_cmake_generation
