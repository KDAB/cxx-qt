<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# The bridge module reference

- [`extern "RustQt"`](./extern_rustqt.md) - exposing Rust types to Qt as `QObject`, `Q_SIGNAL`, `Q_PROPERTY` etc
- [`extern "C++Qt"`](./extern_cppqt.md) - binding Qt features and types to Rust, such as `QObject`, `Q_SIGNAL` etc
- [Shared types](./shared_types.md) - shared enums between Rust and Qt, such as `Q_ENUM`, `Q_ENUM_NS` etc
- [Attributes](./attributes.md) - working with namespaces, giving functions different names
- [Traits](./traits.md) - traits related to a CXX-Qt `QObject`

The `#[cxx_qt::bridge]` macro functions very similarly to [`#[cxx::bridge]`](https://docs.rs/cxx/latest/cxx/attr.bridge.html). This macro needs to be written above a Rust module definition.

This Rust module will then function like a normal CXX bridge, whilst also supporting the additional features added by CXX-Qt. Refer to [the CXX documentation](https://cxx.rs/) for details on how to describe the language boundary.

> Don't forget to add the Rust source file to the `CxxQtBuilder` in your `build.rs` script. For instructions, see the [Getting Started guide](../getting-started/5-cmake-integration.md).

The `#[cxx_qt::bridge]` macro supports the options in its attribute:

- [`namespace`](./attributes.md#namespace)
