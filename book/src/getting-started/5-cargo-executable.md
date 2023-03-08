<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Building with Cargo

In this example, we will demonstrate how to build the same `cxxqt_object.rs` module and QML file as the
previous example, but without using CMake or another C++ build system. Cargo will do the entire build
just like a typical Rust application.

Note that the folder structure of this example is different to the CMake tutorial, this is because the `Cargo.toml` is now in the root. So there isn't a `rust` folder, instead just a `src` folder and the `.rs` files have moved up one folder.

The complete example code is available in [`examples/cargo_without_cmake`](https://github.com/KDAB/cxx-qt/tree/main/examples/cargo_without_cmake)
in the cxx-qt repository.

## Cargo setup
The Cargo.toml file still requires dependencies to `cxx`, `cxx-qt`, `cxx-qt-lib` and `cxx-qt-build` as in our [CMake example](./4-cmake-integration.md). However, we are not building a `staticlib` this time:

```toml,ignore
{{#include ../../../examples/cargo_without_cmake/Cargo.toml:book_cargo_toml_no_cmake}}
```

> Note that instead of the `*.workspace = true` arguments for the CXX-Qt crates, you should instead use the versions from [crates.io](https://crates.io/search?q=cxx-qt).
> As described in the code comment above each dependency.

The `build.rs` script is similar. However, without CMake, CxxQtBuilder needs to do a bit more work:

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/build.rs:book_cargo_executable_build_rs}}
```

Refer to the [CxxQtBuilder](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.CxxQtBuilder.html)
and [cc::Build](https://docs.rs/cc/latest/cc/struct.Build.html) documentation for further details.

## Rust executable

Instead of a `src/lib.rs` file, this time we need a `src/main.rs` file for Cargo to build the Rust code
as an executable rather than a library. In `src/main.rs`, first import the `cxxqt_object` module and some types we
will need to call C++:

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/src/main.rs:book_cargo_imports}}
```

Define the `main` function that will be called when the executable starts. This works just like starting a QML
application in C++:

  * Create a `QGuiApplication`
  * Create a `QQmlApplicationEngine`
  * Set the QML file path to the engine
  * Run the application

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/src/main.rs:book_cargo_rust_main}}
```

To build and run the application, use `cargo run` within the cxx-qt repository:

```shell
cargo run -p qml-minimal-no-cmake
```
