<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Building with CMake

In this example, we will demonstrate how to integrate CXX-Qt code into a C++ application using CMake.
Cargo builds the CXX-Qt code as a static library, then CMake links it into a C++ executable.

> If you don't want to use CMake, and only want to use Cargo to build your project, refer to the [previous section](./4-cargo-executable.md).

We'll first want to modify our project structure to separate the different parts of our project.

```ignore
tutorial
  - cpp
  - qml
  - rust
```

Move the rust project into the `rust` folder.
Pull out the `qml` folder back to the top level.

## C++ executable

To start our QML application, we'll need a small `main.cpp` file with an ordinary `main` function. Puts this in a `cpp` folder to clearly separate the C++ and Rust code:

```cpp,ignore
{{#include ../../../examples/qml_minimal/cpp/main.cpp:book_main_cpp}}
```

You can add as much C++ code as you want in addition to this.

## Using Rust QObjects in C++

For every `#[cxx_qt::bridge]` that we define in Rust, CXX-Qt will generate a corresponding C++ header file.
To include any of the generated files, use the crates name as the include directory.
The name of the header file will be the folder names, combined with the input rust file name of your `#[cxx_qt::bridge]`, followed by `.cxxqt.h`.
So in our case: `#include <qml_minimal/src/cxxqt_object.cxxqt.h>`

> Note any folders relative to the cargo manifest are considered hence the `src` folder.

Including the generated header allows us to access the `MyObject` C++ class, just like any other C++ class.
Inherit from it, connect signals and slots to it, put it in a QVector, do whatever you want with it.
That's the power of CXX-Qt.

## Cargo setup

Before we can get started on building Qt with CMake, we first need to make our Cargo build ready for it.
If you've generated your project with e.g. `cargo new --lib qml_minimal` or `cargo init --lib [folder]` command, your `Cargo.toml` should look something like this:

```toml,ignore
[package]
name = "qml_minimal"
version = "0.1.0"
edition = "2021"

[dependencies]
```

We'll have to do multiple things:

- Instruct cargo to create a static library
- Add `cxx`, `cxx-qt`, as well as `cxx-qt-lib` as dependencies
- Add `cxx-qt-build` as a build dependency

> If you've already followed the Cargo setup, most of this should already be done.
> Make sure to change the `crate-type` to `"staticlib"` though!

In the end, your `Cargo.toml` should look similar to this.

```toml,ignore
{{#include ../../../examples/qml_minimal/rust/Cargo.toml:book_static_lib}}

[dependencies]
cxx = "1.0.95"
cxx-qt = "0.6"
cxx-qt-lib = "0.6"

# cxx-qt-build generates C++ code from the `#[cxx_qt::bridge]` module
# and compiles it together with the Rust static library
[build-dependencies]
cxx-qt-build = "0.6"

[features]
# This feature must be enabled for `cargo test` when linking Qt 6 statically.
link_qt_object_files = [ "cxx-qt-build/link_qt_object_files" ]
```

We'll then also need to add a script named `build.rs` next to the `Cargo.toml`:
> If you've already followed the Cargo build tutorial, simply modify the existing `build.rs` file.

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/build.rs:book_build_rs}}
```

This is what generates and compiles the C++ code for our `MyObject` class at build time.

Every Rust source file that uses the `#[cxx_qt::bridge]` macro need to be included in this script.
In our case, this is only the `src/cxxqt_object.rs` file.

This is also where the QML module is defined with a QML URI and version.
The files and resources in the module are then exposed in the same way as the [qt_add_qml_module CMake function](https://doc.qt.io/qt-6/qt-add-qml-module.html).

> Note that in order for CXX-Qt to work, the `qmake` executable must be located. This is because CXX-Qt relies on `qmake` to locate the necessary Qt libraries and header files on your system.
>
> Usually, the CMake code that CXX-Qt provides you to import a crate should already take care of this.
>
> To overwrite the path to qmake, you may pass the `QMAKE` option to cxx_qt_import_crate, ensuring that CMake and Cargo use the same Qt binaries.

We'll also need to remove the `src/main.rs` and replace it with a `src/lib.rs` file.
This file only needs to include a single line:

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/lib.rs:book_mod_statement}}
```

This simply ensures that our rust module is included in our library.

Feel free to add additional rust modules in your library as well.

## CMake setup

Now add a `CMakeLists.txt` file in the root of your project folder.
Start the `CMakeLists.txt` file like any other C++ project using Qt.
For this example, we are [supporting both Qt5 and Qt6 with CMake](https://doc.qt.io/qt-6/cmake-qt5-and-qt6-compatibility.html):

```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_setup}}
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_setup-2}}
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_setup-3}}
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_setup-4}}
```

Download CXX-Qts CMake code with FetchContent:

```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_find_cxx_qt_start}}
        GIT_TAG v0.7.0
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_find_cxx_qt_end}}
```

This provides you with a few wrappers around [Corrosion](https://github.com/corrosion-rs/corrosion), a tool for integrating Rust libraries into CMake:

1. `cxx_qt_import_crate` - A wrapper around [corrosion_import_crate](https://corrosion-rs.github.io/corrosion/usage.html). It supports the same arguments as corrosion_import_crate, with two new optional arguments:
    - `CXX_QT_EXPORT_DIR` - Manually specify the path where CXX-Qt artifacts will be exported to.
        - This is usually not necessary. However, if you're importing the same crate with different feature sets in the same CMake build configuration, you will need to specify seperate `CXX_QT_EXPORT_DIR`s to avoid multiple versions of the crate exporting to the same directory.
    - `QMAKE` - Override the path to the QMAKE executable
2. `cxx_qt_import_qml_module` - This function imports a QML modules as a new target. It requires the following arguments:
    - TARGET_NAME - Specify the name of the CMake target that this function will create
    - `URI` - The URI of the qml module to import - this needs to exactly match the URI in the `CxxQtBuilder::qml_module` call in your build script.
    - `SOURCE_CRATE` The crate that exports the QML module (this crate must have been imported with `cxx_qt_import_crate`).

```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_use_cxx_qt}}
```

This will create two new CMake targets:

1. `qml_minimal` - The static library exported by our crate
2. `qml_minimal_qml_module` - The QML Module exported by our crate
    - The `_qml_module` target will automatically link to the `qml_minimal` target, so linking to the `_qml_module` is sufficient for our executable target

Finally, we can create the CMake executable target and link it to our crate:

```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_executable-2}}
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_executable}}
```

Your project should now have a structure similar to this:

```console, ignore
$ tree -I target/ -I tests
.
├── CMakeLists.txt
├── cpp
│   └── main.cpp
├── qml
│   └── main.qml
└── rust
    ├── build.rs
    ├── Cargo.toml
    └── src
        ├── cxxqt_object.rs
        └── lib.rs

5 directories, 7 files
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
$ ./build/examples/qml_minimal/example_qml_minimal
```

You should now see the two Labels that display the state of our `MyObject`, as well as the two buttons to call our two Rust functions.

### Windows with MSVC

If you're building CXX-Qt on Windows using MSVC generator, you need to ensure that `set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")` is set in CMake (or use the `-DCMAKE_MSVC_RUNTIME_LIBRARY=MultiThreadedDLL` flag) when building with the `Debug` configuration. This flag is necessary to ensure that the correct C Runtime Library is used. Then you can build using `cmake --build build --config Debug`.

This issue is caused by a bug in the [cc](https://docs.rs/cc/latest/cc/index.html) crate (as described in [this pull request](https://github.com/rust-lang/cc-rs/pull/717)), which has not been merged yet. Specifically, the problem is that cc generated code always links to the MultiThreaded runtime, even when building in Debug mode. We hope that this step won't be necessary in the future, once the cc crate fix is merged and released.
