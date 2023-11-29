<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Build Systems

CXX-Qt can be integrated into existing CMake projects or built with only cargo. The getting started guide provides documentation on how to setup your project:

  * [Cargo Integration](../getting-started/4-cargo-executable.md)
  * [CMake Integration](../getting-started/5-cmake-integration.md)

CXX-Qt could work with any C++ build system so long as the `QMAKE` and `CXXQT_EXPORT_DIR` environment variables are set before calling Cargo,
as documented in [CMake integration](../getting-started/5-cmake-integration.md). However, using C++ build systems besides CMake with CXX-Qt is untested.

## CxxQtBuilder

With both build systems a build script (`build.rs`) file needs to be used,
so that CXX-Qt knows which files to look for bridges and to build a Qt C++ library for linking later.

See [`CxxQtBuilder` documentation](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.CxxQtBuilder.html) for more details.

## QML Modules

When using QML with CXX-Qt [QML modules](https://doc.qt.io/qt-6/qtqml-writing-a-module.html) can be output.
This allows for attributes such as `#[qml_element]` to register the QObject with the QML type system without any C++ code.

See [`QmlModule` documentation](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.QmlModule.html) for more details.

## Splitting a project into multiple crates

As your project grows, it can be helpful to organize your code into multiple Rust crates. If your `main` function is
in Rust ([Cargo is your only build system](../getting-started/4-cargo-executable.md)), simply add the crates as
dependencies of the top level binary crate in its Cargo.toml file.

If your `main` function is in C++, you can only link one staticlib Rust crate into C++, otherwise linking
would fail with duplicate symbol errors from multiple Rust runtimes. So, create one top level staticlib crate to link
into the C++ application. Specify your other crates as normal Rust library (rlib) dependencies
in the staticlib crate's Cargo.toml. You must reference the symbols of the Rust dependencies within the staticlib crate;
if you don't need those symbols in Rust code, you can add `extern crate crate_name;` statements in the staticlib's lib.rs file.
Refer to the [meta_project example](https://github.com/KDAB/cxx-qt/blob/main/examples/meta_project) for how to set this up.
Note that this requires Rust compiler features that were [stabilized](https://github.com/rust-lang/rust/pull/113301)
in Rust 1.74.
