# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

project(tests_qt_types_standalone)
set(APP_NAME ${PROJECT_NAME})

# Rust always links against non-debug Windows runtime on *-msvc targets
# Note it is best to set this on the command line to ensure all targets are consistent
# https://github.com/corrosion-rs/corrosion/blob/master/doc/src/common_issues.md#linking-debug-cc-libraries-into-rust-fails-on-windows-msvc-targets
# https://github.com/rust-lang/rust/issues/39016
if (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
endif()

# TODO: Add a helper function to our CMake module which automatically
# handles some of this boilerplate for a "typical" Qt application
set(CMAKE_AUTOMOC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Qml Test)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Qml Test REQUIRED)
endif()
get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)

set(CRATE qt-types-standalone)
corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})
set(CXXQT_EXPORT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxxqt")
corrosion_set_env_vars(${CRATE}
    "CXXQT_EXPORT_DIR=${CXXQT_EXPORT_DIR}"
    "QMAKE=${QMAKE}"
)
target_include_directories(${CRATE} INTERFACE "${CXXQT_EXPORT_DIR}/${CRATE}")
target_link_libraries(${CRATE} INTERFACE
    Qt::Core
    Qt::Gui
    Qt::Qml
)

add_executable(${APP_NAME}
    cpp/main.cpp
    cpp/qbytearray.h
    cpp/qcolor.h
    cpp/qcoreapplication.h
    cpp/qdate.h
    cpp/qdatetime.h
    cpp/qguiapplication.h
    cpp/qhash.h
    cpp/qlinef.h
    cpp/qlist.h
    cpp/qmap.h
    cpp/qmargins.h
    cpp/qmarginsf.h
    cpp/qmetaobjectconnection.h
    cpp/qmodelindex.h
    cpp/qpersistentmodelindex.h
    cpp/qpoint.h
    cpp/qpointf.h
    cpp/qqmlapplicationengine.h
    cpp/qqmlengine.h
    cpp/qrect.h
    cpp/qrectf.h
    cpp/qset.h
    cpp/qsize.h
    cpp/qsizef.h
    cpp/qstring.h
    cpp/qstringlist.h
    cpp/qtime.h
    cpp/qtimezone.h
    cpp/qurl.h
    cpp/qvariant.h
    cpp/qvector.h
    cpp/qvector2d.h
    cpp/qvector3d.h
    cpp/qvector4d.h
)
target_include_directories(${APP_NAME} PRIVATE cpp)

target_link_libraries(${APP_NAME} PRIVATE
    ${CRATE}
    Qt::Test
)
