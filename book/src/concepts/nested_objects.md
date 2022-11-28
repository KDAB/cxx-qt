<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Nested Objects

Rust Qt objects can be nested as properties or parameters of each other.

A nested object is referred to by using a pointer to its QObject representation.

First define a type with an extern block for your bridge, this should point to the `qobject::T` of the QObject and the `cxx_name` should match the QObject name.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/nested_qobjects.rs:book_extern_block}}
```

This can then be used as a property, invokable parameter, or signal parameter by using `*mut T`. As seen in the example below which nests `InnerObject` into `OuterObject`.

> Note that to reach mutable invokables and property setters of the nested object
> `*mut T` needs to be convered to `Pin<&mut T>`.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/nested_qobjects.rs:book_macro_code}}
```
