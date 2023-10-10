<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Types

CXX-Qt supports most types supported by CXX. These can be used in properties, invokables, and signals.

## `cxx-qt-lib` Types

The `cxx-qt-lib` crate provides CXX bindings for common Qt types.

Use the [`cxx-qt-lib` Docs](https://docs.rs/cxx-qt-lib/latest/cxx_qt_lib/) to explore the available types.

### Container Types

The `cxx-qt-lib` crate has containers types, such as `QSet<T>`.

To use these define a templated type in the CXX bridge, but note that the type
name must be `QSet_T` as this needs to match the name in C++ code.

So for `QSet<i32>` the type name should be `QSet_i32`.

```rust,ignore
{{#include ../../../tests/qt_types_standalone/rust/src/qset.rs:book_qset}}
```

To use key-value based container types such as `QHash<K, V>` an intermediate type on the Rust side
is defined to implement a trait on the key-value combination.

As with other container types, the type name must be `QHash_K_V` as this needs
to match the name in the C++ code.

So for `QHash<QString, QVariant>`, define an intermediate type called `QHashPair_QString_QVariant`.
Then the type name `QHash_QString_QVariant` is used to match the C++ side.

```rust,ignore
{{#include ../../../tests/qt_types_standalone/rust/src/qhash.rs:book_qhash}}
```

> A type alias such as `QVariantMap` can be used by using the matching type in Rust such as `QMap<QString, QVariant>`.

## Defining a Custom Type

Any types that are valid CXX types should be usable with CXX-Qt as well.

For examples of how to wrap Qt objects, explore the [`cxx-qt-lib` source code](https://github.com/KDAB/cxx-qt/tree/main/crates/cxx-qt-lib).

> The same rules apply as CXX, so a type must be [trivial](https://cxx.rs/extern-c++.html?highlight=trivial#integrating-with-bindgen-generated-or-handwritten-unsafe-bindings) to pass by value.
> If they are opaque, references or pointers must be used.

### Using a Custom Type with Containers or QVariant

To use a custom type with containers find the trait that the container uses, eg for `QSet<T>` there is a `QSetElement` trait and for `QHash<K, V>` there is a `QHashPair` trait.

Implement the trait for your custom type and then you can use the containers as described above.

To use a custom type with `QVariant` implement the `QVariantValue` trait for your custom type, as seen below, then it can be used as normal.

```rust,ignore
{{#include ../../../examples/qml_features/rust/src/types.rs:book_qvariantvalue_impl}}
```

A full example of implementing a custom struct with `QVariant` is shown in the [qml_features types example](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/types.rs).

> Any custom types or alias in C++ should be registered with Qt using `qRegisterMetaType<T>("TYPE")` to ensure that they work with QML.
