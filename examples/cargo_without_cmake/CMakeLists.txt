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

add_custom_target(qml-minimal-no-cmake ALL
    cargo build -p qml-minimal-no-cmake
    COMMENT "Building qml-minimal-no-cmake using cargo")
