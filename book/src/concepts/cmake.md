<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CMake

We need to add cmake to generate the C++ code and then link to it, ensure that `CxxQt.cmake` is in CMake's path.

Then we have multiple phases to perform in the CMake

  * `cxx_qt_generate_cpp`
    * Uses the Cargo.toml file in the current directory
    * Parses the Rust project generating relevant C++ code
    * Lits the sources into `GEN_SOURCES`
  * `add_executable`
    * Add the generated C++ sources to the executables as in a normal C++ project
  * `cxx_qt_include`
    * Adds any static sources from CXX-Qt and CXX that need to be in the include directories
  * `cxx_qt_link_rustlib`
    * Links the static Rust library to the given C++ target

```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_generation}}
```

See the [QQmlExtensionPlugin page](./qqmlextensionplugin.md) for CMake differences when building a plugin.
