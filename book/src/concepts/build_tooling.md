<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Build Tooling

Here we describe how to integrate and build a Rust CXX-Qt project into a CMake Qt project.

A folder structure might look like the following if you have a clear split between Rust backend and QML frontend.

```ignore
src/
 - core/
   - build.rs
   - Cargo.toml
   - src/
     - lib.rs
 - ui/
   - main.qml
   - ui.qrc
 CMakeLists.txt
 main.cpp
```

Also see the examples folder for other ways of building the project.

## build.rs

We need to specify a build.rs so that we can parse the macros and generate relevant C++ code.

The following options are available

  * Deciding the clang-format style of the generated C++ code
  * Indicating which files should be parsed to look for macros
  * Enabling build as a QQmlExtensionPlugin if required

A build.rs script could look like the following

```rust,ignore,noplayground
use clang_format::ClangFormatStyle;
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .file("src/lib.rs")
        .build();
}
```

If you are registering as a plugin, with a different clang-format style, and a non-default C++ namespace it could like the following

```rust,ignore,noplayground
use clang_format::ClangFormatStyle;
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .qqmlextensionplugin("com.kdab.cxx_qt.demo", "myqmlplugin")
        .cpp_format(ClangFormatStyle::Mozilla)
        .cpp_namespace_prefix(vec!["rust"])
        .file("src/lib.rs")
        .build();
}
```

## CMake

We need to add cmake to generate the C++ code and then link to it, ensure that `CxxQt.cmake` is in CMake's path.

```cmake,ignore
include(CxxQt)

# Generate the C++ code
cxx_qt_generate_cpp(GEN_SOURCES)

# Define our executable in the usual way, but with our generated sources
add_executable(${APP_NAME} "${CPP_SOURCES}" "${GEN_SOURCES}" "${RESOURCES}")

# Include generated sources
cxx_qt_include(${APP_NAME})

# Link to generated rust library
cxx_qt_link_rustlib(${APP_NAME})

# Link to Qt in the normal way
target_link_libraries(${APP_NAME} Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Qml Qt${QT_VERSION_MAJOR}::QuickControls2)
```

If you are using a QQmlExtensionPlugin then we build a library and use that in the normal way.

```cmake,ignore
include(CxxQt)

set(QML_IMPORT_DIR ${CMAKE_CURRENT_BINARY_DIR}/qml)
set(PLUGIN_OUTPUT_DIR ${QML_IMPORT_DIR}/com/kdab/cxx_qt/demo)

# Generate the C++ code
cxx_qt_generate_cpp(GEN_SOURCES)

# Define our library with our generated sources
add_library(${APP_NAME}_qmlplugin SHARED "${GEN_SOURCES}")
# Set directory for our library
set_target_properties(${APP_NAME}_qmlplugin PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${PLUGIN_OUTPUT_DIR}
)

# Include generated sources
cxx_qt_include(${APP_NAME}_qmlplugin)
# Link to generated rust library
cxx_qt_link_rustlib(${APP_NAME}_qmlplugin)

target_link_libraries(${APP_NAME}_qmlplugin Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Qml Qt${QT_VERSION_MAJOR}::QuickControls2)

configure_file(${CMAKE_CURRENT_SOURCE_DIR}/target/cxx-qt-gen/plugin/qmldir ${PLUGIN_OUTPUT_DIR}/qmldir COPYONLY)
```

## C++ Registering QML types

There are two options for registering the generated QML types, either as a QQmlExtensionPlugin or registering the types to the engine.

If you are registering the types to the engine, then you can include the generated objects (determined by the name of the Rust module), and register in the normal way.

```cpp,ignore
#include "cxx-qt-gen/include/my_object.h"
...
  qmlRegisterType<cxx_qt::my_object::MyObject>(
    "com.kdab.cxx_qt.demo", 1, 0, "MyObject");
```

If you are using a QQmlExtensionPlugin then ensure the generated library is in the import path.

```cpp,ignore
QQmlApplicationEngine engine;
// Add qml dir in runtime folder to QML import paths
engine.addImportPath(QDir(QCoreApplication::applicationDirPath())
                        .filePath(QStringLiteral("qml")));
```

Then from QML you can include these like a normal C++ module.

```qml,ignore
import com.kdab.cxx_qt.demo 1.0
```
