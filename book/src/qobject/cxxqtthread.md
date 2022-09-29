<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CxxQtThread

`CxxQtThread<T>` is used for [threading](../concepts/threading.md) with QObject#s and allows you to queue events from a background thread to occur on the Qt event loop.

To access the `CxxQtThread<T>` use the `qt_thread(&self)` method on the [`CppObj`](./cpp_object.md) or on the C++ pointer of the QObject.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/threading.rs:book_qt_thread}}
```

The `CxxQtThread<T>` can then be moved into the Rust thread, a `queue(fn(ctx: Pin<&mut TQt>) -> bool` is used to queue a Rust function pointer onto the Qt event loop. The first argument in the function pointer is a pinned pointer to the C++ side of the QObject.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/threading.rs:book_qt_thread_queue}}
```
