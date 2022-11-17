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

We need to write a small C++ shim to start the QML application. Later we will call this from the Rust executable.
This looks mostly the same as starting a normal C++ application with a QML GUI with a few small changes.

First, we need to include Qt headers, the C++ header generated from `src/cxxqt_object.rs`, and C++ code generated
from the `qml.qrc` file:

```c++,ignore
{{#include ../../../examples/cargo_without_cmake/cpp/run.cpp:book_cargo_cpp_includes}}
```

Instead of the `main` function in a typical C++ application, write an `extern "C"` function which we will call
from Rust:

```c++,ignore
{{#include ../../../examples/cargo_without_cmake/cpp/run.cpp:book_cargo_run_cpp}}
```

In this function, we need to initialize the Qt resource system:

```c++,ignore
{{#include ../../../examples/cargo_without_cmake/cpp/run.cpp:book_cargo_init_qrc}}
```

Then, register the QML type and run the QML file just like a C++ program:

```c++,ignore
{{#include ../../../examples/cargo_without_cmake/cpp/run.cpp:book_cargo_run_qml}}
```

## Rust executable

Instead of a `src/lib.rs` file, this time we need a `src/main.rs` file for Cargo to build the Rust code
as an executable rather than a library. In `src/main.rs`, first import the `cxxqt_object` module and some types we
will need to call C++:

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/src/main.rs:book_cargo_imports}}
```

Tell the linker to link this Rust code with the `run_cpp` function from the `src/cpp/run.cpp` file:

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/src/main.rs:book_cargo_extern_c}}
```

Define the `main` function that will be called when the executable starts. QGuiApplication's C++ constructor expects
the command line arguments from a C++ `main` function to support [command line arguments common to all Qt programs](https://doc.qt.io/qt-6/qguiapplication.html#supported-command-line-options).
However, Rust does not represent command line arguments the same way as C++, so some conversion is needed before passing
the command line arguments to C++:

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/src/main.rs:book_cargo_rust_main}}
```

You can add as much Rust code to setup your application as you want before calling the `run_cpp` function.

To build and run the application, use `cargo run` within the cxx-qt repository:

```shell
cargo run -p qml-minimal-no-cmake
```
