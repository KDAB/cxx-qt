<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Threading

## Concept

The general concept for threading is that when Rust code is being executed a lock has been acquired on the C++ side to prevent Rust code being executed from multiple threads.

This means that Rust code, such as invokables and properties, which are directly called from C++ are executed on the Qt thread.

Note that a recursive mutex is used internally, this allows for signals to be emitted and then call slots on the same object without deadlocks.

## Multi threading

To achieve safe multi-threading on the Rust side we use an [`CxxQtThread<T>`](../qobject/cxxqtthread.md).
A `CxxQtThread<T>` represents a reference to the Qt thread that the QObject of type `T` lives in.
When a new Rust thread is started (e.g. in an invokable) the `CxxQtThread<T>` can be moved into the thread to later update the QObject in a thread safe manner.

When the Rust thread needs to update a value in the QObject it can then queue a closure to the thread.
This closure will be executed on the thread the QObject lives in while holding a lock on the Rust object.
Updating the QObject is then thread-safe.

Below is a complete Rust example of a multi-threaded object.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/threading.rs:book_macro_code}}
```
