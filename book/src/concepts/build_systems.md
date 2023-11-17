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

## QML Modules

<!--
TODO: expand and talk about cxx-qt-build with qml_modules etc
-->
