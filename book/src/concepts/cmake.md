<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CMake

We need to add CMake to generate the C++ code and then link to it, ensure that `CxxQt.cmake` can be found by CMake.
For this to work, the [`CMAKE_MODULE_PATH` CMake variable](https://cmake.org/cmake/help/latest/variable/CMAKE_MODULE_PATH.html) must be adapted to include the `cmake` folder in the CXX-Qt repository.

Some ways to achieve this include:
- Providing the `-DCMAKE_MODULE_PATH=<path-to-cxx-qt-repo>/cmake` option when calling CMake.
- Adding `list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cxx-qt/cmake")` with the relative path to the CXX-Qt repository.
    - This option is especially useful if CXX-Qt is added as a git submodule to your project.
- Using a CMake GUI to change the variable

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
