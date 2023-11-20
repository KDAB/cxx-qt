<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Building with Cargo

In this example, we will demonstrate how to build the `cxxqt_object.rs` as well as any QML files using the Rust build system.
Cargo will do the entire build, including linking to Qt, just like a typical Rust application.

Note that the folder structure of this example is different to the CMake tutorial.
The CMake example uses a `rust` folder where the Rust part of the project resides in.
In this setup we'll stick with a standard Cargo folder layout with just the added `qml` folder next to the `src` folder.

The complete example code is available in [`examples/cargo_without_cmake`][cargo-without-cmake]
in the CXX-Qt repository.

> If you don't want to use Cargo, and only want to use CMake to build your project, skip ahead to the [next section](./5-cmake-integration.md).
>
> Using a Cargo based setup is easier though, so if in doubt, try building with Cargo first.

## Cargo setup

Add the dependencies to the `Cargo.toml` file.
We'll need `cxx`, `cxx-qt`, `cxx-qt-lib` and `cxx-qt-build`:
```toml,ignore
{{#include ../../../examples/cargo_without_cmake/Cargo.toml:book_cargo_toml_no_cmake}}
cxx = "1.0.95"
cxx-qt = "0.6"
cxx-qt-lib = "0.6"

[build-dependencies]
# The link_qt_object_files feature is required for statically linking Qt 6.
cxx-qt-build = { version = "0.6", features = [ "link_qt_object_files" ] }
```

Now we'll add a `build.rs` script next to the `Cargo.toml` file.

```rust,ignore
{{#include ../../../examples/cargo_without_cmake/build.rs:book_cargo_executable_build_rs}}
```

This is what generates and compiles the C++ code for our `MyObject` class at build time.
It will also link Qt to our Rust binary.

Every Rust source file that uses the `#[cxx_qt::bridge]` macro needs to be included in this script.
In our case, this is only the `src/cxxqt_object.rs` file.

This is also where the QML module is defined with a QML uri and version.
The files and resources in the module are then exposed in the same way as the [qt_add_qml_module CMake function](https://doc.qt.io/qt-6/qt-add-qml-module.html).

Refer to the [CxxQtBuilder](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.CxxQtBuilder.html)
and [cc::Build](https://docs.rs/cc/latest/cc/struct.Build.html) documentation for further details.

## Rust executable

In `src/main.rs`, first import the `cxxqt_object` module and some types we will need to run our Qt application:
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

To build and run the application, use `cargo run`.

> Note that in order for CXX-Qt to work, the `qmake` executable must be located. This is because CXX-Qt relies on `qmake` to locate the necessary Qt libraries and header files on your system.
>
> `cxx-qt` will find qmake in the following order:
> - Look for an environment variable `QMAKE` that should have the path to `qmake`.\
>   e.g.: `QMAKE=/usr/bin/qmake cargo run`
> - Use `qmake` from the `PATH`. If multiple `qmake` exists in `PATH`, environment variable `QT_VERSION_MAJOR` will control the selected one.
>
> To check which version of Qt will be used with `qmake`, you can use the `qmake -query` command. This will display information about the Qt installation, including the version number and installation path.
>
> Check [CxxQtBuilder](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.CxxQtBuilder.html) for more information

If this fails for any reason, take a look at the [`examples/cargo-without-cmake`][cargo-without-cmake] folder in the CXX-Qt repository, which contains the complete example code.

If you have cloned the CXX-Qt repository, you can run this example from within the repository using:

```shell
cargo run -p qml-minimal-no-cmake
```

You should now see the two Labels that display the state of our `MyObject`, as well as the two buttons to call our two Rust functions.

## Success 🥳

For further reading, you can take a look at the [bridge chapter](../bridge/index.md) which goes into detail about all features that CXX-Qt exposes to new QObject subclasses.
As well as the [Concepts chapter](../concepts/index.md), which explains the concepts underlying CXX-Qt.

In the next, optional chapter, we will show how to build the same QML application with CMake.

[cargo-without-cmake]: https://github.com/KDAB/cxx-qt/tree/main/examples/cargo_without_cmake

