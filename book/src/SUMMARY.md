<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Summary

[Introduction](./index.md)

---

- [Getting-Started](./getting-started/index.md)
    - [QObjects in Rust](./getting-started/1-qobjects-in-rust.md)
    - [Our first CXX-Qt module](./getting-started/2-our-first-cxx-qt-module.md)
    - [Exposing to QML](./getting-started/3-exposing-to-qml.md)
    - [Creating the QML GUI](./getting-started/4-qml-gui.md)
    - [Building with CMake](./getting-started/5-cmake-integration.md)
    - [Building with Cargo](./getting-started/6-cargo-executable.md)
- [QObject](./qobject/index.md)
    - [`#[cxx_qt::bridge]` - Bridge Macro](./qobject/bridge-macro.md)
    - [`#[cxx_qt::qobject]` - Defining QObjects](./qobject/qobject_struct.md)
    - [`#[cxx_qt::signals]` - Signals enum](./qobject/signals_enum.md)
    - [`qobject::T` - The generated QObject](./qobject/cpp_object.md)
    - [CxxQtThread](./qobject/cxxqtthread.md)
- [Concepts](./concepts/index.md)
    - [Bridge](./concepts/bridge.md)
    - [Qt](./concepts/qt.md)
    - [Types](./concepts/types.md)
    - [Type Conversions](./concepts/type-conversions.md)
    - [Build Systems](./concepts/build_systems.md)
    - [Register Types](./concepts/register_types.md)
    - [Threading](./concepts/threading.md)
    - [Nested Objects](./concepts/nested_objects.md)
