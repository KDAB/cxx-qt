# TODO: figure out what the actual hard minimum is
cmake_minimum_required(VERSION 3.16)

project(example_basic_cxx_qt_qml)
set(APP_NAME ${PROJECT_NAME})

# TODO: Add a helper function to our CMake module which automatically
# handles some of this boilerplate for a "typical" Qt application
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Gui Qml QuickControls2 QuickTest Test REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Qml QuickControls2 QuickTest Test REQUIRED)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../../cmake")
include(CxxQt)

set(
    RUST_SOURCES
    lib.rs
)

set(
    CPP_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp
)

set(
    RESOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/qml.qrc
)

# Generate the C++ code
cxx_qt_generate_cpp(${RUST_SOURCES} GEN_SOURCES)

# Define our executable with our QML resource files
add_executable(${APP_NAME} ${CPP_SOURCES} ${GEN_SOURCES} ${RESOURCES})

# Include generated sources
cxx_qt_include(${APP_NAME})
# Link to generated rust library
cxx_qt_link_rustlib(${APP_NAME})

target_link_libraries(${APP_NAME} Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Qml Qt${QT_VERSION_MAJOR}::QuickControls2 Qt${QT_VERSION_MAJOR}::Test)

#
# Unit test
#

# TODO: Once we can make a QQmlExtensionPlugin we won't need this C++ helper
# as we'll be able to import the .so + qmldir from QML directly
#
# When that happens we can then use qmltestrunner directly on the QML test files
# rather than building this extra executable
set(
    CPP_TEST_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/tests/tst_myobject.cpp
)

# Copy QML unit test to build folder
# this is so that our tst_myobject executable can find the QML files
configure_file(${CMAKE_CURRENT_SOURCE_DIR}/src/tests/tst_myobject.qml ${CMAKE_CURRENT_BINARY_DIR}/tst_myobject.qml COPYONLY)

cxx_qt_cmake(${APP_NAME}_test ${RUST_SOURCES} ${CPP_TEST_SOURCES})
target_link_libraries(${APP_NAME}_test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Qml Qt${QT_VERSION_MAJOR}::QuickTest)

add_test(NAME ${APP_NAME}_test COMMAND ${APP_NAME}_test)
