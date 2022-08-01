<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Signals enum

The `cxx_qt::signals(T)` attribute is used on an enum to define which signals should exist on the QObject `T`. It allows you to define the signal name and the parameters of the signal.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/src/signals.rs:book_signals_enum}}
```

## Emitting a signal

To emit a signal from Rust use the [`CppObj`](./cpp_object.md) and call either the `emit_queued(Signal)` or `unsafe emit_immediate(Signal)` method.

Note that `emit_immediate` is unsafe as it can cause deadlocks if the `Q_EMIT` is `Qt::DirectConnection` connected to a Rust invokable on the same QObject that has caused the `Q_EMIT`, as this would then try to lock the `RustObj` which is already locked.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/src/signals.rs:book_rust_obj_impl}}
```
