# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# ANCHOR: book_cmake_generation
find_package(Corrosion QUIET)
if(NOT Corrosion_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.4.1
    )

    FetchContent_MakeAvailable(Corrosion)
endif()

set(CRATE qml-extension-plugin)
corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})
set(CXXQT_EXPORT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxxqt")
corrosion_set_env_vars(${CRATE}
    "CXXQT_EXPORT_DIR=${CXXQT_EXPORT_DIR}"
    "QMAKE=${QMAKE}"
)

# Create an interface for our Rust library
add_library(core_qmlplugin_rust INTERFACE)

# Include the generated headers
target_include_directories(core_qmlplugin_rust INTERFACE "${CXXQT_EXPORT_DIR}/${CRATE}")

target_link_libraries(core_qmlplugin_rust INTERFACE
    # WHOLE_ARCHIVE is needed for the generated QML plugin to register on startup,
    # otherwise the linker will discard the static variables that initialize it.
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,${CRATE}-static>"
    Qt::Core
    Qt::Gui
    Qt::Qml
    Qt::QuickControls2
)

# Define our QML plugin library
add_library(core_qmlplugin SHARED cpp/plugin.cpp)

set(QML_IMPORT_DIR ${CMAKE_CURRENT_BINARY_DIR}/../qml)
set(PLUGIN_OUTPUT_DIR ${QML_IMPORT_DIR}/com/kdab/cxx_qt/demo)

# Set directory for our library
set_target_properties(core_qmlplugin PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${PLUGIN_OUTPUT_DIR}
)

# TODO: test
if(WIN32)
    target_link_libraries(core_qmlplugin_rust INTERFACE ws2_32 wsock32)
endif()

# Link our QQmlExtensionPlugin plugin to the Rust library
target_link_libraries(core_qmlplugin PRIVATE core_qmlplugin_rust)

# Install the qmldir file into the plugin folder
configure_file(${CMAKE_CURRENT_SOURCE_DIR}/qmldir ${PLUGIN_OUTPUT_DIR}/qmldir COPYONLY)
# ANCHOR_END: book_cmake_generation
