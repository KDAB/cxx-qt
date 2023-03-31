# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# This CMakeLists.txt does not build any C++ directly. It is only here to integrate building
# this crate into the larger CMake build of this repository. For development and testing, you
# can run
#
# cargo build -p qml-minimal-no-cmake
#
# from any directory in this repository to build this crate without using CMake at all.

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner REQUIRED)
endif()
get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)

# Use Corrosion to create a target for the crate that can be built without CMake.
# This allows it to share built dependencies with the other crates in the workspace.
find_package(Corrosion QUIET)
if(NOT Corrosion_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.3.5
    )

    FetchContent_MakeAvailable(Corrosion)
endif()

set(CRATE qml-minimal-no-cmake)
corrosion_import_crate(MANIFEST_PATH Cargo.toml CRATES ${CRATE})
corrosion_set_env_vars(${CRATE} "QMAKE=${QMAKE}")
