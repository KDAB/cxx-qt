<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# RustObj Struct

The RustObj struct allows you to define the following items

  * Invokable methods that are exposed to Qt
  * Private methods and fields for RustObj to use (eg this is useful for storing the channels for [threading](../concepts/threading.md))
  * Mutate C++ state with [`CppObj`](./cpp_object.md)
  * Implement [handlers](./handlers.md) for property or update requests

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/src/rust_obj_invokables.rs:book_macro_code}}
```

## Invokables

To define a method which is exposed to QML and C++, add a method on the `RustObj` struct and add the attribute `#[invokable]`. The parameters and return type are then matched to the Qt side.

Note to access properties on the C++ object use [Cpp Object](./cpp_object.md).

## Private Methods and Fields

Unlike the [Data Struct](./data_struct.md) fields which are defined on the `RustObj` struct are not exposed as properties to Qt. These can be considered as "private to Rust" fields, and are useful for storing channels for threading or internal information for the QObject.

Methods implemented on the `RustObj` that do not have an `#[invokable]` attribute are not exposed to C++ and are considered "private to Rust" methods. Similar to fields these are useful for threading and internal information.
