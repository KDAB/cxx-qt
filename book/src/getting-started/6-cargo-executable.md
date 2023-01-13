<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Building with Cargo

In this example, we will demonstrate how to build the same `cxxqt_object.rs` module and QML file as the
previous example, but without using CMake or another C++ build system. Cargo will do the entire build
just like a typical Rust application. Because cxx-qt does not bind the entire Qt API, we will still
need to write a bit of C++ code. However, we'll use the cxx-qt-build crate to compile it instead of CMake.

Note that the folder structure of this example is different to the CMake tutorial, this is because the `Cargo.toml` is now in the root. So there isn't a `rust` folder, instead just a `src` folder and the `.rs` files have moved up one folder.

The complete example code is available in [`examples/cargo_without_cmake`](https://github.com/KDAB/cxx-qt/tree/main/examples/cargo_without_cmake)
in the cxx-qt repository.

## Cargo setup
The Cargo.toml file still requires dependencies to `cxx`, `cxx-qt`, `cxx-qt-lib` and `cxx-qt-build` as in our [CMake example](./5-cmake-integration.md). However, we are not building a `staticlib` this time:

```toml,ignore
{{#include ../../../examples/cargo_without_cmake/Cargo.toml:book_cargo_toml_no_cmake}}
```

> Note that instead of the `{ path = "..." }` arguments for the CXX-Qt crates, you should instead use the versions from [crates.io](https://crates.io/search?q=cxx-qt).
> As described in the code comment above each dependency.

The `build.rs` script is similar. However, without CMake, CxxQtBuilder needs to do a bit more work:

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/build.rs:book_cargo_executable_build_rs}}
```

Refer to the [CxxQtBuilder](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.CxxQtBuilder.html)
and [cc::Build](https://docs.rs/cc/latest/cc/struct.Build.html) documentation for further details.

## C++ shim

We need to write a small C++ shim to register QML types. Later we will call this from the Rust executable.

First, we need to include Qt headers and the C++ header generated from `src/cxxqt_object.rs`.

```c++,ignore
{{#include ../../../examples/cargo_without_cmake/cpp/register_types.cpp:book_cargo_cpp_includes}}
```

Now create a `registerTypes` method which uses the included QObject to register
with the `QQmlEngine`.

```c++,ignore
{{#include ../../../examples/cargo_without_cmake/cpp/register_types.cpp:book_cargo_register_types}}
```

## Rust executable

Instead of a `src/lib.rs` file, this time we need a `src/main.rs` file for Cargo to build the Rust code
as an executable rather than a library. In `src/main.rs`, first import the `cxxqt_object` module and some types we
will need to call C++:

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/src/main.rs:book_cargo_imports}}
```

Now create a file called `src/qml.rs` this will contain a bridge which allows
us to initialize the Qt resources and register the QML types.

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/src/qml.rs:book_cargo_qml_bridge}}
```

Define the `main` function that will be called when the executable starts.
This performs the following tasks

  * Initialize the Qt resources
  * Create a `QGuiApplication`
  * Create a `QQmlApplicationEngine`
  * Register the QML types to the engine
  * Set the QML file path to the engine
  * Start the application

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/src/main.rs:book_cargo_rust_main}}
```

To build and run the application, use `cargo run` within the cxx-qt repository:

```shell
cargo run -p qml-minimal-no-cmake
```
