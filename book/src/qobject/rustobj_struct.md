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

A `impl cxx_qt::QObject<RustObj>` is used to define invokables, the `impl cxx_qt::QObject<RustObj>` defines that the methods are implemented onto the C++ QObject.
Therefore they have access to both C++ and Rust methods. Also CXX-Qt adds wrapper code around your invokables to automatically perform any conversion between the [C++ and Rust types](../concepts/types.md).

To mark a method as invokable simply add the `#[invokable]` attribute to the Rust method. This then causes `Q_INVOKABLE` to be set on the C++ definition of the method, allowing QML to call the invokable.

Note to access properties on the C++ object use [Cpp Object](./cpp_object.md).

## Private Methods and Fields

Unlike the [Data Struct](./data_struct.md) fields which are defined on the `RustObj` struct are not exposed as properties to Qt. These can be considered as "private to Rust" fields, and are useful for storing channels for threading or internal information for the QObject.

Methods implemented using `impl RustObj` (and not `impl cxx_qt::QObject<RustObj>`) are just normal Rust member methods.
Therefore they do not have access to any C++ or QObject functionality (e.g. emitting Signals, changing properties, etc.)
You will usually only need to use `impl RustObj` if you want to also use your RustObj struct as a normal Rust struct, that is not wrapped in a QObject.
