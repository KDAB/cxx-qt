# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(tests_qt_types_standalone)
set(APP_NAME ${PROJECT_NAME})

# TODO: Add a helper function to our CMake module which automatically
# handles some of this boilerplate for a "typical" Qt application
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Test)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Test REQUIRED)
endif()
get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)

set(CRATE qt-types-standalone)
corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})
corrosion_add_target_generated_headers(${CRATE})
corrosion_set_env_vars(${CRATE} "QMAKE=${QMAKE}")
target_link_libraries(${CRATE} INTERFACE
    Qt::Core
    Qt::Gui
)

add_executable(${APP_NAME}
    cpp/cxxqtconvert.h
    cpp/main.cpp
    cpp/qcolor.h
    cpp/qdate.h
    cpp/qdatetime.h
    cpp/qpoint.h
    cpp/qpointf.h
    cpp/qrect.h
    cpp/qrectf.h
    cpp/qsize.h
    cpp/qsizef.h
    cpp/qstring.h
    cpp/qtime.h
    cpp/qurl.h
    cpp/qvariant.h
)
target_include_directories(${APP_NAME} PRIVATE cpp)

target_link_libraries(${APP_NAME} PRIVATE
    ${CRATE}
    Qt::Test
)
