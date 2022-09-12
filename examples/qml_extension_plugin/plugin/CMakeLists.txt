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
        GIT_TAG v0.2.1
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
target_include_directories(${CRATE} INTERFACE "${CXXQT_EXPORT_DIR}/${CRATE}")

set(QML_IMPORT_DIR ${CMAKE_CURRENT_BINARY_DIR}/../qml)
set(PLUGIN_OUTPUT_DIR ${QML_IMPORT_DIR}/com/kdab/cxx_qt/demo)

# Define our QML plugin library
add_library(core_qmlplugin SHARED cpp/plugin.cpp)
# Set directory for our library
set_target_properties(core_qmlplugin PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${PLUGIN_OUTPUT_DIR}
)

# Link to Rust library and Qt
target_link_libraries(core_qmlplugin PRIVATE
    ${CRATE}
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
    Qt${QT_VERSION_MAJOR}::Qml
    Qt${QT_VERSION_MAJOR}::QuickControls2
)

# Install the qmldir file into the plugin folder
configure_file(${CMAKE_CURRENT_SOURCE_DIR}/qmldir ${PLUGIN_OUTPUT_DIR}/qmldir COPYONLY)
# ANCHOR_END: book_cmake_generation
