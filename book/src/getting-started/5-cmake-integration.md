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
name = "qml-minimal"
version = "0.1.0"
edition = "2021"

[dependencies]
```

We'll have to do multiple things:
- Instruct cargo to create a static lib with a defined name ("rust") for CMake to link against.
- Add `cxx`, `cxx-qt`, as well as `cxx-qt-lib` as dependencies.
- Add `clang-format` and `cxx-qt-build` as build-dependencies.

In the end, your `Cargo.toml` should look similar to this (note that `path` for the dependencies is not required):
```toml,ignore
{{#include ../../../examples/qml_minimal/rust/Cargo.toml:book_all}}
```

We'll then also need to add a script named `build.rs` next to our `Cargo.toml`:
```rust,ignore
{{#include ../../../examples/qml_minimal/rust/build.rs:book_build_rs}}
```
This is what generates the C++ code for our `MyObject` class at compile-time.
It will output the `cxx-qt-gen/include/my_object.h` file we included earlier in `main.cpp`.

Note that all Rust source files that uses the `#[cxx_qt::bridge]` macro need to be included in this script!
In our case, this is only the `src/lib.rs` file.

Then we can write our `CMakeLists.txt` file:

```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_tutorial_cmake_full}}
```

This looks like a lot, but it is actually just a fairly standard CMake file for building a Qt application.

The difference here are these lines:
```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_tutorial_cmake_diff_1}}
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_tutorial_cmake_diff_2}}
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
$ ./qml_minimal
```

You should now see the two Labels that display the state of our `MyObject`, as well as the two buttons to call our two Rust functions.

## Success   🥳

For further reading, you can take a look at the [QObject chapter](../qobject/index.md) which goes into detail about all features that CXX-Qt exposes to new QObject subclasses.
As well as the [Concepts chapter](../concepts/index.md), which explains the under concepts underlying CXX-Qt.
