<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Build Systems

CXX-Qt can be integrated into existing CMake projects or built with only cargo. The getting started guide provides documentation on how to set up your project:

- [Cargo Integration](../getting-started/4-cargo-executable.md)
- [CMake Integration](../getting-started/5-cmake-integration.md)

CXX-Qt could work with any C++ build system so long as the `QMAKE` and `CXXQT_EXPORT_DIR` environment variables are set before calling Cargo,
as documented in [CMake integration](../getting-started/5-cmake-integration.md). However, using C++ build systems besides CMake with CXX-Qt is untested.

## `CxxQtBuilder`

With both build systems a build script (`build.rs`) file needs to be used,
so that CXX-Qt knows which files to look for bridges and to build a Qt C++ library for linking later.

See [`CxxQtBuilder` documentation](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.CxxQtBuilder.html) for more details.

## QML Modules

When using QML with CXX-Qt [QML modules](https://doc.qt.io/qt-6/qtqml-writing-a-module.html) can be output.
This allows for attributes such as `#[qml_element]` to register the `QObject` with the QML type system without any C++ code.

See [`QmlModule` documentation](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.QmlModule.html) for more details.
