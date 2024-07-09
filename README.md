<!--
SPDX-FileCopyrightText: 2021-2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CXX-Qt

[![Github](https://img.shields.io/badge/github-kdab%2Fcxx--qt-informational?logo=github)](https://github.com/kdab/cxx-qt)
[![Book status](https://img.shields.io/github/actions/workflow/status/kdab/cxx-qt/book.yml?label=book&logo=mdbook)](https://kdab.github.io/cxx-qt/book)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/kdab/cxx-qt/github-cxx-qt-tests.yml)
![License (MIT/Apache2.0)](https://img.shields.io/crates/l/cxx-qt)
[![REUSE status](https://api.reuse.software/badge/github.com/KDAB/cxx-qt)](https://api.reuse.software/info/github.com/KDAB/cxx-qt)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

| Crate | Status |
| --- | --- |
| cxx-qt | [![docs.rs](https://img.shields.io/docsrs/cxx-qt?logo=docsdotrs)](https://docs.rs/cxx-qt) [![Crates.io](https://img.shields.io/crates/v/cxx-qt)](https://crates.io/crates/cxx-qt) |
| cxx-qt-build | [![docs.rs](https://img.shields.io/docsrs/cxx-qt-build?logo=docsdotrs)](https://docs.rs/cxx-qt-build) [![Crates.io](https://img.shields.io/crates/v/cxx-qt-build)](https://crates.io/crates/cxx-qt-build) |
| cxx-qt-lib | [![docs.rs](https://img.shields.io/docsrs/cxx-qt-lib?logo=docsdotrs)](https://docs.rs/cxx-qt-lib) [![Crates.io](https://img.shields.io/crates/v/cxx-qt-lib)](https://crates.io/crates/cxx-qt-lib) |
| qt-build-utils | [![docs.rs](https://img.shields.io/docsrs/qt-build-utils?logo=docsdotrs)](https://docs.rs/qt-build-utils) [![Crates.io](https://img.shields.io/crates/v/qt-build-utils)](https://crates.io/crates/qt-build-utils) |

If you want to get in touch with us, feel free to join our Zulip Chat at:
[https://cxx-qt.zulipchat.com](https://cxx-qt.zulipchat.com).
There we openly discuss all things about CXX-Qt development.

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

CXX-Qt is tested on CI on Linux, Windows, and macOS (all on x86_64). It should work on other platforms that Qt and
Rust both support, however, these are not tested regularly.

CXX-Qt is in early development and the API changes frequently. For the latest documentation between releases, [install mdBook](https://rust-lang.github.io/mdBook/guide/installation.html)
and run `mdbook serve --open` in the [book folder](./book). It will open your own browser.
If you need to open it in another browser goto url [http://localhost:3000](http://localhost:3000).

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

[![GitHub repo Good Issues for newbies](https://img.shields.io/github/issues/KDAB/cxx-qt/good%20first%20issue?style=flat&logo=github&logoColor=green&label=Good%20First%20issues)](https://github.com/KDAB/cxx-qt/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) [![GitHub Help Wanted issues](https://img.shields.io/github/issues/KDAB/cxx-qt/help%20wanted?style=flat&logo=github&logoColor=b545d1&label=%22Help%20Wanted%22%20issues)](https://github.com/KDAB/cxx-qt/issues?q=is%3Aopen+is%3Aissue+label%3A%22help+wanted%22) [![GitHub Help Wanted PRs](https://img.shields.io/github/issues-pr/KDAB/cxx-qt/help%20wanted?style=flat&logo=github&logoColor=b545d1&label=%22Help%20Wanted%22%20PRs)](https://github.com/KDAB/cxx-qt/pulls?q=is%3Aopen+is%3Aissue+label%3A%22help+wanted%22) [![GitHub repo Issues](https://img.shields.io/github/issues/KDAB/cxx-qt?style=flat&logo=github&logoColor=red&label=Issues)](https://github.com/KDAB/cxx-qt/issues?q=is%3Aopen)

### Clone the Git repository

This repository contains symbolic links, which requires some setup on Windows 10 before cloning the repository.
First, [enable Windows Developer Mode](https://learn.microsoft.com/en-us/gaming/game-bar/guide/developer-mode)
to avoid needing administrator privileges to create symlinks. Then, enable symlinks in Git:

```shell
git config --global core.symlinks true
```

Now clone the Git repository:

```shell
git clone https://github.com/KDAB/cxx-qt.git
```

### Building

Ensure that you have the following installed

* C++ compiler
* [clang-format](https://clang.llvm.org/docs/ClangFormat.html)
* [CMake](https://cmake.org/)
* [Qt 5 and/or Qt 6](https://www.qt.io/)
* [Rust toolchain](https://www.rust-lang.org/)
* [mold](https://github.com/rui314/mold), [lld](https://lld.llvm.org/), or GNU ld.gold for Linux (lld is included in the XCode toolchain on macOS)

This repository's build system uses CMake, which calls Cargo under the hood to build all the
examples and tests. One example can be built and run with Cargo directly without using CMake:
`cargo run -p qml-minimal-no-cmake` (this example is also built in the CMake build). This
example does not link with GNU ld.bfd which is the default linker on most Linux distributions;
installing [mold](https://github.com/rui314/mold), [lld](https://lld.llvm.org/), or GNU ld.gold
(from GNU binutils but may be separate package) is required on Linux.

On Windows and macOS vcpkg can be used by adding `-D VCPKG=ON` to the CMake configure step to
automatically download release mode packages from GitHub Packages (this will take several minutes the first time you run CMake).
Note that debug symbols are not built in these packages.

CXX-Qt defaults to building with Qt6. If you want to build with Qt5 when both are installed,
or you want to tell vcpkg to use Qt5, add `-D USE_QT5=ON` to the CMake configure step.

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

For testing the book, it assumes that [`mdbook`](https://rust-lang.github.io/mdBook/guide/installation.html) and [`mdbook-linkcheck`](https://github.com/Michael-F-Bryan/mdbook-linkcheck) are installed.

For license and memory testing, it assumes that you have [`reuse`](https://reuse.software/) installed (eg via `pip3 install reuse`) and [`valgrind`](https://valgrind.org/).

```bash
ctest --test-dir build
```

## Licensing

CXX-Qt is Copyright (C) Klarälvdalens Datakonsult AB, and is available under
the terms of the [MIT](https://github.com/KDAB/cxx-qt/blob/main/LICENSES/MIT.txt)
or the [Apache-2.0](https://github.com/KDAB/cxx-qt/blob/main/LICENSES/Apache-2.0.txt)
licenses.

Contact KDAB at <info@kdab.com> to inquire about additional features or
services related to this project.

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

Please visit <https://www.kdab.com> to meet the people who write code like this.
