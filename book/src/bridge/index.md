<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# The bridge module reference

- [extern "RustQt"](./extern_rustqt.md) - exposing Rust types to Qt as QObject, Q_SIGNAL, Q_PROPERTY etc
- [extern "C++Qt"](./extern_cppqt.md) - binding Qt features and types to Rust, such as QObject, Q_SIGNAL etc
- [Shared types](./shared_types.md) - shared enums between Rust and Qt, such as Q_ENUM, Q_ENUM_NS etc
- [Attributes](./attributes.md) - working with namespaces, giving functions different names
- [Traits](./traits.md) - traits related to a CXX-Qt QObject

The `#[cxx_qt::bridge]` macro functions very similarly to [`#[cxx::bridge]`](https://docs.rs/cxx/latest/cxx/attr.bridge.html). This macro needs to be written above a Rust module definition.

This Rust module will then function like a normal CXX bridge, whilst also supporting the additional features added by CXX-Qt. Refer to the [the CXX documentation](https://cxx.rs/) for details on how to describe the language boundary.

> Don't forget to add the Rust source file to the CxxQtBuilder in your build.rs script. For instructions, see the [Getting Started guide](../getting-started/5-cmake-integration.md).

The `#[cxx_qt::bridge]` macro supports two options in it's attribute
- [`cxx_file_stem`](#cxx_file_stem)
- [`namespace`](./attributes.md#namespace)

## cxx_file_stem

By default, the name of the generated C++ header file will be the name of the module, followed by `.cxxqt.h` (and `.cxx.h` for CXX files).

This can cause issues as the module is normally called `ffi` or `qobject` so collisions would occur.

The `cxx_file_stem` option allow a file name to be specified to avoid collisions.

```rust,ignore
{{#include ../../../examples/qml_features/rust/src/types.rs:book_cxx_file_stem}}
```

> Currently, cxx-qt-gen writes all generated header files into a single folder.
> Therefore you need to be careful to not produce two header files with the same filename.

> We want to use the name of the Rust source file that the macro is located in (the same as CXX).
> However this requires [inspection APIs from `proc_macro::Span`](https://github.com/rust-lang/rust/issues/54725)
> which is currently a nightly feature.
