<!--
SPDX-FileCopyrightText: 2021-2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CXX-Qt

CXX-Qt is a set of Rust crates for creating bidirectional Rust ⇄ C++ bindings with [Qt](https://www.qt.io/).
It can be used to integrate Rust into C++ applications using CMake or used to build Rust applications with Cargo.
CXX-Qt provides tools for implementing [QObject](https://doc.qt.io/qt-6/object.html) subclasses in Rust which can
be used from C++, QML, and JavaScript. It consists of two parts:

  * cxx-qt-lib, a library of Rust bindings to common QtCore and QtGui classes made with [CXX](https://cxx.rs/)

  * cxx-qt & cxx-qt-build, a pair of Rust & C++ code generators which are a superset of CXX plus additional attributes
    to interface with Qt's [signals & slots](https://doc.qt.io/qt-6/signalsandslots.html) and [property system](https://doc.qt.io/qt-6/properties.html).
    The cxx-qt crate implements a macro for Rust code generation. cxx-qt-build is used in [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
    to generate and compile the corresponding C++ code.

The [CXX-Qt Book](https://kdab.github.io/cxx-qt/book/getting-started/index.html) walks through a minimal example
step-by-step and documents CXX-Qt's features for the latest release. The [examples folder](./examples) contains
demonstrations of using threading, QQmlExtensionPlugin, and various other features.

CXX-Qt is tested on CI on Linux, Windows, and macOS (all on x86). It should work on other platforms that Qt and
Rust both support, however, these are not tested regularly.

CXX-Qt is in early development and the API changes frequently. For the latest documentation between releases, [install mdBook](https://rust-lang.github.io/mdBook/guide/installation.html)
and run `mdbook serve` in the [book folder](./book).

## Comparison to other Rust Qt bindings

| Project | Integrate into C++ codebase  | Safe Rust | QML | QWidgets | Maintained<sup>1</sup> | Binding mechanism |
|-------- | ---------------------------- | --------- | --- | -------- | ---------------------- | ----------------- |
| CXX-Qt  |  ✔                           | ✔         | ✔ | limited<sup>2</sup> | ✔       | [cxx](https://cxx.rs) plus additional code generation to implement QObject subclasses in Rust and bind them to C++ |
| [qmetaobject](https://github.com/woboq/qmetaobject-rs/) | ✗ | ✔ | ✔ | ✗ | ✔ | [cpp](https://github.com/mystor/rust-cpp) macro to write C++ inline in Rust, plus Rust macros to create QObject subclasses from Rust structs |
| [Rust Qt Binding Generator](https://invent.kde.org/sdk/rust-qt-binding-generator) | ✔ | ✔ | ✔ | limited<sup>2</sup> | ✗ | generates Rust traits and C++ bindings from JSON description of QObject subclass |
| [rust-qt](https://rust-qt.github.io/) | ✗ | ✗ | ✔ | ✔ | ✗ | [ritual](https://rust-qt.github.io/ritual/) to generate unsafe Rust bindings from C++ headers |
| [qml-rust](https://github.com/White-Oak/qml-rust) | ✗ | ✔ | ✔ | ✗ | ✗ | [DOtherSide](https://github.com/filcuc/DOtherSide) C wrapper for QML C++ classes |
| [qmlrs](https://github.com/flanfly/qmlrs) | ✗ | ✔ | ✔ | ✗ | ✗ | own C++ library to bind QQmlApplicationEngine |
| [qmlrsng](https://github.com/nbigaouette/qmlrsng) | ✗ | ✔ | ✔ | ✗ | ✗ | [libqmlbind](https://github.com/seanchas116/libqmlbind) with [bindgen](https://rust-lang.github.io/rust-bindgen/) |
| [rust-qml](https://github.com/florianjacob/rust-qml) | ✗ | ✔ | ✔ | ✗ | ✗ | [libqmlbind](https://github.com/seanchas116/libqmlbind) |

<sup>1</sup>: maintained: supports Qt6 and repository has had nontrivial commits within last year as of August 2022

<sup>2</sup>: CXX-Qt and Rust Qt Binding Generator can be used to implement custom QObjects subclasses in Rust. C++
bindings for these QObject subclasses can be used in QWidgets applications, but these projects do not provide Rust
bindings for QWidgets APIs.

## Contributing to CXX-Qt

### Building

Ensure that you have the following installed

  * C++ compiler
  * [clang-format](https://clang.llvm.org/docs/ClangFormat.html)
  * [CMake v3.16+](https://cmake.org/)
  * [Qt 5 and/or Qt 6](https://www.qt.io/)
  * [Rust toolchain](https://www.rust-lang.org/)
  * [mold](https://github.com/rui314/mold) or [lld](https://lld.llvm.org/) for Linux (lld is included in the XCode toolchain on macOS)

This repository's build system uses CMake, which calls Cargo under the hood to build all the
examples and tests. One example can be built and run with Cargo directly without using CMake:
`cargo run -p qml-minimal-no-cmake` (this example is also built in the CMake build). This
example does not link with GNU ld; [using mold](https://github.com/rui314/mold#how-to-use) or lld
is required on Linux.

On Windows and macOS, CXX-Qt defaults to installing Qt from vcpkg. Prebuilt packages are
automatically downloaded from GitHub Packages (this will take several minutes the first time
you run CMake). If you already have Qt installed, you can disable this by adding
`-D VCPKG=OFF` to the CMake configure step (the first call to `cmake`).

CXX-Qt defaults to building with Qt6. If you want to build with Qt5 when both are installed,
or you want to tell vcpkg to use Qt5, add `-D QT_DEFAULT_MAJOR_VERSION=5` to the CMake
configure step.

```bash
cmake -S . -B build
cmake --build build
```

### Run the basic QML example

```bash
./build/examples/qml_minimal/example_qml_minimal
```

### Testing
Testing assumes that `cargo clippy` and `cargo fmt` are available, you may need to install these with `rustup component add clippy rustfmt`.

For testing the book, it assumes that [`mdbook` and `mdbook-linkcheck`](https://rust-lang.github.io/mdBook/guide/installation.html) are installed.

For license and memory testing, it assumes that you have [`reuse`](https://reuse.software/) installed (eg via `pip3 install reuse`) and [`valgrind`](https://valgrind.org/).

```bash
ctest --test-dir build
```

## Licensing

CXX-Qt is Copyright (C) 2022, Klarälvdalens Datakonsult AB, and is available under
the terms of the [MIT](https://github.com/KDAB/cxx-qt/blob/main/LICENSES/MIT.txt)
or the [Apache-2.0](https://github.com/KDAB/cxx-qt/blob/main/LICENSES/Apache-2.0.txt)
licenses.

Contact KDAB at <info@kdab.com> to inquire about additional features or
services related to this project.

CXX-Qt includes these source files, also available under the terms of the MIT license:

* [doctest.h](https://github.com/onqtam/doctest) - the lightest feature-rich C++ single-header testing framework for unit tests and TDD (C) 2016-2021 Viktor Kirilov <vik.kirilov@gmail.com>

The following CMake source files are available under the BSD-3-Clause

* [cmake/CompilerCaching.cmake](./cmake/CompilerCaching.cmake) - a helper for using sccache

# About KDAB

CXX-Qt is supported and maintained by Klarälvdalens Datakonsult AB (KDAB).

The KDAB Group is the global No.1 software consultancy for Qt, C++ and
OpenGL applications across desktop, embedded and mobile platforms.

The KDAB Group provides consulting and mentoring for developing Qt applications
from scratch and in porting from all popular and legacy frameworks to Qt.
We continue to help develop parts of Qt and are one of the major contributors
to the Qt Project. We can give advanced or standard trainings anywhere
around the globe on Qt as well as C++, OpenGL, 3D and more.

Please visit https://www.kdab.com to meet the people who write code like this.
