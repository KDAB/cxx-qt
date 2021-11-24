<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# cxx-qt

cxx-qt is a library that automatically generates code to transfer data between Rust and C++ through common interfaces
such as QObjects that can be exposed directly into QML. It relies on the cxx crate internally to achieve this and thus
it is recommended that any interactions with Qt that are not covered by the built-in code generators should be done
directly in C++ and connected to relevant Rust logic by writing additional cxx code. The cxx-qt build system is based
on CMake, but is compatible with cxx on its own as well.

The examples folder contains an example application using the cxx-qt crate and will be used for development and testing
purposes. The cxx-qt folder contains the source for the actual crate which contains a proc-macro. The cxx-qt-gen folder
contains the source for a crate which extracts and generates C++ and Rust source code. The cxx-qt-build folder contains
the source for a crate which provides helper functions to be used in a `build.rs` file.

Initially the projects in the examples folder will also serve as a template for new projects should use cxx-qt.
In future we might improve upon this with a custom CMake module for instance.

# Building

Ensure that you have the following installed

  * C++ compiler
  * [clang-format](https://clang.llvm.org/docs/ClangFormat.html)
  * [CMake v3.16+](https://cmake.org/)
  * [Qt 5](https://www.qt.io/)
  * [Rust](https://www.rust-lang.org/)

## Compiling
In a cxx-qt project, the build system is based on CMake, which uses Cargo under the hood.
Therefore, unlike a typical Rust project, CMake must be used to build cxx-qt.

```bash
mkdir build/
cd build/
cmake ../
cmake --build . -j$(nproc)
```

## Run the basic QML example

```bash
./build/examples/basic_cxx_qt_qml/example_basic_cxx_qt_qml
```

## Testing

Testing assumes that `cargo clippy` and `cargo fmt` are available, you may need to install these with `rustup component add clippy rustfmt`.

It also assumes that you have [`reuse`](https://reuse.software/) installed (eg via `pip3 install reuse`) and [`valgrind`](https://valgrind.org/).

```bash
cd build/
ctest -j$(nproc)
```
