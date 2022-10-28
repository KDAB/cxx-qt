<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Build Systems

CXX-Qt can be integrated into existing CMake projects or built with only cargo. The getting started guide provides documentation on how to setup your project:

  * [CMake Integration](../getting-started/5-cmake-integration.md)
  * [Cargo Integration](../getting-started/6-cargo-executable.md)

CXX-Qt could work with any C++ build system so long as the `QMAKE` and `GENERATED_HEADER_DIR` environment variables are set before calling Cargo,
as documented in [CMake integration](../getting-started/5-cmake-integration.md). However, using C++ build systems besides CMake with CXX-Qt is untested.
