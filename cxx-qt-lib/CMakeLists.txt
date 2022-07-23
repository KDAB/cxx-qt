# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.4)
project(CxxQt VERSION 0.4.0)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Gui REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui REQUIRED)

include(GNUInstallDirs)
set(PROJECT_INSTALL_INCLUDEDIR "${CMAKE_INSTALL_INCLUDEDIR}/cxx-qt${QT_VERSION_MAJOR}")

add_library(cxxqt
    # Only static linking is supported because this can't be linked
    # into a standalone dynamic library without cxx.cc which the cxx
    # Rust crate builds and links into the application's Rust library
    # with its build.rs.
    STATIC
    src/qcolor_cxx.cpp
    src/qdate_cxx.cpp
    src/qdatetime_cxx.cpp
    src/qpoint_cxx.cpp
    src/qpointf_cxx.cpp
    src/qrect_cxx.cpp
    src/qrectf_cxx.cpp
    src/qsize_cxx.cpp
    src/qsizef_cxx.cpp
    src/qstring_cxx.cpp
    src/qtime_cxx.cpp
    src/qt_types.cpp
    src/qurl_cxx.cpp
    src/qvariant_cxx.cpp
    src/update_requester_cxx.cpp
)

set_target_properties(cxxqt PROPERTIES
    OUTPUT_NAME cxxqt${QT_VERSION_MAJOR}
    SOVERSION ${PROJECT_VERSION}
    VERSION ${PROJECT_VERSION}
)

target_include_directories(cxxqt PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:${PROJECT_INSTALL_INCLUDEDIR}>
)

target_link_libraries(cxxqt PUBLIC
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
)

install(TARGETS cxxqt
    EXPORT CxxQt${QT_VERSION_MAJOR}Targets
    ARCHIVE DESTINATION "${CMAKE_INSTALL_LIBDIR}"
    LIBRARY DESTINATION "${CMAKE_INSTALL_LIBDIR}"
    RUNTIME DESTINATION "${CMAKE_INSTALL_BINDIR}"
)

install(FILES
    include/cxx.h
    include/qcolor_cxx.h
    include/qdate_cxx.h
    include/qdatetime_cxx.h
    include/qpoint_cxx.h
    include/qpointf_cxx.h
    include/qrect_cxx.h
    include/qrectf_cxx.h
    include/qsize_cxx.h
    include/qsizef_cxx.h
    include/qstring_cxx.h
    include/qtime_cxx.h
    include/qt_types.h
    include/qurl_cxx.h
    include/qvariant_cxx.h
    include/update_requester_cxx.h
    DESTINATION "${PROJECT_INSTALL_INCLUDEDIR}"
)

# pkgconfig
if(IS_ABSOLUTE "${CMAKE_INSTALL_LIBDIR}")
    set(PKGCONFIG_LIBDIR "${CMAKE_INSTALL_LIBDIR}")
else()
    set(PKGCONFIG_LIBDIR "\${prefix}/${CMAKE_INSTALL_LIBDIR}")
endif()
if(IS_ABSOLUTE "${CMAKE_INSTALL_INCLUDEDIR}")
    set(PKGCONFIG_INCLUDEDIR "${PROJECT_INSTALL_INCLUDEDIR}")
else()
    set(PKGCONFIG_INCLUDEDIR "\${prefix}/${PROJECT_INSTALL_INCLUDEDIR}")
endif()
configure_file(
    ${CMAKE_CURRENT_SOURCE_DIR}/packaging/cxx-qt.pc.in
    ${CMAKE_CURRENT_BINARY_DIR}/packaging/cxx-qt${QT_VERSION_MAJOR}.pc @ONLY
)
install(
    FILES ${CMAKE_CURRENT_BINARY_DIR}/packaging/cxx-qt${QT_VERSION_MAJOR}.pc
    DESTINATION "${CMAKE_INSTALL_LIBDIR}/pkgconfig"
)

# CMake config
include(CMakePackageConfigHelpers)
set(PROJECT_INSTALL_CMAKEDIR "${CMAKE_INSTALL_LIBDIR}/cmake/CxxQt${QT_VERSION_MAJOR}")
install(
    EXPORT CxxQt${QT_VERSION_MAJOR}Targets
    FILE CxxQt${QT_VERSION_MAJOR}Targets.cmake
    NAMESPACE CxxQt${QT_VERSION_MAJOR}::
    DESTINATION "${PROJECT_INSTALL_CMAKEDIR}"
)
configure_package_config_file(packaging/CxxQtConfig.cmake.in
    "${CMAKE_CURRENT_BINARY_DIR}/packaging/CxxQt${QT_VERSION_MAJOR}Config.cmake"
    INSTALL_DESTINATION "${PROJECT_INSTALL_CMAKEDIR}"
)
write_basic_package_version_file(
    "${CMAKE_CURRENT_BINARY_DIR}/packaging/CxxQt${QT_VERSION_MAJOR}ConfigVersion.cmake"
    VERSION "${CMAKE_PROJECT_VERSION}"
    COMPATIBILITY ExactVersion
)
install(
    FILES
        CxxQt.cmake
        "${CMAKE_CURRENT_BINARY_DIR}/packaging/CxxQt${QT_VERSION_MAJOR}Config.cmake"
        "${CMAKE_CURRENT_BINARY_DIR}/packaging/CxxQt${QT_VERSION_MAJOR}ConfigVersion.cmake"
    DESTINATION "${PROJECT_INSTALL_CMAKEDIR}"
)
