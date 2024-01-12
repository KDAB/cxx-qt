<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Core Concepts

CXX-Qt uses [CXX](https://cxx.rs/) for bridging between C++ and Rust safely.

The main purpose of CXX-Qt is to expose Qt's extensions to the C++ language to CXX.

> Note that the Rust `QObject` of a constructed Qt object is owned by the C++ side of the bridge representing it. So when the C++ object is destroyed the Rust object will be destroyed.

- [Supported types between Rust and C++](./types.md)
- [Build Systems](./build_systems.md)
- [Generated QObject](./generated_qobject.md)
- [Nesting Rust objects](./nested_objects.md)
- [Inheriting `QObjects` and overriding methods](./inheritance.md)
