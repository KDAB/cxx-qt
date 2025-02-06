<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Summary

- [Introduction](./index.md)
- [Getting Started](./getting-started/index.md)
  - [`QObject`s in Rust](./getting-started/1-qobjects-in-rust.md)
  - [Our first CXX-Qt module](./getting-started/2-our-first-cxx-qt-module.md)
  - [Creating the QML GUI](./getting-started/3-qml-gui.md)
  - [Building with Cargo](./getting-started/4-cargo-executable.md)
  - [Building with CMake](./getting-started/5-cmake-integration.md)
- [Core Concepts](./concepts/index.md)
  - [Build Systems](./concepts/build_systems.md)
    - [Building for WebAssembly](./concepts/wasm-builds.md)
  - [Generated `QObject`](./concepts/generated_qobject.md)
  - [Types](./concepts/types.md)
  - [Nested Objects](./concepts/nested_objects.md)
  - [Inheritance & Overriding](./concepts/inheritance.md)
- [Reference: the bridge module](./bridge/index.md)
  - [`extern "RustQt"`](./bridge/extern_rustqt.md)
  - [`extern "C++Qt"`](./bridge/extern_cppqt.md)
  - [Shared types](./bridge/shared_types.md)
  - [Attributes](./bridge/attributes.md)
  - [Traits](./bridge/traits.md)
- [Common Issues](./common-issues.md)
- [For Contributors: CXX-Qt Internals](./internals/index.md)
  - [Build System](./internals/build-system.md)
  - [Crate Organization](./internals/crate-organization.md)
