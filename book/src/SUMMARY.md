<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Summary

[Introduction](./index.md)

---

- [Getting Started](./getting-started/index.md)
    - [QObjects in Rust](./getting-started/1-qobjects-in-rust.md)
    - [Our first CXX-Qt module](./getting-started/2-our-first-cxx-qt-module.md)
    - [Creating the QML GUI](./getting-started/3-qml-gui.md)
    - [Building with CMake](./getting-started/4-cmake-integration.md)
    - [Building with Cargo](./getting-started/5-cargo-executable.md)
- [QObject](./qobject/index.md)
    - [`#[cxx_qt::bridge]` - Bridge Macro](./qobject/bridge-macro.md)
    - [`#[cxx_qt::qobject]` - Defining QObjects](./qobject/qobject_struct.md)
    - [`#[cxx_qt::qsignals]` - Signals macro](./qobject/signals.md)
    - [`qobject::T` - The generated QObject](./qobject/generated-qobject.md)
    - [CxxQtThread](./qobject/cxxqtthread.md)
- [Concepts](./concepts/index.md)
    - [Bridge](./concepts/bridge.md)
    - [Qt](./concepts/qt.md)
    - [Types](./concepts/types.md)
    - [Build Systems](./concepts/build_systems.md)
    - [Threading](./concepts/threading.md)
    - [Nested Objects](./concepts/nested_objects.md)
    - [Inheritance & Overriding](./concepts/inheritance.md)
