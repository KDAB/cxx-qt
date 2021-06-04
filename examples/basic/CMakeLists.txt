# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(cxx_example_basic)
set(APP_NAME hello)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../../cmake")
include(CxxQt)

set(
    RUST_SOURCES
    lib.rs
)

set(
    CPP_SOURCES
    src/test.cpp
)

cxx_qt_cmake(${APP_NAME} ${RUST_SOURCES} ${CPP_SOURCES} ${CPP_SOURCES})
