<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Building with CMake

```diff,ignore
- Disclaimer: The CMake integration for CXX-Qt is still work-in-progress.
- The current state is far from optimal and will likely improve a lot
- in the future, so don't be discouraged by anything in this chapter.
- Contributions are also welcome.
```

Before we can get started on building Qt with CMake, we first need to make our Cargo build ready for it.
If you've generated your project with the `cargo new --lib` command, your `Cargo.toml` likely looks something like this:
```toml,ignore
[package]
name = "cxx-qt-getting-started"
version = "0.1.0"
edition = "2021"

[dependencies]
```

We'll have to do multiple things:
- Instruct cargo to create a static lib with a defined name ("rust") for CMake to link against.
- Add `cxx`, `cxx-qt`, as well as `cxx-qt-lib` as dependencies.
- Add `clang-format` and `cxx-qt-build` as build-dependencies.

In the end, your `Cargo.toml` should look like this:
```toml,ignore
[package]
name = "cxx-qt-getting-started"
version = "0.1.0"
edition = "2021"

# This will instruct Cargo to create a static
# lib named "rust" which CMake can link against
[lib]
name = "rust"
crate-type = ["staticlib"]

[dependencies]
cxx = "1.0"
cxx-qt = "0.2"
cxx-qt-lib = "0.2"

# cxx-qt needs to be able to generate C++ code at
# compile time, which is what cxx-qt-build is needed for.
# cxx-qt uses clang-format, if available, to format all
# C++ code in a consistent manner.
[build-dependencies]
clang-format = "0.1"
cxx-qt-build = "0.2"
```

We'll then also need to add a script named `build.rs` next to our `Cargo.toml`:
```rust,ignore
{{#include ../../../examples/qml_minimal/build.rs:book_build_rs}}
```
This is what generates the C++ code for our `MyObject` class at compile-time.
It will output the `cxx-qt-gen/include/my_object.h` file we included earlier in `main.cpp`.

Note that all Rust source files that uses the `#[make_qobject]` macro need to be included in this script!
In our case, this is only the `src/lib.rs` file.

Then we can write our `CMakeLists.txt` file:

```cmake,ignore
cmake_minimum_required(VERSION 3.16)

project(cxx_qt_getting_started)
set(APP_NAME ${PROJECT_NAME})

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Gui Qml QuickControls2 QuickTest Test REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Qml QuickControls2 QuickTest Test REQUIRED)

# Include the CXX-Qt CMake code, which provides some easy functions
# to generate the CXX-Qt code.
include(CxxQt)

# Generate C++ code from Rust using Cargo in the current folder
cxx_qt_generate_cpp(GEN_SOURCES)

# Define our sources
set(
    CPP_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp
)

set(
    RESOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/qml.qrc
)

# Define our executable with our C++ source, generated sources, and QML resource files
add_executable(${APP_NAME} "${CPP_SOURCES}" "${GEN_SOURCES}" "${RESOURCES}")

# Include generated sources
cxx_qt_include(${APP_NAME})

# Link to generated rust library
cxx_qt_link_rustlib(${APP_NAME})

# Link to Qt in the normal way
target_link_libraries(${APP_NAME} PRIVATE
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
    Qt${QT_VERSION_MAJOR}::Qml
    Qt${QT_VERSION_MAJOR}::QuickControls2
)
```

This looks like a lot, but it is actually just a fairly standard CMake file for building a Qt application.

The difference here are these lines:
```cmake,ignore
include(CxxQt)

# Generate C++ code from Rust using Cargo in the current folder
cxx_qt_generate_cpp(GEN_SOURCES)

# Include generated sources
cxx_qt_include(${APP_NAME})

# Link to generated rust library
cxx_qt_link_rustlib(${APP_NAME})
```

Which will do the code generation and include it into the C++ build.

An important thing to note here is that CMake must be able to resolve the call to `include(CxxQt)`.
For this to work, you'll want to clone the [CXX-Qt repository](https://github.com/KDAB/cxx-qt/) and add the `CxxQt.cmake` file to the `CMAKE_MODULE_PATH` CMake variable.
An easy way to achieve this is by using CMake's `-D` option.
For some alternatives, see the [CMake concepts chapter](../concepts/cmake.md).

Therefore building our project can be done like this:
```shell
$ mkdir build && cd build
$ cmake -DCMAKE_MODULE_PATH="<path-to-cxx-qt-repo>/cmake" ..
$ cmake --build .
```
If this fails for any reason, take a look at the [`examples/qml_minimal`](https://github.com/KDAB/cxx-qt/tree/main/examples/qml_minimal) folder, which contains the complete example code.

This should now configure and compile our project.
If this was successful, you can now run our little project.
```shell
$ ./cxx_qt_getting_started
```

You should now see the two Labels that display the state of our `MyObject`, as well as the two buttons to call our two Rust functions.

## Success   🥳

For further reading, you can take a look at the [QObject chapter](../qobject/index.md) which goes into detail about all features that CXX-Qt exposes to new QObject subclasses.
As well as the [Concepts chapter](../concepts/index.md), which explains the under concepts underlying CXX-Qt.
