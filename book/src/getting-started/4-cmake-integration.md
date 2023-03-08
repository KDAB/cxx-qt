<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Building with CMake

In this example, we will demonstrate how to integrate cxx-qt code into a C++ application. Cargo builds the cxx-qt code
as a static library, then CMake links it into a C++ executable.

> If you don't want to use CMake, and only want to use Cargo to build your project, you can [skip to the next chapter](./5-cargo-executable.md).

## C++ executable

To start our QML application, we'll need a small `main.cpp` file with an ordinary `main` function. Puts this in a `cpp` folder to clearly separate the C++ and Rust code:
```cpp,ignore
{{#include ../../../examples/qml_minimal/cpp/main.cpp:book_main_cpp}}
```

You can add as much C++ code as you want in addition to this.

## Using Rust QObjects in C++

For every `#[cxx_qt::bridge]` that we define in Rust, CXX-Qt will generate a corresponding C++ header file.
They will always be in the `cxx-qt-gen/` include path and use the snake_case naming convention.
The name of the header file will be the name of the Rust module of your `#[cxx_qt::bridge]`, followed by `.cxxqt.h`.
So in our case: `#include cxx-qt-gen/my_object.cxxqt.h`

Including the generated header allows accessing the `MyObject` C++ class, just like any other C++ class.
Inherit from it, connect signals and slots to it, put it in a QVector, do whatever you want with it.
That's the power of CXX-Qt.

## Cargo setup
Before we can get started on building Qt with CMake, we first need to make our Cargo build ready for it.
If you've generated your project with the `cargo new --lib` or `cargo init --lib folder` command, your `Cargo.toml` likely looks something like this:
```toml,ignore
[package]
name = "qml-minimal"
version = "0.1.0"
edition = "2021"

[dependencies]
```

We'll have to do multiple things:
- Instruct cargo to create a staticlib
- Add `cxx`, `cxx-qt`, as well as `cxx-qt-lib` as dependencies
- Add `cxx-qt-build` as a build dependency

In the end, your `Cargo.toml` should look similar to this.

```toml,ignore
{{#include ../../../examples/qml_minimal/rust/Cargo.toml:book_all}}
```

> Note that instead of the `*.workspace = true` arguments for the CXX-Qt crates, you should instead use the versions from [crates.io](https://crates.io/search?q=cxx-qt).
> As described in the code comment above each dependency.

We'll then also need to add a script named `build.rs` next to our `Cargo.toml`:
```rust,ignore
{{#include ../../../examples/qml_minimal/rust/build.rs:book_build_rs}}
```
This is what generates and compiles the C++ code for our `MyObject` class at build time.

Every Rust source file that uses the `#[cxx_qt::bridge]` macro need to be included in this script.
In our case, this is only the `src/cxxqt_object.rs` file.

## CMake setup

Now add a `CMakeLists.txt` file in the root of the `tutorial` folder. Start the `CMakeLists.txt` file like any other C++ project using Qt. For this example, we are [supporting both
Qt5 and Qt6 with CMake](https://doc.qt.io/qt-6/cmake-qt5-and-qt6-compatibility.html):

```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_setup}}
```

From the Qt CMake target, find the location of the qmake executable. cxx-qt-build will need this later.
```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_find_qmake}}
```

Locate [Corrosion](https://github.com/corrosion-rs/corrosion), a tool for integrating Rust libraries into CMake.
If Corrosion is not installed, automatically download it:

```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_find_corrosion}}
```

Use Corrosion to create a CMake library target for the Rust library. cxx-qt requires a few more steps beyond using
a typical Rust library with Corrosion:
```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_use_corrosion}}
```

Finally, create the CMake executable target and link it to the Rust library:

```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_executable}}
```

Build the project like any other CMake project:

```shell
$ cmake -S . -B build
$ cmake --build build
```
If this fails for any reason, take a look at the [`examples/qml_minimal`](https://github.com/KDAB/cxx-qt/tree/main/examples/qml_minimal) folder, which contains the complete example code.

This should now configure and compile our project.
If this was successful, you can now run our little project.
```shell
$ build/examples/qml_minimal/example_qml_minimal
```

You should now see the two Labels that display the state of our `MyObject`, as well as the two buttons to call our two Rust functions.

## Success 🥳

For further reading, you can take a look at the [QObject chapter](../qobject/index.md) which goes into detail about all features that CXX-Qt exposes to new QObject subclasses.
As well as the [Concepts chapter](../concepts/index.md), which explains the concepts underlying CXX-Qt.

In the next, optional chapter, we will show how to build the same QML application with Cargo without needing CMake.
