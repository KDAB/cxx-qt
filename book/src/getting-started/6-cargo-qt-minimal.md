<!--
SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Building with Qt Minimal

In this example, we will demostrate how to build the `cxxqt_object.rs` as well as any QML files using the Rust build system.
Cargo will do the entire build, including downloding and linking to Qt, just like a typical Rust application.

> Note that Qt Minimal does not provide all Qt features or modules and is primarily designed for Qt QML usage.

Note that the folder structure of this example is different to the CMake tutorial.
The CMake example uses a `rust` folder where the Rust part of the project resides in.
In this setup we'll stick with a standard Cargo folder layout with just the added `qml` folder next to the `src` folder.

The complete example code is available in [`examples/cargo_without_cmake`][cargo-without-cmake]
in the CXX-Qt repository.

> Note the only difference to [Building with Cargo](./4-cargo-executable.md) is the `qt_minimal` feature and `qt-version` dependency

> Note `qt-build-utils` will first check for a system Qt install matching the Qt version requirements first and then download Qt minimal if required.

## Cargo setup

Add the dependencies to the `Cargo.toml` file.
We'll need `cxx`, `cxx-qt`, `cxx-qt-lib`, `cxx-qt-build`, `qt-build-utils`, and `qt-version`:

```toml,ignore
{{#include ../../../examples/qml_minimal/rust/Cargo.toml:book_package_name}}
{{#include ../../../examples/cargo_without_cmake/Cargo.toml:book_cargo_toml_no_cmake}}
cxx = "1.0.95"
cxx-qt = "0.8"
cxx-qt-lib = { version="0.8", features = ["qt_full"] }

[build-dependencies]
# The link_qt_object_files feature is required for statically linking Qt 6.
cxx-qt-build = { version = "0.8", features = [ "link_qt_object_files" ] }

# Enable the qt_minimal feature in qt-build-utils
# this allows for automatically downloading Qt when not found with qmake
qt-build-utils = { version = "0.8", features = ["qt_minimal"] }
# Indicate which Qt version is required via features
qt-version = { version = "0.1", features = ["qt_version_at_least_6_10"] }
```

Now we'll add a `build.rs` script next to the `Cargo.toml` file.

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/build.rs:book_cargo_executable_build_rs}}
```

This is what generates and compiles the C++ code for our `MyObject` class at build time.
It will also link Qt to our Rust binary.

Every Rust source file that uses the `#[cxx_qt::bridge]` macro needs to be included in this script.
In our case, this is only the `src/cxxqt_object.rs` file.

This is also where the QML module is defined with a QML URI and version.
The files and resources in the module are then exposed in the same way as the [`qt_add_qml_module` CMake function](https://doc.qt.io/qt-6/qt-add-qml-module.html).

Refer to the [`CxxQtBuilder`](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.CxxQtBuilder.html)
and [`cc::Build`](https://docs.rs/cc/latest/cc/struct.Build.html) documentation for further details.

## Rust executable

In `src/main.rs`, first import the `cxxqt_object` module and some types we will need to run our Qt application:

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/src/main.rs:book_cargo_imports}}
```

Define the `main` function that will be called when the executable starts. This works just like starting a QML
application in C++:

- Create a `QGuiApplication`
- Create a `QQmlApplicationEngine`
- Set the QML file path to the engine
- Run the application

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/src/main.rs:book_cargo_rust_main}}
```

To build and run the application, use `cargo run`.

If this fails for any reason, take a look at the [`examples/cargo-without-cmake`][cargo-without-cmake] folder in the CXX-Qt repository, which contains the complete example code.

If you have cloned the CXX-Qt repository, you can run this example from within the repository using:

```shell
cargo run -p qml-minimal-no-cmake
```

You should now see the two Labels that display the state of our `MyObject`, as well as the two buttons to call our two Rust functions.

## Success 🥳

For further reading, you can take a look at the [bridge chapter](../bridge/index.md) which goes into detail about all features that CXX-Qt exposes to new `QObject` subclasses.
As well as the [Concepts chapter](../concepts/index.md), which explains the concepts underlying CXX-Qt.

[cargo-without-cmake]: https://github.com/KDAB/cxx-qt/tree/main/examples/cargo_without_cmake
